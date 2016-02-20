//TCP module
use service_discovery::ServiceDiscovery;

//if a peer is found, return the ip and the port to establish a regular tcp connection.
#[derive(Debug, Clone)]
pub struct Peer {
    pub ip: String,
    pub port: u16,
}

impl Peer {
    //creates a peer Object.
    fn new(port: u16) -> Peer {
        use std::sync::mpsc::channel;
        let (tx, rx) = channel();
        tx.send(1).unwrap(); 
                
        let sd = ServiceDiscovery::new(port, 1u32).unwrap();
        sd.register_seek_peer_observer(tx);
        sd.seek_peers();

        match rx.recv() {
            Ok(0u32) => (),
            x=> println!("{:?}", x),
        }

        Peer {
            ip: " ".to_string(),
            port: port,
        }
    }
}


#[cfg(test)]
mod tests{
    use super::Peer;

    #[test]
    fn it_works() {

    }

    #[test]
    fn test_net_start() {
        let peer = Peer::new(5000);
        assert_eq!(" ".to_string(), peer.ip);
        drop(peer);
    }
}
