use crust_protocol::ser::*;

trait Packet {
    fn id() -> VarInt;
}

pub struct Status {
    pub json_response: String,
}

impl Packet for Status { fn id() -> VarInt { VarInt(0x00) } }
impl Serialize for Status {
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
