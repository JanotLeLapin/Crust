#[derive(crust_macros::Packet)]
#[packet_id(0x00)]
pub struct Status<'a> {
    pub json_response: &'a str,
}

#[derive(crust_macros::Packet)]
#[packet_id(0x01)]
pub struct PingResponse {
    pub payload: i64,
}
