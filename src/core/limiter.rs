use crate::core::fixed_window::FixedWindowLimiter;
use crate::core::types::{CheckInput, CheckOutput};
use std::time::Duration;
use tokio_util::sync::CancellationToken;

pub trait Limiter: Send + Sync + 'static {
    fn check(&self, input: CheckInput) -> CheckOutput;
}

#[derive(Clone)]
pub enum LimiterImpl {
    FixedWindow(FixedWindowLimiter),
}

impl LimiterImpl {
    pub fn cleanup_task(&self, interval: Duration, stop: CancellationToken) {
        match self {
            LimiterImpl::FixedWindow(limiter) => limiter.cleanup(interval, stop),
        }
    }
}

impl Limiter for LimiterImpl {
    fn check(&self, input: CheckInput) -> CheckOutput {
        match self {
            LimiterImpl::FixedWindow(limiter) => limiter.check(input),
        }
    }
}
