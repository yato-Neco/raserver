#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ReqJson {
    pub input: String,
    pub senser: usize,
}



pub struct Input {
    pub w:String,
    pub a:String,
    pub d:String,
    pub s:String,
}