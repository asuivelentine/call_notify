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
    pub fn new(port: u16) -> Peer {
        use std::sync::mpsc::channel;
        let mut ip = " ".to_string();
        let (tx, rx) = channel();
                
        let sd = ServiceDiscovery::new_with_generator(port, || 1u32).unwrap();
        sd.register_seek_peer_observer(tx.clone());
        sd.seek_peers();

        match rx.recv() {
            Ok(msg) => ip = msg.to_string(),
            x=> println!("{:?}", x),
        }

        Peer {
            ip: ip,
            port: port,
        }
    }
}


#[cfg(test)]
mod tests{
    use super::*;
    use std::sync::mpsc::channel;
    use service_discovery::ServiceDiscovery;

    #[test]
    fn it_works() {

    }

    #[test]
    fn test_net_start() {
        let port = 5000;
        let (tx, _) = channel();
        let sd = ServiceDiscovery::new(port, 42u32).unwrap();
        sd.register_seek_peer_observer(tx.clone());
        sd.set_listen_for_peers(true);

        let peer = Peer::new(5000);
        assert_eq!(42.to_string(), peer.ip);
        drop(peer);
        drop(sd);
    }
}
