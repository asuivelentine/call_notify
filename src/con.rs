use net_sd::Peer;
use net_tcp::NotifyStream;
use std::time::Duration;
use std::thread;
use std::sync::mpsc::channel;
use std::sync::mpsc::{Sender, Receiver};

pub struct Connection {
    state: ConnectionState,
    port: u16,
}

enum ConnectionState {
    Connected,
    Disconnected,
}

impl Connection {
    pub fn new(port: u16) -> Connection {
        Connection {
            state: ConnectionState::Disconnected,
            port: port,
        }
    }
    pub fn start(&self) {

    }
}

#[cfg(test)]
mod test {
    
}
