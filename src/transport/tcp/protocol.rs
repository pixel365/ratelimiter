use crate::core::constants::{CHECK, PING};
use crate::core::types::CheckInput;

pub enum Command {
    Check(CheckInput),
    Pong,
}

pub fn parse_command(line: &str) -> Result<Command, String> {
    let parts: Vec<&str> = line.trim().split_whitespace().collect();

    if parts.is_empty() {
        return Err("empty command".into());
    }

    match parts[0].to_uppercase().as_str() {
        PING => {
            if parts.len() != 1 {
                return Err(format!("invalid command: {}", line));
            }
            Ok(Command::Pong)
        }
        CHECK => {
            if parts.len() != 4 {
                return Err(format!("invalid command: {}", line));
            }

            let key = parts[1].to_string();
            let limit = parts[2].parse::<usize>().map_err(|_| "invalid limit")?;
            let window_ms = parts[3].parse::<u64>().map_err(|_| "invalid window")?;

            Ok(Command::Check(CheckInput {
                key,
                limit,
                window_ms,
            }))
        }
        _ => Err(format!(
            "invalid command: {}",
            parts[0].to_uppercase().as_str()
        )),
    }
}
