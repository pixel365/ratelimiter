use crate::core::limiter::Limiter;
use crate::core::types::{CheckInput, CheckOutput};
use dashmap::DashMap;
use std::sync::Arc;
use std::time::{Duration, Instant};
use tokio::time::sleep;

#[derive(Clone)]
pub struct FixedWindowLimiter {
    inner: Arc<Inner>,
}

#[derive(Debug, Clone)]
struct Entry {
    count: usize,
    window_end: Instant,
}

struct Inner {
    map: DashMap<String, Entry>,
}

impl FixedWindowLimiter {
    pub fn new() -> Self {
        Self {
            inner: Arc::new(Inner {
                map: DashMap::new(),
            }),
        }
    }

    pub fn cleanup(&self, interval: Duration) {
        let inner = self.inner.clone();

        tokio::spawn(async move {
            loop {
                sleep(interval).await;
                let now = Instant::now();
                let mut removed = 0;

                inner.map.retain(|_, entry| {
                    let keep = entry.window_end > now;
                    if !keep {
                        removed += 1;
                    }
                    keep
                });

                if removed > 0 {
                    tracing::debug!("Removed {} entries", removed);
                }
            }
        });
    }
}

impl Limiter for FixedWindowLimiter {
    fn check(&self, input: CheckInput) -> CheckOutput {
        debug_assert!(input.limit > 0);
        debug_assert!(input.window_ms > 0);

        let now = Instant::now();
        let window = Duration::from_millis(input.window_ms);

        let mut entry = self
            .inner
            .map
            .entry(input.key.to_owned())
            .or_insert_with(|| Entry {
                count: 0,
                window_end: now + window,
            });

        if now >= entry.window_end {
            entry.count = 0;
            entry.window_end = now + window;
        }

        let allowed = entry.count < input.limit;
        if allowed {
            entry.count += 1;
        }

        let remaining = if allowed {
            input.limit.saturating_sub(entry.count)
        } else {
            0
        };

        let reset_ms = entry
            .window_end
            .saturating_duration_since(now)
            .as_millis()
            .min(u128::from(u64::MAX)) as u64;

        CheckOutput {
            allowed,
            remaining,
            reset_ms,
        }
    }
}
