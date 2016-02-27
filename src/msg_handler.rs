use msg::{ Message, MessageKind };
use con::Connection;
use std::thread;
use std::time::Duration;
use std::sync::mpsc::{ Sender, Receiver };
use std::sync::mpsc::channel;

pub trait NotificationListener {
    fn message_received(&self, msg: Message) -> Message;
    fn connection_closed(&self);
}

impl <'a> NotificationListener for &'a MessageHandler<'a>{
    fn message_received(&self, msg: Message) -> Message {
        if msg.kind == MessageKind::ConnectionClosed {
            self.connection_closed();
        }
        for x in self.listeners.iter() {
            x.message_received(msg.clone());
            println!("{:?}", msg);
        }
        msg
    }

    fn connection_closed(&self) {
        for listener in self.listeners.iter() {
            listener.connection_closed();
        }
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

        let handler = MessageHandler {
            listeners: Vec::new(),
        };
        //will not work! only for test purposes
        let mut hnd = handler.clone();
        hnd.wait_for_messages(rx);

        handler
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
