use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Deserialize)]
pub struct CheckInput {
    pub key: String,
    pub limit: usize,
    pub window_ms: u64,
}

#[derive(Debug, Clone, Copy, Serialize)]
pub struct CheckOutput {
    pub allowed: bool,
    pub remaining: usize,
    pub reset_ms: u64,
}

#[derive(Debug, Clone, Copy)]
pub enum CheckError {
    EmptyKey,
    KeyTooLong { max: usize },
    LimitZero,
    WindowZero,
}

impl CheckError {
    pub fn as_str(&self) -> String {
        match self {
            CheckError::EmptyKey => "key must be not empty".into(),
            CheckError::KeyTooLong { max } => format!("key too long (max {max})"),
            CheckError::LimitZero => "limit must be greater than zero".into(),
            CheckError::WindowZero => "window must be greater than zero".into(),
        }
    }
}

impl CheckInput {
    pub fn validate(&self, max_key_len: usize) -> Result<(), CheckError> {
        if self.key.trim().is_empty() {
            return Err(CheckError::EmptyKey);
        }

        if self.key.len() > max_key_len {
            return Err(CheckError::KeyTooLong { max: max_key_len });
        }

        if self.limit == 0 {
            return Err(CheckError::LimitZero);
        }

        if self.window_ms == 0 {
            return Err(CheckError::WindowZero);
        }

        Ok(())
    }
}
