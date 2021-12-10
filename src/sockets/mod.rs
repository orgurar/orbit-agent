// a server connection stream wrapper socket

use std::io::Result;
use std::net::TcpStream;
use std::io::prelude::*;

pub struct ServerStream {
    stream: TcpStream,
}

impl ServerStream {
    pub fn new(server_ip: &str, server_port: u16) -> Result<Self> {
        let stream = TcpStream::connect(format!("{}:{}", server_ip, server_port))?;
        Ok(Self { stream })
    }

    pub fn receive(&mut self) -> Result<(Vec<u8>, usize)> {
        let mut data = vec![0; 1024];
        let n_bytes = self.stream.read(&mut data)?;

        match n_bytes {
            0 => Ok((vec![0u8], 0)),
            _ => Ok((data, n_bytes))
        }
    }

    pub fn send(&mut self, message: String) -> Result<()> {
        self.stream.write_all(message.as_bytes())
    }
}
