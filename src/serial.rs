use std::time::{Duration};
use std::io;
use std::io::Write;
#[derive(Debug, Clone)]
pub struct Inf {
    pub port: String,
    pub speed:u32,
}

pub trait Serial {
    fn write(&self,txt:&str) -> std::io::Result<()>;
    fn read(&self) -> String;
}

impl Serial for Inf { 
    fn write(&self,txt: &str) -> std::io::Result<()> {
        let mut port = serialport::new(self.port.as_str(), self.speed)
        .timeout(Duration::from_millis(10))
        .open()?;
        let output = txt.as_bytes();
        port.write(output)?;
        Ok(())
    }

    fn read(&self) -> String {
        let mut port = serialport::new(self.port.as_str(), self.speed)
        .timeout(Duration::from_millis(10))
        .open().expect("Port Err?");
        let mut serial_buf: Vec<u8> = vec![0; 32];
        loop {
            match port.read(serial_buf.as_mut_slice()) {
                Ok(t) => io::stdout().write_all(&serial_buf[..t]).unwrap(),
                Err(_e) => {},
            }
        }
    }


}