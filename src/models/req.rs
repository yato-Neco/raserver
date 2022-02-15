#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ReqJsonSenser {
    pub senser: usize,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ReqJsonInput {
    pub input: String,
}



#[warn(dead_code)]
pub struct Input <'a> {
    pub w:&'a str,
    pub a:&'a str,
    pub d:&'a str,
    pub s:&'a str,
}



pub struct Inf {
    pub port:String,
    pub speed:u32,
}