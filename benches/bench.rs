#![feature(test)]

extern crate test;
extern crate crypto;
extern crate hc128;

use test::Bencher;


#[bench]
fn hc128_bench_once(b: &mut Bencher) {
    use hc128::HC128;

    let key = [0; 16];
    let iv = [0; 16];
    let input = [0; 1024];
    let mut output = [0; 1024];

    b.bytes = input.len() as u64;
    b.iter(|| HC128::new(&key, &iv).process(&input, &mut output))
}

#[bench]
fn crypto_bench_once(b: &mut Bencher) {
    use crypto::hc128::Hc128;
    use crypto::symmetriccipher::SynchronousStreamCipher;

    let key = [0; 16];
    let iv = [0; 16];
    let input = [0; 1024];
    let mut output = [0; 1024];

    b.bytes = input.len() as u64;
    b.iter(|| Hc128::new(&key, &iv).process(&input, &mut output))
}

#[bench]
fn hc128_bench(b: &mut Bencher) {
    use hc128::HC128;

    let key = [0; 16];
    let iv = [0; 16];
    let input = [0; 1024];
    let mut output = [0; 1024];
    let mut cipher = HC128::new(&key, &iv);

    b.bytes = input.len() as u64;
    b.iter(|| cipher.process(&input, &mut output))
}

#[bench]
fn crypto_bench(b: &mut Bencher) {
    use crypto::hc128::Hc128;
    use crypto::symmetriccipher::SynchronousStreamCipher;

    let key = [0; 16];
    let iv = [0; 16];
    let input = [0; 1024];
    let mut output = [0; 1024];
    let mut cipher = Hc128::new(&key, &iv);

    b.bytes = input.len() as u64;
    b.iter(|| cipher.process(&input, &mut output))
}
