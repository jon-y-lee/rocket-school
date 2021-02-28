use std::net::UdpSocket;
use crate::producer::temperature_event::Event;
use serde_json::value::Value::String;
use crate::producer::event_type::EventType;
use std::str;

pub struct MulticastProducer {
    socket: UdpSocket
}

impl MulticastProducer {
    pub fn new() -> Self {
        MulticastProducer {
            socket: UdpSocket::bind("127.0.0.1:30000").unwrap()
        }
    }

    pub fn init(&self) {
        self.socket.set_nonblocking(true).unwrap();
    }

    pub fn recv_sync(&self) {
        let mut recvBuffer: [u8; 1000] = [0; 1000];
        let number_of_bytes: usize = 0;
        let mut result: Vec<u8> = Vec::new();

        match self.socket.recv_from(&mut recvBuffer) {
            Ok((num_of_bytes, n)) => {
                result = Vec::from(&recvBuffer[0..num_of_bytes]);
                let display_result = result.clone();
                println!("result:{:?}", display_result);
                let s = match str::from_utf8(&display_result) {
                    Ok(v) => v,
                    Err(e) => panic!("Invalid UTF-8 sequence: {}", e),
                };
                println!("result: {:?}", s);
            },
            Err(ref e) => {
                // wait until network socket is ready, typically implemented
                // via platform-specific APIs such as epoll or IOCP
                //wait_for_fd();
                println!("Error error error");
            }
            Err(e) => panic!("encountered IO error: {}", e),
        };
    }

    pub fn send(&self, payload: Event<EventType>) {
        // Serialize it to a JSON string.
        let serialiedPayload = serde_json::to_string(&payload).unwrap();

        // Print, write to a file, or send to an HTTP server.
        println!("{}", serialiedPayload);

        self.socket.send_to(&serialiedPayload.into_bytes(), "127.0.0.1:30000").expect("couldn't send data");

        self.recv_sync();
    }
}