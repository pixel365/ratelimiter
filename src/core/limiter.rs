use crate::core::fixed_window::FixedWindowLimiter;
use crate::core::types::{CheckInput, CheckOutput};
use std::time::Duration;

pub trait Limiter: Send + Sync + 'static {
    fn check(&self, input: CheckInput) -> CheckOutput;
}

#[derive(Clone)]
pub enum LimiterImpl {
    FixedWindow(FixedWindowLimiter),
}

impl LimiterImpl {
    pub fn cleanup_task(&self, interval: Duration) {
        match self {
            LimiterImpl::FixedWindow(limiter) => limiter.cleanup(interval),
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
