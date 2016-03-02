// This module represents the message itself
use rustc_serialize::json::Json;

#[derive(Debug, PartialEq, Clone)]
pub enum MessageKind {
    ConnectionClosed,
    ModuleMessage,
    Empty,
}

#[derive(Debug, Clone)]
pub struct Message {
    pub kind: MessageKind,
    pub raw_data: String,
    pub version: u16,
    pub data: Option<Json>,
}

impl Message {
    pub fn new(raw: String) -> Message {
        let mut msg_type = MessageKind::ModuleMessage;
        if raw == "Connection closed".to_string() {
            msg_type = MessageKind::ConnectionClosed;
        }

        Message {
            kind: msg_type,
            version: 1,
            raw_data: raw,
            data: None,
        }
    }

    fn set_version(mut self) -> Message {
        self
    }

    pub fn set_data(mut self, data: Option<Json>) -> Message {
        self.data = data;
        self
    }
}

#[cfg(test)]
mod tests {
    use super::Message;
    use super::MessageKind;

    #[test]
    fn msg_new_test() {
        let msg = Message::new("hello world".to_string());
        assert_eq!("hello world".to_string(), msg.raw_data);
        assert_eq!(1, msg.version);
        assert_eq!(MessageKind::ModuleMessage, msg.kind);
    }

    #[test]
    fn msg_new_close_test() {
        let msg = Message::new("Connection closed".to_string());
        assert_eq!("Connection closed".to_string(), msg.raw_data);
        assert_eq!(1, msg.version);
        assert_eq!(MessageKind::ConnectionClosed, msg.kind);
    }
}
