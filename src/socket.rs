use std::net::TcpStream;
use std::io::BufWriter;

pub struct Inf {
    pub ip_port: String,
    pub writer: BufWriter<TcpStream>,
}



pub trait Socketsp {
  fn sokect_write(&self,txt:&str) -> std::io::Result<()>;
}

impl Socketsp for Inf {
    fn sokect_write(&self,txt:&str) -> std::io::Result<()> {
        let msg = format!("{}", txt);
        &self.writer.write(msg.as_bytes()).expect("SEND FAILURE!!!");
        &self.writer.flush().unwrap();
        Ok(())
    }

}




fn sokect() {
    let host_and_port = format!("{}:{}", host, port);
    let mut addrs = host_and_port.to_socket_addrs().unwrap();
  
    if let Some(addr) = addrs.find(|x| (*x).is_ipv4()) {
      match TcpStream::connect(addr) {
        Err(_) => {
          println!("Connection NG.");
        }
        Ok(stream) => {
          println!("Connection Ok.");
          let mut reader = BufReader::new(&stream);
          let mut writer = BufWriter::new(&stream);

          read_something(&mut reader);
          write_something(&mut writer, "hoge");
        }
      }
    } else {
      eprintln!("Invalid Host:Port Number");
    }
  }