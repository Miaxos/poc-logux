use crate::domain::messages::error::WrongFormatErrorMessage;
use crate::domain::messages::lib::LoguxEvent;
use serde_json::Value;

pub struct PongMessage {
    /// Sync number, last added time used by receiver in previous connection,
    /// 0 on first connection.
    pub synced: u64,
}

impl LoguxEvent for PongMessage {
    fn encode(&self) -> String {
        // Horrible hack becayse logux is fucked up
        format!("[ \"pong\", {} ]", &self.synced)
    }
}

/// Function to decode a vec to PongMessage
pub fn decode_pong_message(vec: &[Value]) -> Result<PongMessage, WrongFormatErrorMessage> {
    match &vec[..] {
        [_, Value::Number(synced)] => match synced.as_u64() {
            Some(synced) => Ok(PongMessage { synced }),
            _ => Err(WrongFormatErrorMessage {
                message: "Invalid pong type, please refer to the documentation.".to_string(),
            }),
        },
        _ => Err(WrongFormatErrorMessage {
            message: "Invalid pong type, please refer to the documentation.".to_string(),
        }),
    }
}
