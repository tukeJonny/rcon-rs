use std::io::{Read, Write};
use std::net::TcpStream;
use std::time::Duration;

use anyhow::Result;

use rcon::client::RconClient;

#[derive(Debug)]
pub struct PalworldClient {
    stream: TcpStream,
}

impl<'a> PalworldClient {
    fn new(remote_addr: String) -> Result<Self, Box<dyn std::error::Error>> {
        let stream = TcpStream::connect(remote_addr)?;
        stream.set_read_timeout(Some(Duration::from_secs(1)))?;
        Ok(Self { stream })
    }
}

impl RconClient for PalworldClient {
    fn send(&mut self, b: &[u8]) -> Result<(), Box<dyn std::error::Error>> {
        self.stream.write(b)?;
        Ok(())
    }
    fn receive(&mut self, b: &mut [u8]) -> Result<(), Box<dyn std::error::Error>> {
        self.stream.read(b)?;
        Ok(())
    }
}

fn main() {
    let mut client =
        PalworldClient::new("127.0.0.1:25575".to_string()).expect("failed to initiate client");

    client.auth("<password here>").expect("failed to auth");

    let result = client
        .execute_command("Info")
        .expect("failed to execute command");
    println!("Result:\n{:?}", result);
}
