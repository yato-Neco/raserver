mod models;
#[warn(unused_imports)]
use std::io;
use models::req::{ReqJsonInput,ReqJsonSenser};
use actix_web::{get,post,web,App,HttpResponse,HttpServer,Responder,HttpRequest};
mod serial;
use serial::{Inf,Serial};
use std::thread;
extern crate serde;
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



fn sread(serial_inf:Inf){

    serial_inf.read();
}



#[post("/input")]
async fn input_index(data: web::Json<ReqJsonInput>,serial_inf:web::Data<Inf>)->impl Responder{

    //println!("{:?}",data);



    match data.input.as_str() {
        "w" =>serial_inf.write("w").unwrap(),
        "a" => serial_inf.write("a").unwrap(),
        "d" => serial_inf.write("d").unwrap(),
        "s" => serial_inf.write("s").unwrap(),
        _ => (),
    }
    

    HttpResponse::Ok().body("ok")
}

#[post("/senser")]
async fn senser_index(data: web::Json<ReqJsonSenser>,serial_inf:web::Data<Inf>)->impl Responder{

    //println!("{:?}",data);

    match data.senser {
        0 =>serial_inf.write("s0").unwrap(),
        1 => serial_inf.write("s1").unwrap(),
        2 => serial_inf.write("s2").unwrap(),
        3 => serial_inf.write("s3").unwrap(),
        _ => (),
    }



    HttpResponse::Ok().body("ok")
}

#[actix_web::main]
async fn main()->std::io::Result<()>{
    
    let ports = serialport::available_ports().expect("No ports found!");
    for p in ports { println!("{}", p.port_name );}

    println!("Serial-port:");

    let mut port_:String = String::new();

    io::stdin().read_line(&mut port_).expect("Failed to read line");

    const SPEED_: u32 = 115_200;
    let port_2 = port_.clone();


    thread::spawn(move ||{
        let serial_inf = Inf {port:port_2.trim().to_string(),speed:SPEED_,};
        sread(serial_inf);

    });


    
    HttpServer::new(move ||{
        let serial_inf = Inf {port:port_.trim().to_string(),speed:SPEED_,};
        App::new()
        .data(serial_inf)
        .service(input_index)
        .service(senser_index)
    })
    .bind("0.0.0.0:8080")?
    .run()
    .await
}