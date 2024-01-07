macro_rules! varnum {
    ($name: ident, $type: ty, $unsigned_type: ty, $bytes: literal) => {
        pub struct $name(pub $type);

        impl Serialize for $name {
            fn size(&self) -> usize {
                let v = <$unsigned_type>::from_be_bytes(self.0.to_be_bytes());
                let mut i = 0;
                while ((v >> 7 * i) & 0x80) != 0 { i += 1 }
                i + 1
            }

            fn serialize(&self) -> Vec<u8> {
                let mut v = <$unsigned_type>::from_be_bytes(self.0.to_be_bytes());
                let mut res = Vec::with_capacity($bytes);
                loop {
                    if (v & 0x80) == 0 {
                        res.push(v as u8);
                        break res
                    }

                    res.push((v as u8 & 0x7F) | 0x80);
                    v >>= 7;
                }
            }
        }
    };
}

pub trait Serialize {
    fn size(&self) -> usize;
    fn serialize(&self) -> Vec<u8>;
}

varnum!(VarInt, i32, u32, 4);
varnum!(VarLong, i64, u64, 8);

impl Serialize for String {
    fn size(&self) -> usize { self.len() }

    fn serialize(&self) -> Vec<u8> {
        let size = self.size();
        let mut res = Vec::with_capacity(size + 5);
        res.append(&mut VarInt(size as i32).serialize());
        res.append(&mut self.chars().map(|c| c as u8).collect());
        res
    }
}
