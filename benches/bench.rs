#![feature(test)]

extern crate test;
extern crate rand;
extern crate crypto;
extern crate hc128;
#[macro_use] extern crate lazy_static;

use test::Bencher;
use rand::{ Rng, thread_rng };

lazy_static!{
    static ref KEY: [u8; 16] = {
        let mut key = [0; 16];
        thread_rng().fill_bytes(&mut key);
        key
    };
    static ref IV: [u8; 16] = {
        let mut iv = [0; 16];
        thread_rng().fill_bytes(&mut iv);
        iv
    };
    static ref INPUT: [u8; 1024] = {
        let mut input = [0; 1024];
        thread_rng().fill_bytes(&mut input);
        input
    };
}


#[bench]
fn hc128_bench_once(b: &mut Bencher) {
    use hc128::HC128;

    let mut output = [0; 1024];
    b.bytes = INPUT.len() as u64;
    b.iter(|| HC128::new(&*KEY, &*IV).process(&*INPUT, &mut output))
}

#[bench]
fn crypto_bench_once(b: &mut Bencher) {
    use crypto::hc128::Hc128;
    use crypto::symmetriccipher::SynchronousStreamCipher;

    let mut output = [0; 1024];
    b.bytes = INPUT.len() as u64;
    b.iter(|| Hc128::new(&*KEY, &*IV).process(&*INPUT, &mut output))
}

#[bench]
fn hc128_bench(b: &mut Bencher) {
    use hc128::HC128;

    let mut output = [0; 1024];
    let mut cipher = HC128::new(&*KEY, &*IV);
    b.bytes = INPUT.len() as u64;
    b.iter(|| cipher.process(&*INPUT, &mut output))
}

#[bench]
fn crypto_bench(b: &mut Bencher) {
    use crypto::hc128::Hc128;
    use crypto::symmetriccipher::SynchronousStreamCipher;

    let mut output = [0; 1024];
    let mut cipher = Hc128::new(&*KEY, &*IV);
    b.bytes = INPUT.len() as u64;
    b.iter(|| cipher.process(&*INPUT, &mut output))
}
