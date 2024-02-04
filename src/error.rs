use thiserror::Error;

/// Errors for serialize/deserialize packets.
#[derive(Error, Debug)]
pub enum PacketError {
    /// Malformed packet error (wrong identifier, etc...)
    #[error("the packet is malformed")]
    MalformedPacketError,
}

/// Errors for client operation(authentication/execute_command).
#[derive(Error, Debug)]
pub enum ClientError {
    /// Authentication error (maybe password is wrong?)
    #[error("authentication failed")]
    AuthenticationError,
}
