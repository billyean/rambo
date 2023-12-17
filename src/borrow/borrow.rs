use std::error::Error;

pub type DirectBuffer = Vec<u8>;
fn b(capacity: usize) -> Result<DirectBuffer, Box<dyn Error>> {
    Ok(vec![0; capacity])
}
fn main() -> Result<(), Box<dyn Error>>{
    if let b = b(10)? {
        println!("len = {}", b.len());
    }
    Ok(())
}