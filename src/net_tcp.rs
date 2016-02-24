//TCP connection

use std::thread;
use std::io::{ Read, ErrorKind };
use std::sync::mpsc::Sender;
use std::net::{ TcpStream, Ipv4Addr, Shutdown };
use std::time::Duration;
use net_sd::Peer;

pub struct NotifyStream {
    stream: TcpStream,
    local_sender: Sender<String>,
}

impl NotifyStream{

    pub fn connect(peer: Peer, sender: Sender<String>) {
        let ip = NotifyStream::get_ip_addr(peer.ip);
        let port = peer.port + 1;
        let tcp_s = TcpStream::connect((ip, port));
        match tcp_s {
            Ok(stream) => { 
                let tcp = NotifyStream {
                    stream: stream,
                    local_sender: sender,
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
        let mut stream = self.stream.try_clone().unwrap();
        loop {
            let incomming = stream.read(&mut buffer);
            if incomming.is_ok() {
                if incomming.unwrap() <= 0 {
                    stream.shutdown(Shutdown::Both);
                    break;
                }
                let stream_data = String::from_utf8(buffer.to_vec()).unwrap();
                msg_buffer = msg_buffer + &stream_data;
                if stream_data.contains('\n') {
                    //removes \n and all bytes after it
                    msg_buffer = NotifyStream::remove_trailing_bytes(msg_buffer);
                    match self.local_sender.send(msg_buffer.clone()) {
                        Ok(_) => {},
                        Err(_) => {
                            stream.shutdown(Shutdown::Both);
                            break;
                        }
                    }
                    msg_buffer.clear();
                }
            } else {
                let err = incomming.err();

                if err.is_none() {
                    println!("unknown error occured - closing connection");
                    stream.shutdown(Shutdown::Both);
                    break;
                }

                match err.unwrap().kind() {
                    ErrorKind::WouldBlock => continue,
                    ErrorKind::TimedOut => continue,
                    _ => {
                        stream.shutdown(Shutdown::Both);
                        break;
                    }
                }
            }
        }
    }

    fn remove_trailing_bytes(msg: String) -> String {
        let mut s = msg.clone();
        let beta_offset = s.find('\n').unwrap_or(s.len());
        let t: String = s.drain(..beta_offset).collect();

        t.trim().to_string()
    }

}

#[cfg(test)]
mod tests {
    use super::NotifyStream;
    use std::thread;
    use std::io::Write;
    use std::time::Duration;
    use net_sd::Peer;
    use std::sync::mpsc::{Sender, Receiver, channel};
    use std::net::{ TcpListener, TcpStream, Shutdown };

    #[test]
    fn tcp_connect_test() {
        let peer = Peer {
            ip: String::from("127.0.0.1"),
            ip_dec: 2130706433,
            port: 12345,
        };
        let msg = String::from("testString\n").into_bytes();

        let (tx, rx): (Sender<String>, Receiver<String>) = channel();
        let listener = TcpListener::bind("127.0.0.1:12346");
        assert!(listener.is_ok());

        let listener = listener.unwrap();
        thread::spawn (move || {
            NotifyStream::connect(peer, tx);
        });
        let stream = listener.accept();
        thread::sleep(Duration::from_millis(7000));
        assert!(stream.is_ok());
        let mut stream = stream.unwrap().0;
        stream.write(&msg);

        //thread::sleep(Duration::from_millis(500));

        stream.shutdown(Shutdown::Both);
        let reply = rx.recv();
        assert!(reply.is_ok());
        let reply = reply.unwrap();
        assert_eq!("testString", reply);
    }

    #[test]
    fn tcp_get_ip_test() {
        let ip = String::from("192.168.0.100");
        let ip = NotifyStream::get_ip_addr(ip);
        assert_eq!(192, ip.octets()[0]);
        assert_eq!(168, ip.octets()[1]);
        assert_eq!(0, ip.octets()[2]);
        assert_eq!(100, ip.octets()[3]);
    }

    #[test]
    fn tcp_remove_trailing_bytes_test() {
        let msg = String::from("testString \ng");
        let msg = NotifyStream::remove_trailing_bytes(msg);
        assert_eq!("testString".to_string(), msg);
    }

}
