use net_sd::Peer;
use net_tcp::NotifyStream;
use std::time::Duration;
use std::thread;
use std::sync::mpsc::Sender;

pub struct Connection;

impl Connection {
    pub fn start(port: u16, sender: Sender<String>) {
        let peer = Connection::watchdog_sd(port);
        NotifyStream::connect(peer, sender);
    }

    fn watchdog_sd(port: u16) -> Peer {
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
