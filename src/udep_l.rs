use std::net::UdpSocket;

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

  type Item = UdpSocket;


/*
  fn default() -> Self {
    Self {
      ip_port:"172.0.0.1".to_owned(),
      
    }
  }

*/
    fn init(&self) -> Self::Item {

    
    let mut tmp = format!("{}:{}",&self.ip_port.trim(),55555);



    UdpSocket::bind(tmp).expect("failed to bind socket")

    
  }


    fn sokect_write(&self,txt:&str) {


        let socket = self.init();

        socket.send_to(txt.as_bytes(), "127.0.0.1:8888").expect("failed to send data");
        

    }

}
