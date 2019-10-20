use crate::domain::messages::error::WrongFormatErrorMessage;
use serde_json::Value;

#[derive(serde::Deserialize)]
pub struct OptionnalConnectMessage {
    /// Subprotocol version is a string in SemVer. It describes an application
    /// subprotocol, which developper will create on top of Logux protocol.If other node doesn't
    /// support this suboprotocol, it could send wrong-subprotocol error.
    pub subprotocol: Option<String>,
    /// Credentials are string, receiver may check credentials data. On wrong
    /// credentials, receiver may send wrong-credentials error and close connection.
    pub credentials: Option<String>,
}

pub struct ConnectMessage {
    /// Protocol Version.
    pub protocol: u64,
    /// Node id, should be unique across the network.
    pub node_id: String,
    /// Sync number, last added time used by receiver in previous connection,
    /// 0 on first connection.
    pub synced: u64,
    /// Optionals props for connected message
    pub options: Option<OptionnalConnectMessage>,
}

/// Function to decode a vec to a ConnectMessage
pub fn decode_connect_message(vec: &[Value]) -> Result<ConnectMessage, WrongFormatErrorMessage> {
    match &vec[..] {
        [_, Value::Number(protocol), Value::String(node_id), Value::Number(synced)] => {
            match [protocol.as_u64(), synced.as_u64()] {
                [Some(protocol), Some(synced)] => Ok(ConnectMessage {
                    protocol,
                    node_id: node_id.to_string(),
                    synced,
                    options: None,
                }),
                _ => Err(WrongFormatErrorMessage {
                    message: "Invalid connect type, please refer to the documentation.".to_string(),
                }),
            }
        }
        [_, Value::Number(protocol), Value::String(node_id), Value::Number(synced), optionnal_field] =>
        {
            let options: serde_json::Result<OptionnalConnectMessage> =
                serde::Deserialize::deserialize(optionnal_field);

            // Right now, if we put something like
            // ["connect", 124, "nodeid", 12, { "de": 1 }]
            // it won't fail, it'll be a success with optionnals to None.
            match (protocol.as_u64(), synced.as_u64(), options.ok()) {
                (Some(protocol), Some(synced), Some(good_option)) => Ok(ConnectMessage {
                    protocol,
                    node_id: node_id.to_string(),
                    synced,
                    options: Some(good_option),
                }),
                _ => Err(WrongFormatErrorMessage {
                    message: "Invalid connected type, please refer to the documentation."
                        .to_string(),
                }),
            }
        }
        _ => Err(WrongFormatErrorMessage {
            message: "Invalid connected type, please refer to the documentation.".to_string(),
        }),
    }
}
