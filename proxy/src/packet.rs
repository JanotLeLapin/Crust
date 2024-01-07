use crust_protocol::ser::*;

trait Packet {
    fn id() -> VarInt;
}

pub struct Status<'a> {
    pub json_response: &'a str,
}

impl<'a> Packet for Status<'a> { fn id() -> VarInt { VarInt(0x00) } }
impl<'a> Serialize for Status<'a> {
    fn size(&self) -> usize {
        Self::id().size() + self.json_response.size()
    }

    fn serialize(&self) -> Vec<u8> {
        let size = self.size();
        let size = size + VarInt(size as i32).size();
        let mut res = Vec::with_capacity(size);
        res.append(&mut VarInt(size as i32).serialize());
        res.append(&mut Self::id().serialize());
        res.append(&mut self.json_response.serialize());

        res
    }
}

pub struct PingResponse {
    pub payload: i64,
}

impl Packet for PingResponse { fn id() -> VarInt { VarInt(0x01) } }
impl Serialize for PingResponse {
    fn size(&self) -> usize { 9 }

    fn serialize(&self) -> Vec<u8> {
        let mut res = Vec::with_capacity(10);
        res.push(9);
        res.push(Self::id().0 as u8);
        res.append(&mut self.payload.to_be_bytes().to_vec());
        res
    }
}
