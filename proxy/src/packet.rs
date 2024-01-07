use crust_protocol::ser::*;

pub trait Packet {
    fn id() -> VarInt;
}

#[derive(crust_macros::Packet)]
pub struct Status<'a> {
    pub json_response: &'a str,
}
impl<'a> Packet for Status<'a> { fn id() -> VarInt { VarInt(0x00) } }

#[derive(crust_macros::Packet)]
pub struct PingResponse {
    pub payload: i64,
}
impl Packet for PingResponse { fn id() -> VarInt { VarInt(0x01) } }
