/// A trait to define SPI port behavior
///
/// Blocking vs. Non-blocking behavior is not defined for this trait.
pub trait SpiMaster {
    /// Attempt to read up to `output.len()` bytes. `send_byte` will be sent to
    ///   the slave `output.len()` times. Number of bytes read is returned. If
    ///   an error is returned, no bytes are assumed to be written
    fn read_bytes(&mut self, send_byte: u8, output: &mut [u8]) -> Result<usize, ()>;

    /// Attempt to write all given `bytes`, discarding all output from the slave.
    ///   Number of bytes written is returned. If an error is returned, no bytes
    ///   assumed to be written
    fn write_bytes(&mut self, bytes: &[u8]) -> Result<usize, ()>;

    /// Attempt to write all given `bytes`, replacing each byte with the value
    ///   returned by the slave. Number of bytes written/read is returned. If an
    ///   error is returned, no bytes are assumed to be written
    fn cycle(&mut self, bytes: &mut [u8]) -> Result<usize, ()>;
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {}
}
