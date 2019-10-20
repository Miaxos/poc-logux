use super::messagesKind::MessageKind;
use std::fmt;

enum ErrorMessageKind {
    /// Client Logux protocol version is not supported by server.
    WrongProtocol,
    /// Message is not correct JSON, is not an array or have no kind.
    WrongFormat,
    /// Message's type is not supported.
    UnkownMessage,
    /// Sent credentials doesn't pass authentication.
    WrongCredentials,
    /// Not `connect`, `connected` or `error` messages were sent before authentication.
    MissedAuth,
    /// A timeout was reached.
    Timeout,
    /// Client application subprotocol version is not supported by server.
    WrongSubprotocol,
}

impl fmt::Display for ErrorMessageKind {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            ErrorMessageKind::WrongCredentials => write!(f, "wrong-credentials"),
            ErrorMessageKind::WrongFormat => write!(f, "wrong-format"),
            ErrorMessageKind::WrongProtocol => write!(f, "wrong-protocol"),
            ErrorMessageKind::WrongSubprotocol => write!(f, "wrong-subprotocol"),
            ErrorMessageKind::UnkownMessage => write!(f, "unknown-message"),
            ErrorMessageKind::MissedAuth => write!(f, "missed-auth"),
            ErrorMessageKind::Timeout => write!(f, "timeout"),
        }
    }
}

pub struct WrongProtocolErrorMessage {
    /// Key with minimum supported version.
    supported: String,
    /// Key with the used version.
    used: String,
}

impl fmt::Display for WrongProtocolErrorMessage {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "[\"{}\", \"{}\", \"{}\", \"{}\"]",
            &MessageKind::Error,
            &ErrorMessageKind::WrongProtocol,
            &self.supported,
            &self.used,
        )
    }
}

pub struct WrongFormatErrorMessage {
    /// Bad message string.
    pub message: String,
}

impl fmt::Display for WrongFormatErrorMessage {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "[\"{}\", \"{}\", \"{}\"]",
            &MessageKind::Error,
            &ErrorMessageKind::WrongFormat,
            &self.message
        )
    }
}

/*
impl Serialize for WrongFormatErrorMessage {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut state = serializer.serialize_struct("WrongProtocolErrorMessage", 1)?;
        state.serialize_field("type", &MessageKind::Error)?;
        state.serialize_field("errorType", &ErrorMessageKind::WrongSubprotocol)?;
        state.serialize_field("message", &self.message)?;
        state.end()
    }
}
*/

pub struct UnkownMessageErrorMessage {
    /// Bad message type string.
    pub message_type: String,
}

impl fmt::Display for UnkownMessageErrorMessage {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "[\"{}\", \"{}\", \"{}\"]",
            &MessageKind::Error,
            &ErrorMessageKind::UnkownMessage,
            &self.message_type,
        )
    }
}

struct MissedAuthErrorMessage {
    r#type: MessageKind,
    /// Bad message string.
    message: String,
}

struct TimeoutErrorMessage {
    r#type: MessageKind,
    timeout_duration: String,
}

struct WrongSubProtocolErrorMessage {
    r#type: MessageKind,
    /// Key with minimum supported version.
    supported: String,
    /// Key with the used version.
    used: String,
}
