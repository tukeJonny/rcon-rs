use num_derive::{FromPrimitive, ToPrimitive};

#[derive(PartialEq, Eq, Debug, Clone, Copy, Hash, FromPrimitive, ToPrimitive)]
pub enum RequestPacketType {
    ServerDataAuth = 3,
    ServerDataExecCommand = 2,
}

#[derive(PartialEq, Eq, Debug, Clone, Copy, Hash, FromPrimitive, ToPrimitive)]
pub enum ResponsePacketType {
    ServerDataAuthResponse = 2,
    ServerDataResponseValue = 0,
}
