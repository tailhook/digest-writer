extern crate digest;
extern crate sha2;
extern crate digest_writer;

use std::fs::File;
use std::io;
use sha2::Sha256;
use digest::FixedOutput;
use digest_writer::Writer;

fn main() {
    let mut digest = Writer::new(Sha256::default());
    let mut f = File::open("LICENSE-MIT").unwrap();
    io::copy(&mut f, &mut digest).unwrap();
    assert_eq!(&digest.fixed_result()[..],
        [214, 130, 5, 48, 59, 8, 249, 189, 124, 97, 98, 230, 139,
         13, 96, 161, 131, 7, 139, 1, 165, 29, 101, 162, 79, 61,
         84, 6, 39, 176, 185, 107]);
}
