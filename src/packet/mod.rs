pub mod packet_type;

use anyhow::Result;
use bytes::{Buf, BufMut, Bytes, BytesMut};
use num_traits::ToPrimitive;

use crate::error::PacketError::MalformedPacketError;
use crate::packet::packet_type::{RequestPacketType, ResponsePacketType};

#[derive(Debug)]
pub struct Packet {
    pub size: usize,
    pub identifier: i32,
    pub packet_type: i32,
    pub payload: Vec<u8>,
}

impl Default for Packet {
    fn default() -> Self {
        Self {
            size: 10,
            identifier: 0,
            packet_type: ToPrimitive::to_i32(&RequestPacketType::ServerDataAuth).unwrap(),
            payload: vec![],
        }
    }
}

impl Packet {
    pub fn identifier(mut self, identifier: i32) -> Self {
        self.identifier = identifier;
        self
    }

    pub fn request_packet_type(mut self, packet_type: RequestPacketType) -> Self {
        self.packet_type = packet_type as i32;
        self
    }

    pub fn response_packet_type(mut self, packet_type: ResponsePacketType) -> Self {
        self.packet_type = packet_type as i32;
        self
    }

    pub fn payload(mut self, payload: Vec<u8>) -> Self {
        self.size = 10 + payload.len();
        self.payload = payload;
        self
    }
}

impl TryFrom<&mut BytesMut> for Packet {
    type Error = anyhow::Error;

    fn try_from(bytes: &mut BytesMut) -> Result<Packet, Self::Error> {
        let size = bytes.get_i32_le() as usize;
        let identifier = bytes.get_i32_le();
        let packet_type = bytes.get_i32_le();
        let payload = bytes.split_to(size.saturating_sub(10)).into();
        Ok(Packet {
            size,
            identifier,
            packet_type,
            payload,
        })
    }
}

impl TryFrom<Packet> for Bytes {
    type Error = anyhow::Error;

    fn try_from(packet: Packet) -> Result<Bytes, Self::Error> {
        // packet size(4 bytes) + packet itself size(packet.size bytes)
        let mut buffer = BytesMut::with_capacity(packet.size + 4);
        buffer.put_i32_le(packet.size as i32);
        buffer.put_i32_le(packet.identifier);
        let packet_type = ToPrimitive::to_i32(&packet.packet_type)
            .ok_or_else(|| anyhow::Error::new(MalformedPacketError))?;
        buffer.put_i32_le(packet_type);
        buffer.put(packet.payload.as_ref());
        buffer.put_slice(b"\x00\x00");
        Ok(buffer.freeze())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use num_traits::FromPrimitive;

    #[test]
    fn serialize_packet() -> Result<(), Box<dyn std::error::Error>> {
        let rcon_password = "passwrd".as_bytes().to_vec();
        let auth_packet = Packet::default()
            .identifier(0)
            .request_packet_type(RequestPacketType::ServerDataAuth)
            .payload(rcon_password);
        let auth_packet_bytes = Bytes::try_from(auth_packet)?;
        let want_packet_bytes = Bytes::from_static(&[
            0x11, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x03, 0x00, 0x00, 0x00, 0x70, 0x61,
            0x73, 0x73, 0x77, 0x72, 0x64, 0x00, 0x00,
        ]);
        assert_eq!(*auth_packet_bytes, *want_packet_bytes);

        Ok(())
    }

    #[test]
    fn deserialize_packet() -> Result<(), Box<dyn std::error::Error>> {
        // authentication response is constructed with empty `ResponseValue` and empty `AuthResponse`.

        // Tests empty `ResponseValue`.
        let mut response_value_packet_bytes = BytesMut::from(
            &[
                0x0a, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
            ][..],
        );
        let response_value_packet = Packet::try_from(&mut response_value_packet_bytes)?;
        assert_eq!(response_value_packet.size, 0xa);
        assert_eq!(response_value_packet.identifier, 0);
        let packet_type: ResponsePacketType =
            FromPrimitive::from_i32(response_value_packet.packet_type)
                .ok_or_else(|| anyhow::Error::new(MalformedPacketError))?;
        assert_eq!(packet_type, ResponsePacketType::ServerDataResponseValue);
        assert_eq!(response_value_packet.payload.len(), 0);

        // Tests empty `AuthResponse`.
        let mut auth_response_packet_bytes = BytesMut::from(
            &[
                0x0a, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x02, 0x00, 0x00, 0x00, 0x00, 0x00,
            ][..],
        );
        let auth_packet = Packet::try_from(&mut auth_response_packet_bytes)?;
        assert_eq!(auth_packet.size, 0xa);
        assert_eq!(auth_packet.identifier, 0);
        let packet_type: ResponsePacketType = FromPrimitive::from_i32(auth_packet.packet_type)
            .ok_or_else(|| anyhow::Error::new(MalformedPacketError))?;
        assert_eq!(packet_type, ResponsePacketType::ServerDataAuthResponse);
        assert_eq!(auth_packet.payload.len(), 0);

        Ok(())
    }
}
