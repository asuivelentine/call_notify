//Service Discovery module
use std::thread;
use std::time::Duration;
use service_discovery::ServiceDiscovery;

//if a peer is found, return the ip and the port to establish a regular tcp connection.
#[derive(Debug, Clone)]
pub struct Peer {
    pub ip: Option<String>,
    pub ip_dec: Option<u32>,
    pub port: u16,
}

impl Peer {
    //creates a peer Object.
    pub fn new(port: u16) -> Peer {
        use std::sync::mpsc::channel;
        let mut ip = None;
        let mut ip_dec = None;
        let (tx, rx) = channel();
                
        let sd = ServiceDiscovery::new(port, 1u32).unwrap();
        sd.register_seek_peer_observer(tx);
        sd.seek_peers();

        //this is needed because try_recv() is called too fast and can't receive anything
        thread::sleep(Duration::from_millis(100));

        //check for incomming data
        //the received data will be the ip of the service in decimal
        match rx.try_recv() {
            Ok(msg) => {
                ip_dec = Some(msg);
                ip = Some(Peer::ip_decimal_to_dotted(msg));
            }
            x=> println!("{:?}", x),
        }

        Peer {
            ip: ip,
            ip_dec: ip_dec,
            port: port,
        }
    }

    //The service will respond with it's ip in decimal
    //calculates the dotted ip from the decimal form
    fn ip_decimal_to_dotted(ip: u32) -> String {
       let first = ip >> 24;
       let second = (ip >> 16) & 0xFF;
       let third = (ip >> 8) & 0xFF;
       let fourth = ip & 0xFF;

       format!("{}.{}.{}.{}", first.to_string(), second.to_string(),
            third.to_string(), fourth.to_string())
    }
}


#[cfg(test)]
mod test{
    use super::*;
    use std::sync::mpsc::channel;
    use service_discovery::ServiceDiscovery;

    #[test]
    fn it_works() {

    }

    #[test]
    fn sd_start_test() {
        let port = 5000;
        let (tx, _) = channel();
        let sd = ServiceDiscovery::new(port, 42u32).unwrap();
        sd.register_seek_peer_observer(tx.clone());
        sd.set_listen_for_peers(true);

        let peer = Peer::new(5000);
        assert!(peer.ip.is_some());
        assert!(peer.ip_dec.is_some());
        assert_eq!(42, peer.ip_dec.unwrap());
        drop(peer);
        drop(sd);
    }

    #[test]
    fn sd_ip_to_dotted_test() {
        let ip = 3232235620;
        assert_eq!("192.168.0.100", Peer::ip_decimal_to_dotted(ip));
    }
}
