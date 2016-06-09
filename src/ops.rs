/// ```
/// use hc128::Hc128Rng;
/// assert_eq!(
///     Hc128Rng::init(&[0; 8], &[0; 8]).gen(),
///     1930756226
/// );
/// ```
#[derive(Copy)]
pub struct Hc128Rng {
    p: [u32; 512],
    q: [u32; 512],
    c: u32
}

impl Clone for Hc128Rng { fn clone(&self) -> Hc128Rng { *self } }

impl Hc128Rng {
    pub fn init(key: &[u32], iv: &[u32]) -> Hc128Rng {
        let mut w = [0; 1280];
        let mut hc128 = Hc128Rng {
            p: [0; 512],
            q: [0; 512],
            c: 0
        };

        for i in 0..8 {
            w[i] = key[i];
            w[i + 8] = iv[i];
        }
        for i in 16..1280 {
            w[i] = f2(w[i - 2])
                .wrapping_add(w[i - 7])
                .wrapping_add(f1(w[i - 15]))
                .wrapping_add(w[i - 16])
                .wrapping_add(i as u32);
        }
        for i in 0..512 {
            hc128.p[i] = w[256 + i];
            hc128.q[i] = w[768 + i];
        }
        for i in 0..512 {
            hc128.p[i] = hc128.gen();
        }
        for i in 0..512 {
            hc128.q[i] = hc128.gen();
        }

        hc128.c = 0;
        hc128
    }

    pub fn gen(&mut self) -> u32 {
        let i = self.c as usize & 0x1ff;
        let i3 = i.wrapping_sub(3) & 0x1ff;
        let i10 = i.wrapping_sub(10) & 0x1ff;
        let i12 = i.wrapping_sub(12) & 0x1ff;
        let i511 = i.wrapping_sub(511) & 0x1ff;

        let output = if self.c < 512 {
            self.p[i] = self.p[i]
                .wrapping_add(self.p[i3].rotate_right(10) ^ self.p[i511].rotate_right(23))
                .wrapping_add(self.p[i10].rotate_right(8));
            h(&self.q, self.p[i12]) ^ self.p[i]
        } else {
            self.q[i] = self.q[i]
                .wrapping_add(self.q[i3].rotate_left(10) ^ self.q[i511].rotate_left(23))
                .wrapping_add(self.q[i10].rotate_left(8));
            h(&self.p, self.q[i12]) ^ self.q[i]
        };

        self.c = (self.c + 1) & 0x3ff;
        output
    }
}


#[inline]
fn h(q: &[u32], u: u32) -> u32 {
    q[(u & 0xff) as usize]
        .wrapping_add(q[((u >> 16 & 0xff) + 256) as usize])
}

#[inline]
fn f1(x: u32) -> u32 {
    x.rotate_right(7)
        ^ x.rotate_right(18)
        ^ (x >> 3)
}

#[inline]
fn f2(x: u32) -> u32 {
    x.rotate_right(17)
        ^ x.rotate_right(19)
        ^ (x >> 10)
}
