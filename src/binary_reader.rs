pub enum ByteOrder {
    BigEndian,
    LittleEndian,
}

pub struct BinaryReader<'a> {
    bytes: &'a [u8],
    pos: usize,
    order: ByteOrder,
}

impl<'a> BinaryReader<'a> {
    pub fn from(bytes: &'a [u8], order: ByteOrder) -> Self {
        BinaryReader {
            bytes,
            pos: 0,
            order,
        }
    }

    pub fn skip(&mut self, n: usize) {
        self.pos = self.pos + n;
    }

    pub fn u8(&mut self) -> u8 {
        let byte = self.bytes[self.pos];
        self.pos += 1;
        byte
    }

    pub fn i16(&mut self) -> i16 {
        let mut byte_arr: [u8; 2] = [0; 2];
        byte_arr.copy_from_slice(&self.bytes[(self.pos)..(self.pos + 2)]);
        let n = match self.order {
            ByteOrder::BigEndian => i16::from_be_bytes(byte_arr),
            ByteOrder::LittleEndian => i16::from_le_bytes(byte_arr),
        };
        self.pos += 2;
        n
    }

    pub fn i32(&mut self) -> i32 {
        let bytes = [
            self.bytes[self.pos],
            self.bytes[self.pos + 1],
            self.bytes[self.pos + 2],
            self.bytes[self.pos + 3],
        ];
        let n = match self.order {
            ByteOrder::BigEndian => i32::from_be_bytes(bytes),
            ByteOrder::LittleEndian => i32::from_le_bytes(bytes),
        };
        self.pos += 4;
        n
    }

    pub fn f64(&mut self) -> f64 {
        let bytes = [
            self.bytes[self.pos],
            self.bytes[self.pos + 1],
            self.bytes[self.pos + 2],
            self.bytes[self.pos + 3],
            self.bytes[self.pos + 4],
            self.bytes[self.pos + 5],
            self.bytes[self.pos + 6],
            self.bytes[self.pos + 7],
        ];
        let n = match self.order {
            ByteOrder::BigEndian => f64::from_be_bytes(bytes),
            ByteOrder::LittleEndian => f64::from_le_bytes(bytes),
        };
        self.pos += 8;
        n
    }

    pub fn c_str(&mut self) -> String {
        let start = self.pos;
        while self.bytes[self.pos] != 0x00 {
            self.pos += 1
        }
        let s = std::str::from_utf8(&self.bytes[start..self.pos])
            .expect("Value will be a valid UTF-8 string.")
            .to_owned();
        self.skip(1); // skip the null terminator
        s
    }

    pub fn bytes(&mut self, len: usize) -> Vec<u8> {
        let slice = &self.bytes[self.pos..(self.pos + len)];
        let vec = slice.to_vec();
        self.skip(len);
        vec
    }
}
