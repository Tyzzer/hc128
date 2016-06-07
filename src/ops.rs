macro_rules! step {
    ( @step $h:ident, $t:expr, $x:expr, $tv:expr, $tc:expr, $tb:expr, ( $u:expr, $a:expr, $d:expr, $n:expr ) ) => {{
        let tmp3 = $h(&$t, $x[$d]);
        $t[$u] = $t[$u]
            .wrapping_add($tb)
            .wrapping_add($tv ^ $tc);
        $x[$a] = $t[$u];
        $n = tmp3 ^ $t[$u];
    }};
    ( P $ctx:expr, $u:expr, $v:expr, $a:expr, $b:expr, $c:expr, $d:expr, $n:expr ) => {
        step!(@step
            h1, $ctx.t, $ctx.x,
            $ctx.t[$v].rotate_right(23),
            $ctx.x[$c].rotate_right(10),
            $ctx.x[$b].rotate_right(8),
            ($u, $a, $d, $n)
        )
    };
    ( Q $ctx:expr, $u:expr, $v:expr, $a:expr, $b:expr, $c:expr, $d:expr, $n:expr ) => {
        step!(@step
            h2, $ctx.t, $ctx.y,
            $ctx.t[$v].rotate_right(32 - 23),
            $ctx.y[$c].rotate_right(32 - 10),
            $ctx.y[$b].rotate_right(32 - 8),
            ($u, $a, $d, $n)
        )
    }
}

macro_rules! update {
    ( @update $h:ident, $t:expr, $x:expr, $tv:expr, $tc:expr, $tb:expr, ( $u:expr, $a:expr, $d:expr ) ) => {{
        let tmp3 = $h(&$t, $x[$d]);
        $t[$u] = $t[$u]
            .wrapping_add($tb)
            .wrapping_add($tv ^ $tc)
            ^ tmp3;
        $x[$a] = $t[$u];
    }};
    ( P $ctx:expr, $u:expr, $v:expr, $a:expr, $b:expr, $c:expr, $d:expr ) => {
        update!(@update
            h1, $ctx.t, $ctx.x,
            $ctx.t[$v].rotate_right(23),
            $ctx.x[$c].rotate_right(10),
            $ctx.x[$b].rotate_right(8),
            ($u, $a, $d)
        )
    };
    ( Q $ctx:expr, $u:expr, $v:expr, $a:expr, $b:expr, $c:expr, $d:expr ) => {
        update!(@update
            h2, $ctx.t, $ctx.y,
            $ctx.t[$v].rotate_right(32 - 23),
            $ctx.y[$c].rotate_right(32 - 10),
            $ctx.y[$b].rotate_right(32 - 8),
            ($u, $a, $d)
        )
    };
}


pub struct Hc128Rng {
    t: [u32; 1024],
    x: [u32; 16],
    y: [u32; 16],
    c: u32
}

impl Hc128Rng {
    pub fn init(key: &[u32], iv: &[u32]) -> Hc128Rng {
        let mut hc128 = Hc128Rng {
            t: [0; 1024],
            x: [0; 16],
            y: [0; 16],
            c: 0
        };

        for i in 0..8 {
            hc128.t[i] = key[i];
        }
        for i in 8..16 {
            hc128.t[i] = iv[i - 8];
        }
        for i in 16..(256 + 16) {
            hc128.t[i] = f2(hc128.t[i - 2])
                .wrapping_add(hc128.t[i - 7])
                .wrapping_add(f1(hc128.t[i - 15]))
                .wrapping_add(hc128.t[i - 16])
                .wrapping_add(i as u32);
        }
        for i in 0..16 {
            hc128.t[i] = hc128.t[256 + i];
        }
        for i in 16..1024 {
            hc128.t[i] = f2(hc128.t[i - 2])
                .wrapping_add(hc128.t[i - 7])
                .wrapping_add(f1(hc128.t[i - 15]))
                .wrapping_add(hc128.t[i - 16])
                .wrapping_add(256 + i as u32);
        }
        for i in 0..16 {
            hc128.x[i] = hc128.t[512 - 16 + i];
            hc128.y[i] = hc128.t[512 + 512 - 16 + i];
        }
        for _ in 0..64 {
            hc128.update();
        }

        hc128
    }

    pub fn gen(&mut self) -> [u32; 16] {
        let mut output = [0; 16];
        let cc = self.c as usize & 0x1ff;
        let dd = (cc + 16) & 0x1ff;

        if self.c < 512 {
            step!(P self, cc + 0, cc + 1,  0,  6,  13,  4, output[0]);
            step!(P self, cc + 1, cc + 2,  1,  7,  14, 5,  output[1]);
            step!(P self, cc + 2, cc + 3,  2,  8,  15, 6,  output[2]);
            step!(P self, cc + 3, cc + 4,  3,  9,  0,  7,  output[3]);
            step!(P self, cc + 4, cc + 5,  4,  10, 1,  8,  output[4]);
            step!(P self, cc + 5, cc + 6,  5,  11, 2,  9,  output[5]);
            step!(P self, cc + 6, cc + 7,  6,  12, 3,  10, output[6]);
            step!(P self, cc + 7, cc + 8,  7,  13, 4,  11, output[7]);
            step!(P self, cc + 8, cc + 9,  8,  14, 5,  12, output[8]);
            step!(P self, cc + 9, cc + 10, 9,  15, 6,  13, output[9]);
            step!(P self, cc + 10,cc + 11, 10, 0,  7,  14, output[10]);
            step!(P self, cc + 11,cc + 12, 11, 1,  8,  15, output[11]);
            step!(P self, cc + 12,cc + 13, 12, 2,  9,  0,  output[12]);
            step!(P self, cc + 13,cc + 14, 13, 3,  10, 1,  output[13]);
            step!(P self, cc + 14,cc + 15, 14, 4,  11, 2,  output[14]);
            step!(P self, cc + 15,dd + 0,  15, 5,  12, 3,  output[15]);
        } else {
            step!(Q self, 512 + cc + 0,  512 + cc + 1,  0,  6,  13, 4,  output[0]);
            step!(Q self, 512 + cc + 1,  512 + cc + 2,  1,  7,  14, 5,  output[1]);
            step!(Q self, 512 + cc + 2,  512 + cc + 3,  2,  8,  15, 6,  output[2]);
            step!(Q self, 512 + cc + 3,  512 + cc + 4,  3,  9,  0,  7,  output[3]);
            step!(Q self, 512 + cc + 4,  512 + cc + 5,  4,  10, 1,  8,  output[4]);
            step!(Q self, 512 + cc + 5,  512 + cc + 6,  5,  11, 2,  9,  output[5]);
            step!(Q self, 512 + cc + 6,  512 + cc + 7,  6,  12, 3,  10, output[6]);
            step!(Q self, 512 + cc + 7,  512 + cc + 8,  7,  13, 4,  11, output[7]);
            step!(Q self, 512 + cc + 8,  512 + cc + 9,  8,  14, 5,  12, output[8]);
            step!(Q self, 512 + cc + 9,  512 + cc + 10, 9,  15, 6,  13, output[9]);
            step!(Q self, 512 + cc + 10, 512 + cc + 11, 10, 0,  7,  14, output[10]);
            step!(Q self, 512 + cc + 11, 512 + cc + 12, 11, 1,  8,  15, output[11]);
            step!(Q self, 512 + cc + 12, 512 + cc + 13, 12, 2,  9,  0,  output[12]);
            step!(Q self, 512 + cc + 13, 512 + cc + 14, 13, 3,  10, 1,  output[13]);
            step!(Q self, 512 + cc + 14, 512 + cc + 15, 14, 4,  11, 2,  output[14]);
            step!(Q self, 512 + cc + 15, 512 + dd + 0,  15, 5,  12, 3,  output[15]);
        }

        self.c = (self.c + 16) & 0x3ff;
        output
    }

    #[inline]
    fn update(&mut self) {
        let cc = self.c as usize & 0x1ff;
        let dd = (cc + 16) & 0x1ff;

        if self.c < 512 {
            update!(P self, cc + 0,  cc + 1,  0,  6,  13, 4);
            update!(P self, cc + 1,  cc + 2,  1,  7,  14, 5);
            update!(P self, cc + 2,  cc + 3,  2,  8,  15, 6);
            update!(P self, cc + 3,  cc + 4,  3,  9,  0,  7);
            update!(P self, cc + 4,  cc + 5,  4,  10, 1,  8);
            update!(P self, cc + 5,  cc + 6,  5,  11, 2,  9);
            update!(P self, cc + 6,  cc + 7,  6,  12, 3,  10);
            update!(P self, cc + 7,  cc + 8,  7,  13, 4,  11);
            update!(P self, cc + 8,  cc + 9,  8,  14, 5,  12);
            update!(P self, cc + 9,  cc + 10, 9,  15, 6,  13);
            update!(P self, cc + 10, cc + 11, 10, 0,  7,  14);
            update!(P self, cc + 11, cc + 12, 11, 1,  8,  15);
            update!(P self, cc + 12, cc + 13, 12, 2,  9,  0);
            update!(P self, cc + 13, cc + 14, 13, 3,  10, 1);
            update!(P self, cc + 14, cc + 15, 14, 4,  11, 2);
            update!(P self, cc + 15, dd + 0,  15, 5,  12, 3);
        } else {
            update!(Q self, 512 + cc + 0,  512 + cc + 1,  0,  6,  13, 4);
            update!(Q self, 512 + cc + 1,  512 + cc + 2,  1,  7,  14, 5);
            update!(Q self, 512 + cc + 2,  512 + cc + 3,  2,  8,  15, 6);
            update!(Q self, 512 + cc + 3,  512 + cc + 4,  3,  9,  0,  7);
            update!(Q self, 512 + cc + 4,  512 + cc + 5,  4,  10, 1,  8);
            update!(Q self, 512 + cc + 5,  512 + cc + 6,  5,  11, 2,  9);
            update!(Q self, 512 + cc + 6,  512 + cc + 7,  6,  12, 3,  10);
            update!(Q self, 512 + cc + 7,  512 + cc + 8,  7,  13, 4,  11);
            update!(Q self, 512 + cc + 8,  512 + cc + 9,  8,  14, 5,  12);
            update!(Q self, 512 + cc + 9,  512 + cc + 10, 9,  15, 6,  13);
            update!(Q self, 512 + cc + 10, 512 + cc + 11, 10, 0,  7,  14);
            update!(Q self, 512 + cc + 11, 512 + cc + 12, 11, 1,  8,  15);
            update!(Q self, 512 + cc + 12, 512 + cc + 13, 12, 2,  9,  0);
            update!(Q self, 512 + cc + 13, 512 + cc + 14, 13, 3,  10, 1);
            update!(Q self, 512 + cc + 14, 512 + cc + 15, 14, 4,  11, 2);
            update!(Q self, 512 + cc + 15, 512 + dd + 0,  15, 5,  12, 3);
        }

        self.c = (self.c + 16) & 0x3ff;
    }
}

impl Clone for Hc128Rng {
    fn clone(&self) -> Hc128Rng {
        let mut hc128 = Hc128Rng {
            t: [0; 1024],
            x: [0; 16],
            y: [0; 16],
            c: 0
        };
        hc128.t.clone_from_slice(&self.t);
        hc128.x.clone_from_slice(&self.x);
        hc128.y.clone_from_slice(&self.y);
        hc128.c = self.c;
        hc128
    }
}


#[inline]
fn h1(t: &[u32], x: u32) -> u32 {
    t[512 + (x & 0xff) as usize]
        .wrapping_add(t[512 + 256 + (x >> 16 & 0xff) as usize])
}

#[inline]
fn h2(t: &[u32], x: u32) -> u32 {
    t[(x & 0xff) as usize]
        .wrapping_add(t[256 + (x >> 16 & 0xff) as usize])
}

#[inline]
fn f1(x: u32) -> u32 {
    x.rotate_right(7)
        ^ x.rotate_right(18)
        ^ x.wrapping_shr(3)
}

#[inline]
fn f2(x: u32) -> u32 {
    x.rotate_right(17)
        ^ x.rotate_right(19)
        ^ x.wrapping_shr(10)
}
