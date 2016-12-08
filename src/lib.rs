//!
//!
#![warn(missing_docs)]
extern crate digest;
extern crate generic_array;

use std::io;

use digest::Digest;
use generic_array::GenericArray;

/// A wrapper around Digest type that allows to use Write trait for hashing
///
/// # Example
///
/// ```rust
/// extern crate sha2;
/// extern crate digest;
/// extern crate digest_writer;
/// use std::fs::File;
/// use std::io::{self, Write};
/// use digest::Digest;
/// use digest_writer::Writer;
/// # fn main() {
/// let mut digest = Writer::new(sha2::Sha256::new());
/// let mut f = File::open("LICENSE-MIT").unwrap();
/// io::copy(&mut f, &mut digest).unwrap();
/// digest.result();
/// # }
/// ```
pub struct Writer<D>(D);

impl<D: Digest> Writer<D> {
    /// Wrap a Digest into a Writer
    pub fn new(d: D) -> Writer<D> {
        Writer(d)
    }
    /// Return the original Digest
    pub fn into_inner(self) -> D {
        self.0
    }
}

impl<D: Digest> io::Write for Writer<D> {

    fn write(&mut self, buf: &[u8]) -> io::Result<usize> {
        self.0.input(buf);
        Ok(buf.len())
    }
    fn flush(&mut self) -> io::Result<()> {
        Ok(())
    }
}

impl<D: Digest> Digest for Writer<D> {
    type OutputSize = D::OutputSize;
    type BlockSize = D::BlockSize;
    fn input(&mut self, input: &[u8]) {
        self.0.input(input);
    }
    fn result(self) -> GenericArray<u8, Self::OutputSize> {
        self.0.result()
    }

    fn block_bytes(&self) -> usize {
        self.0.block_bytes()
    }
    fn block_bits(&self) -> usize {
        self.0.block_bits()
    }
    fn output_bytes(&self) -> usize {
        self.0.output_bytes()
    }
    fn output_bits(&self) -> usize {
        self.0.output_bits()
    }
}
