use msg::Message;

trait NotificationListener {
    fn message_received(msg: Message) -> Message;
    fn connectionClosed();
}
