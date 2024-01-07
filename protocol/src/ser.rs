macro_rules! num {
    ($type: ty) => {
        impl Serialize for $type {
            fn size(&self) -> usize {
                std::mem::size_of::<$type>()
            }

            fn serialize(&self) -> Vec<u8> {
                self.to_be_bytes().to_vec()
            }
        }
    };
}

macro_rules! varnum {
    ($name: ident, $type: ty, $unsigned_type: ty, $bytes: literal) => {
        #[derive(Debug, PartialEq)]
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

num!(u8);
num!(i8);
num!(u16);
num!(i16);
num!(i32);
num!(i64);

varnum!(VarInt, i32, u32, 4);
varnum!(VarLong, i64, u64, 8);

impl<'a> Serialize for &'a str {
    fn size(&self) -> usize {
        let len = self.len();
        VarInt(len as i32).size() + len
    }

    fn serialize(&self) -> Vec<u8> {
        let mut res = Vec::with_capacity(self.size());
        res.append(&mut VarInt(self.len() as i32).serialize());
        res.append(&mut self.chars().map(|c| c as u8).collect());
        res
    }
}

#[cfg(test)]
mod test {
    use crate::ser::Serialize;

    #[test]
    fn varint() {
        assert_eq!(super::VarInt(0).serialize(), [0x00]);
        assert_eq!(super::VarInt(1).serialize(), [0x01]);
        assert_eq!(super::VarInt(2).serialize(), [0x02]);
        assert_eq!(super::VarInt(127).serialize(), [0x7F]);
        assert_eq!(super::VarInt(128).serialize(), [0x80, 0x01]);
        assert_eq!(super::VarInt(255).serialize(), [0xFF, 0x01]);
        assert_eq!(super::VarInt(2097151).serialize(), [0xFF, 0xFF, 0x7F]);
        assert_eq!(super::VarInt(2147483647).serialize(), [0xFF, 0xFF, 0xFF, 0xFF, 0x07]);
        assert_eq!(super::VarInt(-1).serialize(), [0xFF, 0xFF, 0xFF, 0xFF, 0x0F]);
    }

    #[test]
    fn string() {
        assert_eq!("foo".serialize(), [3, 0x66, 0x6F, 0x6F]);
        assert_eq!("bar".serialize(), [3, 0x62, 0x61, 0x72]);
    }
}
