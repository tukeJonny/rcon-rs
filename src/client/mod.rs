/// This module provides SRCDS response types.
pub mod response;

use anyhow::Result;
use bytes::{Bytes, BytesMut};

use crate::client::response::AuthResponse;
use crate::error::ClientError;
use crate::packet::packet_type::{RequestPacketType, ResponsePacketType};
use crate::packet::Packet;

/// A trait providing methods for authenticate/execute command to SRCDS.
pub trait RconClient {
    /// Sends bytes
    fn send(&mut self, buf: &[u8]) -> Result<(), Box<dyn std::error::Error>>;
    /// Receives bytes
    fn receive(&mut self, buf: &mut [u8]) -> Result<(), Box<dyn std::error::Error>>;

    /// Authenticate to SRCDS
    fn auth(&mut self, rcon_password: &str) -> Result<AuthResponse, Box<dyn std::error::Error>> {
        let auth_packet = Bytes::try_from(
            Packet::default()
                .identifier(0)
                .request_packet_type(RequestPacketType::ServerDataAuth)
                .payload(rcon_password.as_bytes().to_vec()),
        )?;
        self.send(&auth_packet)?;

        self.receive(&mut [])?; // empty response

        let mut auth_bytes = [0u8; 4096];
        self.receive(&mut auth_bytes)?;
        let auth_response_packet = Packet::try_from(&mut BytesMut::from(&auth_bytes[..]))?;
        if auth_response_packet.identifier == 0 {
            Ok(AuthResponse::AuthenticationSucceeded)
        } else {
            Err(ClientError::AuthenticationError.into())
        }
    }
    /// Execute command to SRCDS.
    fn execute_command(&mut self, command: &str) -> Result<String, Box<dyn std::error::Error>> {
        let command_packet = Bytes::try_from(
            Packet::default()
                .identifier(0)
                .request_packet_type(RequestPacketType::ServerDataExecCommand)
                .payload(command.as_bytes().to_vec()),
        )?;
        self.send(&command_packet)?;

        let empty_packet = Bytes::try_from(
            Packet::default()
                .identifier(1)
                .response_packet_type(ResponsePacketType::ServerDataResponseValue),
        )?;
        self.send(&empty_packet)?;

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
