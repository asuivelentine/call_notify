//TCP connection

use std::thread;
use std::net::{ TcpStream, Ipv4Addr };
use std::time::Duration;
use net_sd::Peer;

struct NotifyStream {
    stream: TcpStream,
}

impl NotifyStream{

    pub fn connect(peer: Peer, port: u16) {
        let ip = NotifyStream::get_ip_addr(peer.ip);
        let tcp_s = TcpStream::connect((ip, port));
        match tcp_s {
            Ok(stream) => { 
                let tcp = NotifyStream {
                    stream: stream,
                };
                tcp.stream.set_read_timeout(Some(Duration::new(3,0)));
                tcp.stream.set_write_timeout(Some(Duration::new(3,0)));
                thread::spawn( move || {
                    tcp.run()
                });
            }
            Err(err) => panic!(err),
        }
    }

    fn get_ip_addr(ip: String) -> Ipv4Addr {
        let octets: Vec<_> = ip.split('.').collect();
        let octet_iter = octets.into_iter();
        let decimals: Vec<u8> = octet_iter.map(|oct| u8::from_str_radix(oct, 10).unwrap()).collect();

        Ipv4Addr::new(decimals[0], decimals[1], decimals[2], decimals[3])
    }

    fn run(&self) {
        loop {
            
        }
    }

}

#[cfg(test)]
mod tests {
    use super::NotifyStream;
    use net_sd::Peer;

    #[test]
    fn test_connect() {
        let peer = Peer {
            ip: String::from("192.168.0.100"),
            ip_dec: 3232235620,
            port: 12346,
        };

    }

    #[test]
    fn test_get_ip() {
        let ip = String::from("192.168.0.100");
        let ip = NotifyStream::get_ip_addr(ip);
        assert_eq!(192, ip.octets()[0]);
        assert_eq!(168, ip.octets()[1]);
        assert_eq!(0, ip.octets()[2]);
        assert_eq!(100, ip.octets()[3]);
    }

}
