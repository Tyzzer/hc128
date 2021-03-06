pub struct Hc128Rng {
    p: [u32; 512],
    q: [u32; 512],
    c: usize
}

impl Clone for Hc128Rng {
    fn clone(&self) -> Hc128Rng {
        let mut p = [0; 512];
        let mut q = [0; 512];
        p.copy_from_slice(&self.p);
        q.copy_from_slice(&self.q);
        Hc128Rng { p, q, c: self.c }
    }
}

impl Hc128Rng {
    pub fn init(key: &[u32; 4], iv: &[u32; 4]) -> Hc128Rng {
        let mut w = [0; 1280];
        w[..4].copy_from_slice(key);
        w[4..8].copy_from_slice(key);
        w[8..12].copy_from_slice(iv);
        w[12..16].copy_from_slice(iv);
        Self::with_w(&mut w)
    }

    pub fn with_w(w: &mut [u32; 1280]) -> Hc128Rng {
        let mut hc128 = Hc128Rng {
            p: [0; 512],
            q: [0; 512],
            c: 0
        };

        for i in 16..1280 {
            w[i] = f2(w[i - 2])
                .wrapping_add(w[i - 7])
                .wrapping_add(f1(w[i - 15]))
                .wrapping_add(w[i - 16])
                .wrapping_add(i as u32);
        }
        hc128.p.copy_from_slice(&w[256..768]);
        hc128.q.copy_from_slice(&w[768..]);
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
        let i = self.c & 0x1ff;
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

#[test]
fn test() {
   assert_eq!(
       Hc128Rng::init(&[0; 4], &[0; 4]).gen(),
       1930756226
   );
}
