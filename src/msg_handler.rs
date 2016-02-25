use msg::Message;

trait NotificationListener {
    fn message_received(&self, msg: Message) -> Message;
    fn connectionClosed(&self);
}

struct MessageHandler;

impl MessageHandler {

}

#[cfg(test)]
mod test {

}
