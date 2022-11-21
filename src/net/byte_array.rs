pub struct ByteArray {
    pub data: Vec<u8>,
    offset: usize,
}

impl ByteArray {
    pub fn new(data: Option<Vec<u8>>) -> Self {
        ByteArray {
            data: match data {
                Some(data) => data,
                None => Vec::new(),
            },
            offset: 0,
        }
    }

    pub fn space_left(&self) -> usize {
        self.data.len() - self.offset
    }

    pub fn hex_data(&self) -> String {
        let mut hex_data = String::new();
        for byte in &self.data {
            hex_data.push_str(&format!("{:02X}", byte));
        }
        hex_data
    }

    pub fn write_bool(&mut self, value: bool) -> &mut Self {
        self.data.push(if value { 1 } else { 0 });
        self
    }

    pub fn write_byte(&mut self, value: u8) -> &mut Self {
        self.data.push(value);
        self
    }

    pub fn write_short(&mut self, value: u16) -> &mut Self {
        self.data.push((value >> 8) as u8);
        self.data.push(value as u8);
        self
    }

    pub fn write_int(&mut self, value: u32) -> &mut Self {
        self.data.push((value >> 24) as u8);
        self.data.push((value >> 16) as u8);
        self.data.push((value >> 8) as u8);
        self.data.push(value as u8);
        self
    }

    pub fn write_long(&mut self, value: u64) -> &mut Self {
        self.data.push((value >> 56) as u8);
        self.data.push((value >> 48) as u8);
        self.data.push((value >> 40) as u8);
        self.data.push((value >> 32) as u8);
        self.data.push((value >> 24) as u8);
        self.data.push((value >> 16) as u8);
        self.data.push((value >> 8) as u8);
        self.data.push(value as u8);
        self
    }

    pub fn write_utf8(&mut self, value: &str) -> &mut Self {
        self.write_short(value.len() as u16);
        for byte in value.as_bytes() {
            self.data.push(*byte);
        }
        self
    }

    pub fn write_hex(&mut self, value: &str) -> &mut Self {
        let mut hex_data = String::new();
        for byte in value.as_bytes() {
            if byte.is_ascii_hexdigit() {
                hex_data.push(*byte as char);
            }
        }

        let mut bytes = Vec::new();
        for chunk in hex_data.as_bytes().chunks(2) {
            let mut byte = 0;
            for (i, item) in chunk.iter().enumerate() {
                byte += match item {
                    b'0'..=b'9' => (item - b'0') as u8,
                    b'a'..=b'f' => (item - b'a' + 10) as u8,
                    b'A'..=b'F' => (item - b'A' + 10) as u8,
                    _ => 0,
                } << (4 * (1 - i));
            }
            bytes.push(byte);
        }

        self.data.append(&mut bytes);
        self
    }

    pub fn write_raw(&mut self, value: &[u8]) -> &mut Self {
        self.data.extend_from_slice(value);
        self
    }

    pub fn read_utf8(&mut self) -> String {
        let length = self.read_short() as usize;
        let mut string = String::new();
        for _ in 0..length {
            string.push(self.data[self.offset] as char);
            self.offset += 1;
        }
        string
    }

    pub fn read_byte(&mut self) -> u8 {
        let byte = self.data[self.offset];
        self.offset += 1;
        byte
    }

    pub fn read_bool(&mut self) -> bool {
        let byte = self.data[self.offset];
        self.offset += 1;
        byte != 0
    }

    pub fn read_short(&mut self) -> u16 {
        let short = ((self.data[self.offset] as u16) << 8) | (self.data[self.offset + 1] as u16);
        self.offset += 2;
        short
    }

    pub fn read_int(&mut self) -> u32 {
        let int = ((self.data[self.offset] as u32) << 24)
            | ((self.data[self.offset + 1] as u32) << 16)
            | ((self.data[self.offset + 2] as u32) << 8)
            | (self.data[self.offset + 3] as u32);
        self.offset += 4;
        int
    }

    pub fn read_float(&mut self) -> f32 {
        let int = self.read_int();
        f32::from_bits(int)
    }

    pub fn read_long(&mut self) -> u64 {
        let long = ((self.data[self.offset] as u64) << 56)
            | ((self.data[self.offset + 1] as u64) << 48)
            | ((self.data[self.offset + 2] as u64) << 40)
            | ((self.data[self.offset + 3] as u64) << 32)
            | ((self.data[self.offset + 4] as u64) << 24)
            | ((self.data[self.offset + 5] as u64) << 16)
            | ((self.data[self.offset + 6] as u64) << 8)
            | (self.data[self.offset + 7] as u64);
        self.offset += 8;
        long
    }

    pub fn read_fully(&mut self, buffer: &mut [u8]) {
        for i in 0..buffer.len() {
            buffer[i] = self.data[self.offset];
            self.offset += 1;
        }
    }
}
