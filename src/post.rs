use std::time::{Duration};
use std::io;
use std::io::Write;
use async_trait::async_trait;

#[derive(Debug, Clone)]
pub struct Inf {
    pub ip: String,
}

#[async_trait]
pub  trait PostReq {
    async fn req(&self,jn:&str);
    async fn req_senser(&self,jn:&str);
}

#[async_trait]
impl PostReq for Inf { 
    async fn req(&self,jn:&str) {
        let postjson = [("input", jn)];
        let client = reqwest::Client::new();
        let res = client.post("http://".to_owned() + self.ip.as_str()+"/input")
            .form(&postjson)
            .send()
            .await.unwrap();
    }

    async fn req_senser(&self,jn:&str) {
        let postjson = [("senser", jn)];
        let client = reqwest::Client::new();
        let res = client.post("http://".to_owned() + self.ip.as_str()+"/senser")
            .form(&postjson)
            .send()
            .await.unwrap();
    }


}