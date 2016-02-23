//TCP connection

use std::thread;
use std::io::Read;
use std::sync::mpsc::Receiver;
use std::net::{ TcpStream, Ipv4Addr, Shutdown };
use std::time::Duration;
use net_sd::Peer;

pub struct NotifyStream {
    stream: TcpStream,
    localReceiver: Receiver<String>,
}

impl NotifyStream{

    pub fn connect(peer: Peer, receiver: Receiver<String>) {
        let ip = NotifyStream::get_ip_addr(peer.ip);
        let port = peer.port + 1;
        let tcp_s = TcpStream::connect((ip, port));
        println!("{:?}", tcp_s);
        match tcp_s {
            Ok(stream) => { 
                let tcp = NotifyStream {
                    stream: stream,
                    localReceiver: receiver,
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
        let mut buffer = [0, 128];
        let mut msg_buffer = String::new();
        loop {
            let mut stream = self.stream.try_clone().unwrap();
            let incomming = stream.read(&mut buffer);
            if incomming.is_ok() {
                if incomming.unwrap() <= 0 {
                    stream.shutdown(Shutdown::Both);
                    break;
                }
                let stream_data = String::from_utf8(buffer.to_vec()).unwrap();
                msg_buffer = msg_buffer + &stream_data;
                if stream_data.contains('\n') {
                    msg_buffer = msg_buffer.trim_matches('\n').to_string();
                    println!("--> {}", msg_buffer);
                    msg_buffer.clear();
                }
            } else {
                let err = incomming.err().unwrap().raw_os_error();

                if err.is_none() {
                    println!("unknown error occured - closing connection");
                    stream.shutdown(Shutdown::Both);
                    break;
                }

                match err.unwrap() {
                    11 => continue,
                    _ => {
                        stream.shutdown(Shutdown::Both);
                        break;
                    }
                }
            }
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
            ip: String::from("127.0.0.1"),
            ip_dec: 2130706433,
            port: 12345,
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
