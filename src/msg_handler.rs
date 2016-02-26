use msg::Message;
use con::Connection;
use std::thread;
use std::time::Duration;
use std::sync::mpsc::{ Sender, Receiver };
use std::sync::mpsc::channel;

pub trait NotificationListener {
    fn message_received(&self, msg: Message) -> Message;
    fn connectionClosed(&self);
}

impl <'a> NotificationListener for &'a MessageHandler<'a>{
    fn message_received(&self, msg: Message) -> Message {
        println!("{:?}", msg);
        msg
    }

    fn connectionClosed(&self) {
        println!("connection closed");
    }
}

#[derive(Clone)]
pub struct MessageHandler<'a> {
    listeners: Vec<&'a NotificationListener>,
}

impl<'a> MessageHandler<'a> {
    pub fn connect(port: u16) -> MessageHandler<'a> {
        let (tx, rx): (Sender<String>, Receiver<String>) = channel();
        Connection::start(port, tx);

        MessageHandler {
            listeners: Vec::new(),
        }

    }

    fn wait_for_messages(&self, rx: Receiver<String>) {
        let timeout = Duration::from_millis(100);
        loop {
            if let Ok(msg) = rx.recv() {
                self.message_received(Message::new(msg));
            }
            thread::sleep(timeout);
        }
        
    }

    pub fn register(&mut self, item: &'a NotificationListener) {
        self.listeners.push(item);
    }
}

#[cfg(test)]
mod test {

}
