use crate::domain::messages::error::WrongFormatErrorMessage;
use serde_json::Value;

pub struct SyncedMessage {
    /// Sync number, last added time used by receiver in previous connection,
    /// 0 on first connection.
    synced: u64,
}

/// Function to decode a vec to PingMessage
pub fn decode_synced_message(vec: &[Value]) -> Result<SyncedMessage, WrongFormatErrorMessage> {
    match &vec[..] {
        [_, Value::Number(synced)] => match synced.as_u64() {
            Some(synced) => Ok(SyncedMessage { synced }),
            _ => Err(WrongFormatErrorMessage {
                message: "Invalid synced type, please refer to the documentation.".to_string(),
            }),
        },
        _ => Err(WrongFormatErrorMessage {
            message: "Invalid synce type, please refer to the documentation.".to_string(),
        }),
    }
}
