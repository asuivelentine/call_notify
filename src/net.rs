//TCP module
use service_discovery::ServiceDiscovery;
use rustc_serialize::{ Encodable, Decodable };

#[derive(Debug, Clone, RustcEncodable, RustcDecodable)]
pub struct Peer {
    ip: String,
    port: u16,
}

impl Peer {
    fn new(port: u16) -> Peer {
        Peer {
            ip: " ".to_string(),
            port: port,
        }
    }
}

struct ServDis;

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
    }
}
