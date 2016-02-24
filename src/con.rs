use std::sync::mpsc::channel;
use std::sync::mpsc::{Sender, Receiver};

struct Connection {
    state: ConnectionState,
}

enum ConnectionState {
    Connected,
    Disconnected,
}

impl Connection {
    pub fn start() {

    }
}

#[cfg(test)]
mod test {
    
}
