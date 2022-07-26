use serde::{Serialize,Deserialize};
use serde_json::{Value,json};

#[derive(Serialize,Deserialize)]
pub struct Packet {
    pub pid: String,
    #[serde(skip_serializing_if = "Value::is_null")]
    pub state: Value,
    pub data: Vec<u8>,
}

pub struct PacketBuilder {
    packet_id: i32,
    process_id: String,
    state: Value,
    data: Vec<u8>,
}

fn to_varint(value: i32) -> Vec<u8> {
    let mut value = u32::from_ne_bytes(value.to_ne_bytes());
    let mut varint: Vec<u8> = Vec::new();
    loop {
        if (value & 0x80) == 0 {
            varint.push(value as u8);
            break varint;
        }

        varint.push((value as u8 & 0x7F) | 0x80);
        value >>= 7;
    }
}

fn to_sized<T: num::Num + num::ToPrimitive>(value: T) -> Vec<u8> {
    let mut sized: Vec<u8> = Vec::new();
    let size = std::mem::size_of_val(&value);
    for i in 0..size {
        sized.push(((value.to_usize().unwrap() >> (size - 1 - i) * 8) & 0xFF) as u8)
    };
    sized
}

impl PacketBuilder {
    pub fn new(packet_id: i32, process_id: String) -> Self {
        PacketBuilder {
            packet_id,
            process_id,
            state: Value::Null,
            data: Vec::new(),
        }
    }

    pub fn state(mut self, state: Value) -> Self {
        self.state = state;
        self
    }

    pub fn write_varint(mut self, value: i32) -> Self {
        self.data.append(&mut to_varint(value));
        self
    }

    pub fn write_string(mut self, value: String) -> Self {
        let mut size = to_varint(value.len() as i32);
        self.data.append(&mut size);
        self.data.append(&mut value.as_bytes().to_vec());
        self
    }

    pub fn write_sized<T: num::Num + num::ToPrimitive>(mut self, value: T) -> Self {
        self.data.append(&mut to_sized(value));
        self
    }

    pub fn finish(mut self) -> Vec<u8> {
        // Minecraft packet format, will be interpreted by the client
        let mut packet_id = to_varint(self.packet_id);
        let mut length = to_varint((packet_id.len() + self.data.len()) as i32);
        let mut data = vec![];

        data.append(&mut length);
        data.append(&mut packet_id);
        data.append(&mut self.data);

        // Additional metadata, will be interpreted by the proxy
        let mut wrap = json!(Packet {
            pid: self.process_id,
            state: self.state,
            data,
        }).to_string().into_bytes();
        let mut packet = to_sized(wrap.len() as u32);
        packet.append(&mut wrap);

        // The final packet should look like:
        // <packet size>{
        //   "pid": <proxy process id>,
        //   "state": <proxy process state>,
        //   "data": <Minecraft packet>
        // }
        packet
    }
}

pub fn read_varint(packet: &Vec<u8>, offset: usize) -> Option<(i32, usize)> {
    let mut i = 0;
    let mut v: i32 = 0;
    loop {
        let b = packet[i + offset];
        v |= (b as i32 & 0x7F) << (i * 7);

        if (b & 0x80) == 0 {
            return Some((v, i + offset + 1));
        };

        i += 1;
        if i > 5 {
            return None;
        }
    }
}

pub fn read_string(packet: &Vec<u8>, offset: usize) -> Option<(String, usize)> {
    let (size, offset) = match read_varint(&packet, offset) {
        Some((size, length)) => (size as usize, length),
        None => {
            return None;
        }
    };

    match String::from_utf8(packet[offset..size+offset].to_vec()) {
        Ok(res) => Some((res, offset + size)),
        Err(_) => None
    }
}

pub fn read_sized<T: num::Num + num::FromPrimitive>(packet: &Vec<u8>, offset: usize) -> (T, usize) {
    let size = std::mem::size_of::<T>();
    let mut value: u64 = packet[offset] as u64;
    for i in 1..size {
        value = value << 8 | packet[offset + i] as u64;
    };

    (num::FromPrimitive::from_u64(value).unwrap(), size)
}

