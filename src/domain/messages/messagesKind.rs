use std::fmt;

pub enum MessageKind {
    Error,
    Connect,
    Connected,
    Ping,
    Pong,
    Sync,
    Synced,
    Debug,
}

impl fmt::Display for MessageKind {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            MessageKind::Error => write!(f, "error"),
            MessageKind::Connect => write!(f, "connect"),
            MessageKind::Connected => write!(f, "connected"),
            MessageKind::Ping => write!(f, "ping"),
            MessageKind::Pong => write!(f, "pong"),
            MessageKind::Sync => write!(f, "sync"),
            MessageKind::Synced => write!(f, "synced"),
            MessageKind::Debug => write!(f, "debug"),
        }
    }
}

