mod de;

pub use de::Deserialize;

macro_rules! varnum {
    ($name: ident, $type: ty, $unsigned_type: ty, $bytes: literal) => {
        pub struct $name(pub $type);

        impl Size for $name {
            fn size(&self) -> usize {
                let v = <$unsigned_type>::from_be_bytes(self.0.to_be_bytes());
                let mut i = 0;
                while ((v >> 7 * i) & 0x80) != 0 { i += 1 }
                i + 1
            }
        }
    };
}

pub trait Size {
    fn size(&self) -> usize;
}

varnum!(VarInt, i32, u32, 4);
varnum!(VarLong, i64, u64, 8);
