use std::net::TcpStream;
use std::io::BufWriter;
use std::io::Write;
use std::net::ToSocketAddrs;

pub struct Inf {
    pub ip_port: String,
}



pub trait Socketsp {

  //fn default() -> Self;

  type Item;
  fn init(&self) ->  Self::Item;

  //fn test(&self) -> Self;

  fn sokect_write(&self,txt:&str);
}

impl Socketsp for Inf {

  type Item = TcpStream;


/*
  fn default() -> Self {
    Self {
      ip_port:"172.0.0.1".to_owned(),
      
    }
  }

*/
    fn init(&self) -> Self::Item {

    
    let mut tmp = format!("{}:{}",&self.ip_port.trim(),9999);


    let mut addrs = tmp.to_socket_addrs().unwrap();


    

    TcpStream::connect(tmp).unwrap()
    

    
  }


    fn sokect_write(&self,txt:&str) {
        let msg = format!("{}", txt);
        let mut _writer = BufWriter::new(self.init());
        _writer.write(msg.as_bytes()).expect("SEND FAILURE!!!");

        //&self.writer.flush().unwrap();
    }

}


/*

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

  */