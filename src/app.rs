use crate::core::limiter::LimiterImpl;

#[derive(Clone)]
pub struct App {
    pub limiter: LimiterImpl,
}
