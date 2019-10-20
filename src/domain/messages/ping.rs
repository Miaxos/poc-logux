use crate::domain::messages::error::WrongFormatErrorMessage;
use serde_json::Value;

pub struct PingMessage {
    /// Sync number, last added time used by receiver in previous connection,
    /// 0 on first connection.
    pub synced: u64,
}

/// Function to decode a vec to PingMessage
pub fn decode_ping_message(vec: &[Value]) -> Result<PingMessage, WrongFormatErrorMessage> {
    match &vec[..] {
        [_, Value::Number(synced)] => match synced.as_u64() {
            Some(synced) => Ok(PingMessage { synced }),
            _ => Err(WrongFormatErrorMessage {
                message: "Invalid ping type, please refer to the documentation.".to_string(),
            }),
        },
        _ => Err(WrongFormatErrorMessage {
            message: "Invalid ping type, please refer to the documentation.".to_string(),
        }),
    }
}
