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
    pub fn start(&self) -> Peer{
        let port = self.port.clone();
        Connection::watchdog_sd(port)
    }

    fn watchdog_sd(port: u16) {
        let mut peer = Peer::new(port);

        while peer.ip.is_none() {
            peer = Peer::new(port);
            thread::sleep(Duration::from_millis(5000)); 
        }

        peer
    }
}

#[cfg(test)]
mod test {
    
}
