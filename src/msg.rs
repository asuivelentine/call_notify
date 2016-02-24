#[derive(Debug, Clone)]
pub struct Message {
    pub raw_data: String,
    pub version: u16,
}

impl Message {
    fn new(raw: String) -> Message {
        Message {
            version: 1,
            raw_data: raw,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::Message;

    #[test]
    fn msg_new_test() {
        let msg = Message::new("hello world".to_string());
        assert_eq!("hello world".to_string(), msg.raw_data);
        assert_eq!(1, msg.version);
    }
}

