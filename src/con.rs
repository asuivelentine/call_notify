//This module wil handle All network connections.
//It will wait antil the service is found in the lan.
//as soon as it receives a message from the service,
//it will use the ip from the answer to establish an TCP connection

use net_sd::Peer;
use net_tcp::NotifyStream;
use std::time::Duration;
use std::thread;
use std::sync::mpsc::Sender;

pub struct Connection;

impl Connection {
    //start the watchdog for service discovery. This is a Blocking call,
    //after the service is found, start the TCP connection
    pub fn start(port: u16, sender: Sender<String>) {
        let peer = Connection::watchdog_sd(port);
        NotifyStream::connect(peer, sender);
    }

    //request the service, if the service is not present, wait 5 seconds and try it again
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
