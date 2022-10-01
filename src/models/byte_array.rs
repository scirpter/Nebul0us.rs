use std::ops::Mul;

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
            hex_data.push_str(&format!("{:02x}", byte).to_uppercase());
        }
        hex_data
    }

    pub fn write_bool(&mut self, value: bool) {
        self.data.push(if value { 1 } else { 0 });
    }

    pub fn write_byte(&mut self, value: u8) {
        self.data.push(value);
    }

    pub fn write_short(&mut self, value: u16) {
        self.data.push((value >> 8) as u8);
        self.data.push(value as u8);
    }

    pub fn write_int(&mut self, value: u32) {
        self.data.push((value >> 24) as u8);
        self.data.push((value >> 16) as u8);
        self.data.push((value >> 8) as u8);
        self.data.push(value as u8);
    }

    pub fn write_long(&mut self, value: u64) {
        self.data.push((value >> 56) as u8);
        self.data.push((value >> 48) as u8);
        self.data.push((value >> 40) as u8);
        self.data.push((value >> 32) as u8);
        self.data.push((value >> 24) as u8);
        self.data.push((value >> 16) as u8);
        self.data.push((value >> 8) as u8);
        self.data.push(value as u8);
    }

    pub fn write_utf8(&mut self, value: &str) {
        self.write_short(value.len() as u16);
        for byte in value.as_bytes() {
            self.data.push(*byte);
        }
    }

    pub fn write_hex(&mut self, value: &str) {
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
    }

    pub fn write_raw(&mut self, value: &[u8]) {
        for byte in value {
            self.data.push(*byte);
        }
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

    /*
    GAME FUNCTIONS
    */
    pub fn gfx_d(&mut self) -> u8 {
        ((self.read_byte() & 255) << 16)
            + ((self.read_byte() & 255) << 8)
            + (self.read_byte() & 255)
    }

    pub fn gfx_b(&mut self, f10: f32) -> f32 {
        (((f10 - 0.0)
            * (((((self.read_byte()) & 255) << 16) as f32)
                + (((self.read_byte() & 255) << 8) as f32)
                + ((self.read_byte() & 255) << 0) as f32))
            / 1.6777215e7)
            + 0.0
    }

    pub fn gfx_i(&mut self, f: f32, f2: f32) -> f32 {
        (((self.read_short() & 255) as f32) * (f2 - f)) / 255.0 + f
    }

    pub fn gfx_n(&mut self, f: f32, f2: f32) -> f32 {
        (((self.read_byte() & 255) as f32) * (f2 - f)) / 255.0 + f
    }

    pub fn gfx_q(&mut self, f: f32, f2: f32) -> f32 {
        ((((self.read_byte() & 255) << 16)
            + ((self.read_byte() & 255) << 8)
            + (self.read_byte() & 255)) as f32
            * (f2 - f))
            / 1.6777215e7
            + f
    }

    pub fn gfx_m1715j(&mut self, f: f32) -> f32 {
        ((f - 0.0) * ((self.read_short() & 65535) as f32)) / 65535.0 + 0.0
    }

    pub fn gfx_m1726i(&mut self) -> u8 {
        ((self.read_byte() & 255) << 16)
            + ((self.read_byte() & 255) << 8)
            + (self.read_byte() & 255)
            << 0
    }

    pub fn m1717b(&mut self, f: f32) -> f32 {
        ((f - 0.0)
            * ((((self.read_byte() & 255) << 16) as f32)
                + (((self.read_byte() & 255) << 8) as f32)
                + ((self.read_byte() & 255) << 0) as f32))
            / 1.6777215e7
            + 0.0
    }

    pub fn m1718a(&mut self, f: f32, f2: f32) -> f32 {
        ((f2 - f) * ((self.read_byte() & 255) as f32)) / 255.0 + f
    }
}
