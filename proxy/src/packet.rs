use bytes::Buf;

pub trait RawPacket {
    fn get_varint(&mut self) -> Option<i32>;
    fn get_string(&mut self) -> Option<String>;
}

impl RawPacket for bytes::Bytes {
    fn get_varint(&mut self) -> Option<i32> {
        let mut v = 0;
        let mut i = 0;
        loop {
            let byte = self.get_u8();
            v |= ((byte & 0x7F) << i) as i32;
            if (byte & 0x80) == 0 { break Some(v) }

            i += 1;
            if i > 4 { break None }
        }
    }

    fn get_string(&mut self) -> Option<String> {
        let len = self.get_varint()? as usize;
        let mut res = String::with_capacity(len);
        for _ in 0..len { res.push(self.get_u8() as char); }
        Some(res)
    }
}
