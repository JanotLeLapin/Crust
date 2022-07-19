pub struct PacketBuilder {
    id: i32,
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

impl PacketBuilder {
    pub fn new(id: i32) -> Self {
        PacketBuilder {
            id,
            data: Vec::new(),
        }
    }

    pub fn write_varint(mut self, value: i32) -> Self {
        self.data.append(&mut to_varint(value));
        self
    }

    pub fn finish(mut self) -> Vec<u8> {
        let mut id = to_varint(self.id);
        let mut length = to_varint((id.len() + self.data.len()) as i32);

        length.append(&mut id);
        length.append(&mut self.data);
        length
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

