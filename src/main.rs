mod models;
#[warn(unused_imports)]
use std::net::ToSocketAddrs;
use std::io;
use std::collections::HashMap;
use actix_cors::Cors;
use std::net::TcpStream;
use std::io::BufWriter;



use models::req::{ReqJsonInput,ReqJsonSenser};
use actix_web::{http,get,post,web,App,HttpResponse,HttpServer,Responder,HttpRequest,Result};
use actix_web::http::header;
use actix_web::middleware::Logger;


mod socket;

mod serial;
//use serial::{Inf,Serial};
mod senser;
mod post;
use post::{PostReq, Inf};
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

#[post("/input")]
async fn input_index(data: web::Json<ReqJsonInput>,reqwests_post:web::Data<Inf>)->impl Responder{

    println!("{:?}",data);

    
    match data.input.as_str() {
        "w" => reqwests_post.req("w").await,
        "a" => reqwests_post.req("a").await,
        "d" => reqwests_post.req("d").await,
        "s" => reqwests_post.req("s").await,
        "stop" => reqwests_post.req(" ").await,
        _ => (),
    }
    

    HttpResponse::Ok().body("ok")
}

#[post("/senser")]
async fn senser_index(data: web::Json<ReqJsonSenser>,reqwests_post:web::Data<Inf>)->impl Responder{

    //println!("{:?}",data);
    

    match data.senser {
        0 => {thread::spawn(||{senser01().unwrap();});},
        1 => reqwests_post.req_senser("1").await,
        2 => reqwests_post.req_senser("2").await,
        3 => reqwests_post.req_senser("3").await,
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

    let mut tmp = format!("{}:{}",ip_,8888);


    let mut addrs = tmp.to_socket_addrs().unwrap();



  
    if let Some(addr) = addrs.find(|x| (*x).is_ipv4()) {
      match TcpStream::connect(addr) {
        Err(_) => {
          println!("Connection NG.");
        }
        Ok(stream) => {
          println!("Connection Ok.");
          let mut writer = BufWriter::new(&stream);
          HttpServer::new(move ||{
            //let serial_inf = Inf {port:port_.trim().to_string(),speed:SPEED_,};
            
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
            .data(writer)
            .service(input_index)
            .service(senser_index)
        })
        .bind("0.0.0.0:8081")
        .run()
        .await

        }
      }
    } else {
      eprintln!("Invalid Host:Port Number");
    }


    println!("{:?}",addrs);

    /*
    let reqwests_post = socket::Inf {
        ip_port:ip:ip_.trim().to_string(),
        writer:

    };
    */

    
}