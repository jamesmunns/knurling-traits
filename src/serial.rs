/// A trait to define Serial (UART) port behavior
///
/// Blocking vs. Non-blocking behavior is not defined for this trait.
pub trait Serial {
    /// Attempt to read up to `output.len()` bytes. Number of bytes read is returned.
    ///   If an error is returned, no bytes are assumed to be read
    fn read_bytes(&mut self, output: &mut [u8]) -> Result<usize, ()>;

    /// Attempt to write all given `bytes`. Number of bytes written is returned.
    ///   If an error is returned, no bytes are assumed to be written
    fn write_bytes(&mut self, bytes: &[u8]) -> Result<usize, ()>;
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {}
}
