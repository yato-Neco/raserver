mod models;
#[warn(unused_imports)]
use std::io;
use std::collections::HashMap;

use models::req::{ReqJsonInput,ReqJsonSenser};
use actix_web::{get,post,web,App,HttpResponse,HttpServer,Responder,HttpRequest};
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

    //println!("{:?}",data);

    match data.input.as_str() {
        "w" => reqwests_post.req("w").await,
        "a" => reqwests_post.req("a").await,
        "d" => reqwests_post.req("d").await,
        "s" => reqwests_post.req("s").await,
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


    let mut ip_:String = String::new();

    io::stdin().read_line(&mut ip_).expect("Failed to read line");

    HttpServer::new(move ||{
        //let serial_inf = Inf {port:port_.trim().to_string(),speed:SPEED_,};
        let reqwests_post = Inf {ip:ip_.trim().to_string()};
        App::new()
        .data(reqwests_post)
        .service(input_index)
        .service(senser_index)
    })
    .bind("0.0.0.0:8080")?
    .run()
    .await
}