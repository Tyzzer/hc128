#![no_std]

extern crate byteorder;

mod ops;

use byteorder::{ ByteOrder, LittleEndian };
pub use ops::Hc128Rng;


#[derive(Clone)]
pub struct HC128 {
    inner: Hc128Rng,
    buff: u32,
    count: usize
}

impl HC128 {
    pub fn new(key: &[u8], iv: &[u8]) -> HC128 {
        let mut w = [0; 1280];

        for i in 0..4 {
            w[i] = LittleEndian::read_u32(&key[i * 4..][..4]);
            w[i + 8] = LittleEndian::read_u32(&iv[i * 4..][..4]);
            w[i + 4] = w[i];
            w[i + 8 + 4] = w[i + 8];
        }

        HC128 {
            inner: Hc128Rng::with_w(&mut w),
            buff: 0,
            count: 0
        }
    }

    pub fn process(&mut self, input: &[u8], output: &mut [u8]) {
        let mut pos = 0;

        if self.count != 0 && input.len() >= self.count {
            pos += self.count;
            for (i, b) in self.take(pos).enumerate() {
                output[i] = input[i] ^ b;
            }
        }

        while pos + 4 <= input.len() {
            LittleEndian::write_u32(
                &mut output[pos..pos + 4],
                LittleEndian::read_u32(&input[pos..pos + 4]) ^ self.inner.gen()
            );

            pos += 4;
        }

        for b in self.take(input.len() - pos) {
            output[pos] = input[pos] ^ b;
            pos += 1;
        }
    }
}

impl Iterator for HC128 {
    type Item = u8;

    fn next(&mut self) -> Option<Self::Item> {
        if self.count == 0 {
            self.buff = self.inner.gen();
            self.count = 4;
        }
        let output = (self.buff & 0xff) as u8;
        self.buff >>= 8;
        self.count -= 1;
        Some(output)
    }
}
