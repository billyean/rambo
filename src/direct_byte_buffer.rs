use std::error::Error;
use bytebuffer::ByteBuffer;

pub type DirectByteBuffer = ByteBuffer;
pub type ByteBufferSlice<'a> = &'a [u8];


pub trait DirectBuffer {
    fn allocate_vec_for_buffer(capacity: usize) -> Result<Vec<u8>, Box<dyn Error>>;
}

pub trait Slice {
    fn create_buffer_slice (
        self,
        start: usize,
        len: usize,
    ) -> Result<ByteBufferSlice, Box<dyn Error>>;
}

impl DirectBuffer for ByteBuffer {
    fn allocate_vec_for_buffer(capacity: usize) -> Result<Vec<u8>, Box<dyn Error>> {
        todo!()
    }
}

impl Slice for ByteBuffer {
    fn create_buffer_slice(self, start: usize, len: usize) -> Result<ByteBufferSlice, Box<dyn Error>> {
        let end = start + len;
        Ok(&self.as_bytes()[start..end])
    }
}
