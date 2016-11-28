#![feature(test)]

extern crate test;
extern crate rand;
extern crate crypto;
extern crate hc128;

use test::Bencher;
use rand::{ Rng, thread_rng };


#[bench]
fn hc128_bench_once(b: &mut Bencher) {
    use hc128::HC128;

    let mut rng = thread_rng();
    let mut key = [0; 16];
    let mut iv = [0; 16];
    let mut input = [0; 1024];
    let mut output = [0; 1024];
    rng.fill_bytes(&mut key);
    rng.fill_bytes(&mut iv);
    rng.fill_bytes(&mut input);


    b.bytes = input.len() as u64;
    b.iter(|| HC128::new(&key, &iv).process(&input, &mut output))
}

#[bench]
fn crypto_bench_once(b: &mut Bencher) {
    use crypto::hc128::Hc128;
    use crypto::symmetriccipher::SynchronousStreamCipher;

    let mut rng = thread_rng();
    let mut key = [0; 16];
    let mut iv = [0; 16];
    let mut input = [0; 1024];
    let mut output = [0; 1024];
    rng.fill_bytes(&mut key);
    rng.fill_bytes(&mut iv);
    rng.fill_bytes(&mut input);

    b.bytes = input.len() as u64;
    b.iter(|| Hc128::new(&key, &iv).process(&input, &mut output))
}

#[bench]
fn hc128_bench(b: &mut Bencher) {
    use hc128::HC128;

    let mut rng = thread_rng();
    let mut key = [0; 16];
    let mut iv = [0; 16];
    let mut input = [0; 1024];
    let mut output = [0; 1024];
    let mut cipher = HC128::new(&key, &iv);
    rng.fill_bytes(&mut key);
    rng.fill_bytes(&mut iv);
    rng.fill_bytes(&mut input);

    b.bytes = input.len() as u64;
    b.iter(|| cipher.process(&input, &mut output))
}

#[bench]
fn crypto_bench(b: &mut Bencher) {
    use crypto::hc128::Hc128;
    use crypto::symmetriccipher::SynchronousStreamCipher;

    let mut rng = thread_rng();
    let mut key = [0; 16];
    let mut iv = [0; 16];
    let mut input = [0; 1024];
    let mut output = [0; 1024];
    let mut cipher = Hc128::new(&key, &iv);
    rng.fill_bytes(&mut key);
    rng.fill_bytes(&mut iv);
    rng.fill_bytes(&mut input);

    b.bytes = input.len() as u64;
    b.iter(|| cipher.process(&input, &mut output))
}
