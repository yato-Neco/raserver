#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ReqJsonSenser {
    pub senser: usize,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ReqJsonInput {
    pub input: String,
}


#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ReqJsonInputnew {
    pub power: usize,
    pub direction:isize,
    pub stop: usize,
}



#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ReqJsonInputnewad {
    pub power: usize,
    pub direction:isize,
    pub stop: usize,
    pub rotate: isize,
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