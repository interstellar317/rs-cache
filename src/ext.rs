use std::io::{ self, Read };

/// Adds easy byte reading onto a [`Read`] instance.
/// 
/// [`Read`]: https://doc.rust-lang.org/std/io/trait.Read.html
pub trait ReadExt: Read {
    fn read_u8(&mut self) -> io::Result<u8>;
    fn read_i8(&mut self) -> io::Result<i8>;
    fn read_u16(&mut self) -> io::Result<u16>;
    fn read_u24(&mut self) -> io::Result<u32>;
    fn read_i32(&mut self) -> io::Result<i32>;
    fn read_string(&mut self) -> io::Result<String>;
}

impl<T: Read> ReadExt for T {
    #[inline]
    fn read_u8(&mut self) -> io::Result<u8> {
        let mut buffer = [0; 1];
        self.read_exact(&mut buffer)?;

        Ok(u8::from_be_bytes(buffer))
    }
    
    #[inline]
    fn read_i8(&mut self) -> io::Result<i8> {
        let mut buffer = [0; 1];
        self.read_exact(&mut buffer)?;

        Ok(i8::from_be_bytes(buffer))
    }
    
    #[inline]
    fn read_u16(&mut self) -> io::Result<u16> {
        let mut buffer = [0; 2];
        self.read_exact(&mut buffer)?;

        Ok(u16::from_be_bytes(buffer))
    }
    
    #[inline]
    fn read_u24(&mut self) -> io::Result<u32> {
        let mut buffer = [0; 3];
        self.read_exact(&mut buffer)?;

        Ok(((buffer[0] as u32) << 16) | ((buffer[1] as u32) << 8) | (buffer[2] as u32))
    }
    
    #[inline]
    fn read_i32(&mut self) -> io::Result<i32> {
        let mut buffer = [0; 4];
        self.read_exact(&mut buffer)?;

        Ok(i32::from_be_bytes(buffer))
    }
    
    #[inline]
    fn read_string(&mut self) -> io::Result<String> {
        let mut bytes = Vec::new();
    
        loop {
            let mut buffer = [0; 1];
            self.read_exact(&mut buffer)?;
            let byte = u8::from_be_bytes(buffer);
            if byte != 0 {
                bytes.push(byte);
            } else {
                break;
            }
        }
    
        Ok(String::from_utf8_lossy(&bytes[..]).to_string())
    }
}