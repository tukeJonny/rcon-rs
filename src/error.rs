use thiserror::Error;

#[derive(Error, Debug)]
pub enum PacketError {
    #[error("the packet is malformed")]
    MalformedPacketError,
}
