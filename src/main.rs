mod models;
#[warn(unused_imports)]
use std::net::ToSocketAddrs;
use std::io;
use std::collections::HashMap;
use actix_cors::Cors;
use std::net::TcpStream;
use std::io::BufWriter;
use std::net::UdpSocket;



use models::req::{ReqJsonInput,ReqJsonInputnew,ReqJsonSenser,ReqJsonInputnewad};
use actix_web::{http,get,post,web,App,HttpResponse,HttpServer,Responder,HttpRequest,Result};
use actix_web::http::header;
use actix_web::middleware::Logger;
use std::process::{Stdio, Command};
use std::{env};


mod socket_l;
mod udep_l;
use crate::udep_l::Socketsp;
//use socket_l::{Inf,Socketsp};

mod serial;
//use serial::{Inf,Serial};
mod senser;
mod post;
//use post::{PostReq, Inf};
use senser::{senser01};
use std::thread;
extern crate serde;
use std::sync::Mutex;

#[macro_use]
extern crate serde_derive;
extern crate serde_json;
//use std::time::{Duration, Instant};


/*
fn serial_write(txt:&str) {

    let mut port = serialport::new("COM11", 115_200)
    .timeout(Duration::from_millis(10))
    .open().expect("Failed to open port");
    let output = txt.as_bytes();
    port.write(output).expect("Write failed!");

}
*/


/*

fn sread(serial_inf:Inf){

    serial_inf.read();
}

*/

/*

fn udp(txt:&str){
    let socket = UdpSocket::bind("127.0.0.1:11111").expect("failed to bind socket");

    socket.send_to(txt.as_bytes(), "127.0.0.1:8888").expect("failed to send data");

    
}
*/

pub fn get_exe_dir() -> String {
    match env::current_exe() {
        Ok(p) => {
            let mut path = p.clone();
            path.pop();
            match path.into_os_string().into_string() {
                Ok(p2) => { return p2; }
                Err(_) => "./".to_string()
            }
        }
        Err(_) => { "./".to_string() }
    }
}

fn python(){
    //println!("{}",get_exe_dir());
    let mut child = Command::new("python")
        .args(&["-u".to_string(), get_exe_dir() + "/test.py"])
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()
        .unwrap();

        child.stdout.as_mut().unwrap();
}

#[post("/input")]
async fn input_index(data: web::Json<ReqJsonInputnewad>,inf:web::Data<udep_l::Inf>)->impl Responder{

    println!("{:?}",data);
    let mut tmp = format!(r#"{{"Power": "{}", "Direction": "{}","Rotate":"{}" ,"Stop": "{}"}}"#,data.power,data.direction,data.rotate,data.stop);
    
    println!("{}",tmp);

    inf.sokect_write(&tmp);
    


    HttpResponse::Ok().body("ok")
}




#[post("/inputad")]
async fn inputad_index(data: web::Json<ReqJsonInputnewad>,inf:web::Data<udep_l::Inf>)->impl Responder{

    println!("{:?}",data);
    let mut tmp = format!(r#"{{"Power": "{}", "Direction": "{}","Rotate":"{}" ,"Stop": "{}"}}"#,data.power,data.direction,data.rotate,data.stop);
    
    println!("{}",tmp);

    inf.sokect_write(&tmp);
    


    HttpResponse::Ok().body("ok")
}





#[post("/senser")]
async fn senser_index(data: web::Json<ReqJsonSenser>,reqwests_post:web::Data<udep_l::Inf>)->impl Responder{

    //println!("{:?}",data);
    
    
    match data.senser {
        0 => {thread::spawn(||{senser01().unwrap();});},
        1 => python(),
        2 => {println!("2");},
        3 => (),
        _ => (),
    }

    HttpResponse::Ok().body("ok")
}




#[actix_web::main]
async fn main()->std::io::Result<()>{
    /*
    
    let ports = serialport::available_ports().expect("No ports found!");
    for p in ports { println!("{}", p.port_name );}

    println!("Serial-port:");
    */

    /*

    let mut port_:String = String::new();

    io::stdin().read_line(&mut port_).expect("Failed to read line");

    const SPEED_: u32 = 115_200;
    let port_2 = port_.clone();
    let serial_inf_test = Inf {port:port_.trim().to_string(),speed:SPEED_,};
    serial_inf_test.write("hello").unwrap();


    thread::spawn(move ||{
        let serial_inf = Inf {port:port_2.trim().to_string(),speed:SPEED_,};
        sread(serial_inf);

    });

*/  



    std::env::set_var("RUST_LOG", "actix_web=info");

    let mut ip_:String = String::new();

    io::stdin().read_line(&mut ip_).expect("Failed to read line");

    /*

    let mut tmp = format!("{}:{}",ip_.trim(),8888);


    let mut addrs = tmp.to_socket_addrs().unwrap();

    if let Some(addr) = addrs.find(|x| (*x).is_ipv4()) {
        match TcpStream::connect(addr) {
          Err(_) => {
            println!("Connection NG.");
          }
          Ok(stream) => {
            println!("Connection Ok.");
            let mut writer = BufWriter::new(&stream);
            println!("{:?}",stream);
            
  
          }
        }
      } else {
        eprintln!("Invalid Host:Port Number");
      }
  */
    


    //println!("{:?}",addrs);

    /*
    let reqwests_post = socket::Inf {
        ip_port:ip:ip_.trim().to_string(),
        writer:

    };
    */


    HttpServer::new(move ||{
        let datas = udep_l::Inf {ip_port:ip_.trim().to_string()};

        let cors = Cors::default()
            .allowed_origin_fn(|origin, _req_head| {
                true
            })
            .allowed_methods(vec!["GET", "POST"])
            .allowed_headers(vec![header::AUTHORIZATION, header::ACCEPT])
            .allowed_header(header::CONTENT_TYPE)
            .supports_credentials()
            .max_age(3600);

        App::new()
        .wrap(cors)
        .data(datas)
        .service(input_index)
        .service(senser_index)
    })
    .bind("0.0.0.0:8081")?
    .run()
    .await

    
}