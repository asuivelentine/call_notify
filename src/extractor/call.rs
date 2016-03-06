//call module -> handles incomming call informations
use msg_handler::NotificationListener;
use msg::Message;

struct Call;

impl NotificationListener for Call {
    fn message_received(&self, msg: Message) -> Message {
        
        msg
    }

    fn connection_closed(&self){

    }
}

impl Call {
    pub fn new() -> Call {
        Call
    }   
}
