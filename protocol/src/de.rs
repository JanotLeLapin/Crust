macro_rules! de_num {
    ($name: ident, $type: ty, $bits: literal) => {
        fn $name(&mut self) -> Option<$type> {
            let mut res: $type = 0;
            for i in 0..$bits {
                let b = self.next()? as $type;
                res |= b << 8 * ($bits - 1 - i);
            }
            Some(res)
        }
    };
}

macro_rules! de_varlen {
    ($name: ident, $type: ty, $max_bits: literal) => {
        fn $name(&mut self) -> Option<$type> {
            let mut v: $type = 0;
            let mut i: usize = 0;
            loop {
                let b = self.next()? as $type;
                v |= (b & 0x7F) << i;

                if (b & 0x80) == 0 { break Some(v) };

                i += 7;
                if i >= $max_bits { break  None };
            }
        }
    };
}

pub trait Deserialize {
    fn read_u8(&mut self) -> Option<u8>;
    fn read_i8(&mut self) -> Option<i8>;
    fn read_u16(&mut self) -> Option<u16>;
    fn read_i16(&mut self) -> Option<i16>;
    fn read_i32(&mut self) -> Option<i32>;
    fn read_i64(&mut self) -> Option<i64>;

    fn read_varint(&mut self) -> Option<i32>;
    fn read_varlong(&mut self) -> Option<i64>;

    fn read_string(&mut self) -> Option<String>;
}

impl<T: Iterator<Item = u8>> Deserialize for T {
    de_num!(read_u8, u8, 1);
    de_num!(read_i8, i8, 1);
    de_num!(read_u16, u16, 2);
    de_num!(read_i16, i16, 2);
    de_num!(read_i32, i32, 4);
    de_num!(read_i64, i64, 8);

    de_varlen!(read_varint, i32, 32);
    de_varlen!(read_varlong, i64, 64);

    fn read_string(&mut self) -> Option<String> {
        let len = self.read_varint()?;
        Some(self.take(len as usize).map(|c| c as char).collect())
    }
}
