use crate::domain::messages::error::WrongFormatErrorMessage;
use serde_json::Value;

pub struct SyncMessage {
    /// Sync number, last added time used by receiver in previous connection,
    /// 0 on first connection.
    pub synced: u64,
    pub actions: std::vec::Vec<Value>,
}

/// Function to decode a vec to PingMessage
pub fn decode_sync_message(vec: &[Value]) -> Result<SyncMessage, WrongFormatErrorMessage> {
    if vec.len() < 2 {
        Err(WrongFormatErrorMessage {
            message: "Invalid sync type, please refer to the documentation.".to_string(),
        })
    } else {
        let (_, actions) = vec.split_at(2);
        let sync_options = vec.get(1);

        match sync_options {
            Some(sync) => match sync.as_u64() {
                Some(synced) => Ok(SyncMessage {
                    synced,
                    actions: actions.to_vec(),
                }),
                _ => Err(WrongFormatErrorMessage {
                    message: "Invalid synced number, please refer to the documentation."
                        .to_string(),
                }),
            },
            _ => Err(WrongFormatErrorMessage {
                message: "Invalid sync type, please refer to the documentation.".to_string(),
            }),
        }
    }
}
