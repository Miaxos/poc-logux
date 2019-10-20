use crate::domain::messages::error::WrongFormatErrorMessage;
use crate::domain::messages::lib::LoguxEvent;
use serde::{Deserialize, Serialize};
use serde_json::Value;

#[derive(Deserialize, Serialize)]
pub struct OptionnalConnectedMessage {
    /// Subprotocol version is a string in SemVer. It describes an application
    /// subprotocol, which developper will create on top of Logux protocol.If other node doesn't
    /// support this suboprotocol, it could send wrong-subprotocol error.
    pub subprotocol: Option<String>,
    /// Credentials are string, receiver may check credentials data. On wrong
    /// credentials, receiver may send wrong-credentials error and close connection.
    pub credentials: Option<String>,
}

impl LoguxEvent for OptionnalConnectedMessage {
    fn encode(&self) -> String {
        // TODO: Fix this hsit
        // serde_json::to_string(&self).unwrap()
        r#"{ "credentials": { "env": "development" }, "subprotocol": "1.0.0" }"#.to_string()
    }
}

#[derive(Deserialize, Serialize)]
pub struct ConnectedMessage {
    /// Protocol Version.
    pub protocol: u64,
    /// Node id, should be unique across the network.
    pub node_id: String,
    /// Contains connect receiving time and connected sending time.
    /// Time should be a milliseconds elapsed since 1 January 1970 00:00:00 UTC
    pub time_sync: [u64; 2],
    /// Optionals props for connected message
    pub options: Option<OptionnalConnectedMessage>,
}

impl LoguxEvent for ConnectedMessage {
    fn encode(&self) -> String {
        // Horrible hack becayse logux is fucked up
        format!(
            "[ \"connected\", {}, \"{}\", {}, {} ]",
            &self.protocol,
            &self.node_id,
            serde_json::to_string(&self.time_sync).unwrap(),
            match &self.options {
                Some(opt) => opt.encode(),
                None => "null".to_string(),
            },
        )
    }
}

/// Function to decode a vec to a ConnectMessage
pub fn decode_connected_message(
    vec: &[Value],
) -> Result<ConnectedMessage, WrongFormatErrorMessage> {
    match &vec[..] {
        [_, Value::Number(protocol), Value::String(node_id), Value::Array(time_sync)] => {
            match (protocol.as_u64(), &time_sync[..]) {
                (Some(protocol), [Value::Number(start), Value::Number(end)]) => {
                    match (start.as_u64(), end.as_u64()) {
                        (Some(start), Some(end)) => Ok(ConnectedMessage {
                            protocol,
                            node_id: node_id.to_string(),
                            time_sync: [start, end],
                            options: None,
                        }),
                        _ => Err(WrongFormatErrorMessage {
                            message: "Invalid sync time format.".to_string(),
                        }),
                    }
                }
                _ => Err(WrongFormatErrorMessage {
                    message: "Invalid connected type, please refer to the documentation."
                        .to_string(),
                }),
            }
        }
        [_, Value::Number(protocol), Value::String(node_id), Value::Array(time_sync), optionnal_field] =>
        {
            let options: serde_json::Result<OptionnalConnectedMessage> =
                serde::Deserialize::deserialize(optionnal_field);

            // Right now, if we put something like
            // ["connect", 124, "nodeid", 12, { "de": 1 }]
            // it won't fail, it'll be a success with optionnals to None.
            match (protocol.as_u64(), &time_sync[..]) {
                (Some(protocol), [Value::Number(start), Value::Number(end)]) => {
                    match (start.as_u64(), end.as_u64(), options.ok()) {
                        (Some(start), Some(end), Some(good_option)) => Ok(ConnectedMessage {
                            protocol,
                            node_id: node_id.to_string(),
                            time_sync: [start, end],
                            options: Some(good_option),
                        }),
                        _ => Err(WrongFormatErrorMessage {
                            message: "Invalid sync time format.".to_string(),
                        }),
                    }
                }
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
