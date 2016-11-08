/// A trait to define an I2C Master port behavior
///
/// Blocking vs. Non-blocking behavior is not defined for this trait.
pub trait I2cMaster {
    /// Attempt to read up to `output.len()` bytes. Number of bytes read is returned.
    ///   If an error is returned, no bytes are assumed to be read.
    fn read_bytes(&mut self, address: u8, output: &mut [u8]) -> Result<usize, ()>;

    /// Attempt to write all given `bytes`. Number of bytes written is returned.
    ///   If an error is returned, no bytes are assumed to be written
    ///   If a NAK is encountered, writing will be stopped.
    fn write_bytes(&mut self, address: u8, bytes: &[u8]) -> Result<usize, ()>;

}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {}
}
