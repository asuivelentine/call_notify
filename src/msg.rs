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
    pub version: u64,
    pub data: Option<Json>,
}

impl Message {
    pub fn new(raw: String) -> Message {
        let mut msg_type = MessageKind::ModuleMessage;
        let mut version = 0;
        if raw == "Connection closed".to_string() {
            msg_type = MessageKind::ConnectionClosed;
        }
        
        let json = Message::create_json(&raw);
        if json.is_some() {
            let json = json.clone().unwrap();
            version = match Message::get_version(&json.clone()) {
                Some(n) => n,
                None => 0,
            }
        }

        Message {
            kind: msg_type,
            version: version,
            raw_data: raw,
            data: json,
        }
    }

    fn get_version(data: &Json) -> Option<u64> {
        let root = data.as_object();
        if root.is_none() {
            return None;
        }

        let version_json = root.unwrap().get("version");
        if version_json.is_none() {
            return None;
        }

        version_json.unwrap().as_u64()
    }

    fn create_json(data: &str) -> Option<Json>{
        let json_data = Json::from_str(data);
        match json_data {
            Ok(n) => Some(n),
            Err(_) => None,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::Message;
    use super::MessageKind;

    #[test]
    fn msg_new_test() {
        let msg = Message::new("{\"version\":6,\"data\":\"hellow\",\"res\":{\"id\":42,\"is_good\":false}}".to_string());
        assert_eq!("{\"version\":6,\"data\":\"hellow\",\"res\":{\"id\":42,\"is_good\":false}}".to_string(), msg.raw_data);
        assert_eq!(6, msg.version);
        assert_eq!(MessageKind::ModuleMessage, msg.kind);
    }

    #[test]
    fn msg_new_close_test() {
        let msg = Message::new("Connection closed".to_string());
        assert_eq!("Connection closed".to_string(), msg.raw_data);
        assert_eq!(MessageKind::ConnectionClosed, msg.kind);
    }

    #[test]
    fn msg_get_version() {
        let msg = String::from("{\"version\":6,\"data\":\"hellow\",\"res\":{\"id\":42,\"is_good\":false}}");
        let json = Message::create_json(&msg);
        assert!(json.is_some());
        let json = json.unwrap();
        let version = Message::get_version(&json);
        assert!(version.is_some());
        let version = version.unwrap();
        assert_eq!(6, version);

    }
}
