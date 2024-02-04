//! Palworld client example.
//!
//! # Usage Examples
//!
//! ```bash
//! $ cargo run --example palworld -- --help
//! $ cargo run --example palworld -- -a 127.0.0.1:25575 -p passwrd -c "Info"
//! ```
use std::io::{Read, Write};
use std::net::TcpStream;
use std::time::Duration;

use anyhow::{anyhow, Result};
use clap::Parser;

use rcon::client::RconClient;

#[derive(Debug, Parser)]
#[clap(name = "palworld")]
struct Args {
    #[clap(long, short = 'a', default_value = "127.0.0.1:25575")]
    remote_address: String,

    #[clap(long, short = 'p', default_value = "password here")]
    rcon_password: String,

    #[clap(long, short = 'c', default_value = "Info")]
    command: String,
}

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

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = Args::parse();

    let mut client = PalworldClient::new(args.remote_address)?;
    client.auth(args.rcon_password.as_ref())?;

    // NOTE: Teleport commands is not supported. because these commands often used in case of playing.
    if let Some(command) = args.command.split_whitespace().next() {
        match command {
            "Shutdown" | "DoExit" | "Broadcast" | "KickPlayer" | "BanPlayer" | "ShowPlayers"
            | "Info" | "Save" => {
                let result = client.execute_command(args.command.as_ref())?;
                println!("{}", result);
                Ok(())
            }
            _ => Err(anyhow!("unsupported command has specified: {}", args.command).into()),
        }
    } else {
        Err(anyhow!("invalid command string: {}", args.command).into())
    }
}
