use msg::{ Message, MessageKind };
use con::Connection;
use std::thread;
use std::time::Duration;
use std::sync::mpsc::{ Sender, Receiver };
use std::sync::mpsc::channel;

// this trait is used to inform other modules about incomming messages
// to reveive messages, the module has to register itself.
pub trait NotificationListener {
    fn message_received(&self, msg: Message) -> Message;
    fn connection_closed(&self);
}

impl <'a> NotificationListener for &'a MessageHandler<'a>{
    fn message_received(&self, msg: Message) -> Message {
        if msg.kind == MessageKind::ConnectionClosed {
            self.connection_closed();
            return msg;
        }
        for x in self.listeners.iter() {
            x.message_received(msg.clone());
        }
        msg
    }

    fn connection_closed(&self) {
       for listener in self.listeners.iter() {
            listener.connection_closed();
        }
       self.wait_for_messages();
    }
}

#[derive(Clone)]
pub struct MessageHandler<'a> {
    listeners: Vec<&'a NotificationListener>,
    port: u16,
}

impl<'a> MessageHandler<'a> {
    pub fn new(port: u16) ->  MessageHandler<'a> {
        MessageHandler {
            listeners: Vec::new(),
            port: port,
        }
    }

    pub fn run(self) {
        self.wait_for_messages();
    }

    fn wait_for_messages(&self) {
        let (tx, rx): (Sender<String>, Receiver<String>) = channel();
        Connection::start(self.port, tx);

        let timeout = Duration::from_millis(100);
        loop {
            if let Ok(msg) = rx.recv() {
                &self.message_received(Message::new(msg));
            }
            thread::sleep(timeout);
        }

    }

    pub fn register(&mut self, item: &'a NotificationListener) -> &mut MessageHandler<'a> {
        self.listeners.push(item);
        self
    }
}

#[cfg(test)]
mod test {

}
