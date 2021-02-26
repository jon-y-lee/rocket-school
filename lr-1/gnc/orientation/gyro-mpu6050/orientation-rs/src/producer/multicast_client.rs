use std::net::UdpSocket;

pub struct MulticastProducer {
    socket: UdpSocket
}

impl MulticastProducer {
    fn new() -> Self {
        MulticastProducer {
            socket: UdpSocket::bind("127.0.0.1:3000")
        }
    }

    pub fn test(&self) {
        let mut buf = [0; 10];
        buf[0] = 'A';
        println!("Test multicast");
        self.socket.send_to(buf, &src)?;
    }
}