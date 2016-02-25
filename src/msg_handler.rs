use msg::Message;
use std::sync::mpsc::{ Sender, Receiver };
use std::sync::mpsc::channel;

trait NotificationListener {
    fn message_received(&self, msg: Message) -> Message;
    fn connectionClosed(&self);
}

struct MessageHandler<'a> {
    listeners: Vec<&'a NotificationListener>,
}

impl<'a> MessageHandler<'a> {
    pub fn connect(port: u16) {
        let (tx, rx): (Sender<String>, Receiver<String>) = channel();

    }
    pub fn register(&self, item: &NotificationListener) {

    }
}

#[cfg(test)]
mod test {

}
