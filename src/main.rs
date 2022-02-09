mod models;
use models::req::{ReqJson,Input};
use actix_web::{get,post,web,App,HttpResponse,HttpServer,Responder,HttpRequest};
extern crate serde;
#[macro_use]
extern crate serde_derive;
extern crate serde_json;



#[post("/")]
async fn post_index(data: web::Json<ReqJson>)->impl Responder{

    println!("{:?}",data);


    let uinput = Input{
        w:String::from("w"),
        a:String::from("a"),
        d:String::from("d"),
        s:String::from("s"),
    };


    let _w = uinput.w;
    let _a = uinput.a;
    let _d = uinput.d;
    let _s = uinput.s;


    match &data.input {
        _w => println!("one"),
        _a => println!("three"),
        _d => println!("five"),
        _s => println!("seven"),
        _ => println!("seven"),
    }
    

    HttpResponse::Ok().body("ok")
}



#[actix_web::main]
async fn main()->std::io::Result<()>{
    HttpServer::new(||{
        App::new()
        .service(post_index)
    })
    .bind("0.0.0.0:8080")?
    .run()
    .await
}