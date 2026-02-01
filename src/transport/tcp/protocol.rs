use crate::core::types::CheckInput;

pub const CHECK: &str = "CHECK";
pub const PING: &str = "PING";

pub enum CommandResponse {
    Check(CheckInput),
    Pong,
}

pub fn parse_command(line: &str) -> Result<CommandResponse, String> {
    let parts: Vec<&str> = line.trim().split_whitespace().collect();

    if parts.is_empty() {
        return Err("empty command".into());
    }

    match parts[0].to_uppercase().as_str() {
        PING => {
            if parts.len() != 1 {
                return Err(format!("invalid command: {}", line));
            }
            Ok(CommandResponse::Pong)
        }
        CHECK => {
            if parts.len() != 4 {
                return Err(format!("invalid command: {}", line));
            }

            let key = parts[1].to_string();
            let limit = parts[2].parse::<u32>().map_err(|_| "invalid limit")?;
            let window_ms = parts[3].parse::<u64>().map_err(|_| "invalid window size")?;

            Ok(CommandResponse::Check(CheckInput {
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
