use std::io::{Error, ErrorKind};

pub fn invalid_cfg(msg: &'static str) -> Error {
    Error::new(ErrorKind::InvalidInput, msg)
}
