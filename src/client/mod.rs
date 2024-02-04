pub mod response;

use anyhow::Result;
use bytes::{Bytes, BytesMut};

use crate::client::response::AuthResponse;
use crate::error::ClientError;
use crate::packet::packet_type::{RequestPacketType, ResponsePacketType};
use crate::packet::Packet;

pub trait RconClient {
    fn send(&mut self, buf: &[u8]) -> Result<(), Box<dyn std::error::Error>>;
    fn receive(&mut self, buf: &mut [u8]) -> Result<(), Box<dyn std::error::Error>>;

    fn auth(&mut self, rcon_password: &str) -> Result<AuthResponse, Box<dyn std::error::Error>> {
        let packet = Packet::default()
            .identifier(0)
            .request_packet_type(RequestPacketType::ServerDataAuth)
            .payload(rcon_password.as_bytes().to_vec());
        let packet_bytes = Bytes::try_from(packet)?;
        self.send(&packet_bytes)?;

        self.receive(&mut [])?; // empty response

        let mut auth_bytes = [0u8; 4096];
        self.receive(&mut auth_bytes)?;
        let mut auth_buf = BytesMut::from(&auth_bytes[..]);
        let auth_response_packet = Packet::try_from(&mut auth_buf)?;

        if auth_response_packet.identifier == 0 {
            Ok(AuthResponse::AuthenticationSucceeded)
        } else {
            Err(ClientError::AuthenticationError.into())
        }
    }
    fn execute_command(&mut self, command: &str) -> Result<String, Box<dyn std::error::Error>> {
        let command_packet = Packet::default()
            .identifier(0)
            .request_packet_type(RequestPacketType::ServerDataExecCommand)
            .payload(command.as_bytes().to_vec());
        let command_packet_bytes = Bytes::try_from(command_packet)?;
        self.send(&command_packet_bytes)?;

        let empty_packet = Packet::default()
            .identifier(1)
            .response_packet_type(ResponsePacketType::ServerDataResponseValue);
        let empty_packet_bytes = Bytes::try_from(empty_packet)?;
        self.send(&empty_packet_bytes)?;

        // receive loop (for >4096 payloads)
        let mut buf: Vec<u8> = vec![];
        loop {
            let mut receive_buf = [0u8; 4096];
            if self.receive(&mut receive_buf).is_ok() {
                let packet = Packet::try_from(&mut BytesMut::from(&receive_buf[..]))?;
                if packet.identifier != 0 {
                    let payload_string = String::from_utf8(buf)?;
                    return Ok(payload_string);
                }

                buf.extend_from_slice(&packet.payload);
            } else {
                let payload_string = String::from_utf8(buf)?;
                return Ok(payload_string);
            }
        }
    }
}
