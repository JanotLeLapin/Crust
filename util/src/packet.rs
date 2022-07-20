pub struct PacketBuilder {
    packet_id: i32,
    process_id: String,
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

fn to_string(value: String) -> Vec<u8> {
    let mut size = to_varint(value.len() as i32);
    size.append(&mut value.as_bytes().to_vec());
    size
}

impl PacketBuilder {
    pub fn new(packet_id: i32, process_id: String) -> Self {
        PacketBuilder {
            packet_id,
            process_id,
            data: Vec::new(),
        }
    }

    pub fn write_varint(mut self, value: i32) -> Self {
        self.data.append(&mut to_varint(value));
        self
    }

    pub fn write_string(mut self, value: String) -> Self {
        self.data.append(&mut to_string(value));
        self
    }

    pub fn finish(mut self) -> Vec<u8> {
        let mut packet_id = to_varint(self.packet_id);
        let mut length = to_varint((packet_id.len() + self.data.len()) as i32);
        let mut data = to_string(self.process_id);

        data.append(&mut length);
        data.append(&mut packet_id);
        data.append(&mut self.data);
        data
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

    let mut res = String::new();
    for i in 0..size {
        res.push(packet[i + offset] as char);
    };

    Some((res, offset + size))
}

