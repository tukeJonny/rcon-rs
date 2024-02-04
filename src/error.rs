use thiserror::Error;

#[derive(Error, Debug)]
pub enum PacketError {
    #[error("the packet is malformed")]
    MalformedPacketError,
}

#[derive(Error, Debug)]
pub enum ClientError {
    #[error("authentication failed")]
    AuthenticationError,
}
