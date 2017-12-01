//!
//!
#![warn(missing_docs)]
#![warn(missing_debug_implementations)]
extern crate digest;
extern crate generic_array;

use std::io;

use generic_array::{ArrayLength, GenericArray};
use digest::InvalidLength;
pub use digest::{Input, BlockInput, FixedOutput, VariableOutput, Digest};

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
/// use digest::FixedOutput;
/// use digest_writer::Writer;
/// # fn main() {
/// let mut digest = Writer::new(sha2::Sha256::default());
/// let mut f = File::open("LICENSE-MIT").unwrap();
/// io::copy(&mut f, &mut digest).unwrap();
/// digest.fixed_result();
/// # }
/// ```
#[derive(Debug, Clone, Default)]
pub struct Writer<D>(D);

impl<D: Input + FixedOutput> Writer<D> {
    /// Wrap a Digest into a Writer
    pub fn new(d: D) -> Writer<D> {
        Writer(d)
    }
    /// Returns a reference to underlying Digest object
    pub fn get_ref(&self) -> &D {
        &self.0
    }
    /// Returns a mutable reference to underlying Digest object
    pub fn get_mut(&mut self) -> &mut D {
        &mut self.0
    }
    /// Return the original Digest
    pub fn into_inner(self) -> D {
        self.0
    }
}

impl<D: Input> io::Write for Writer<D> {

    fn write(&mut self, buf: &[u8]) -> io::Result<usize> {
        self.0.process(buf);
        Ok(buf.len())
    }
    fn flush(&mut self) -> io::Result<()> {
        Ok(())
    }
}

impl<D: Input> Input for Writer<D> {
    fn process(&mut self, input: &[u8]) {
        self.0.process(input);
    }
}

impl<D> BlockInput for Writer<D>
    where D: BlockInput, D::BlockSize: ArrayLength<u8>
{
    type BlockSize = D::BlockSize;
}

impl<D> FixedOutput for Writer<D>
    where D: FixedOutput, D::OutputSize: ArrayLength<u8>
{
    type OutputSize = D::OutputSize;

    fn fixed_result(self) -> GenericArray<u8, Self::OutputSize> {
        self.0.fixed_result()
    }
}

impl<D: VariableOutput> VariableOutput for Writer<D> {
    fn new(output_size: usize) -> Result<Writer<D>, InvalidLength> {
        D::new(output_size).map(Writer)
    }
    fn output_size(&self) -> usize {
        self.0.output_size()
    }
    fn variable_result(self, buffer: &mut [u8]) -> Result<&[u8], InvalidLength> {
        self.0.variable_result(buffer)
    }
}
