use serde::*;

#[derive(Serialize, Deserialize, Default, Debug, Clone)]
pub struct commits{
    pub reponame:String,
    pub additions:u64,
    pub deletions:u64,
    pub total:u64,
    pub message:String,
    pub time:i64,
    pub committer:String,
    pub commit:String,
}
#[derive(Deserialize, Debug)]   
pub struct sessioncount{
    pub datetime:String,
    pub count:i32
}
#[derive(Deserialize, Debug,Serialize)]   
pub struct osl{
    // pub date:String,
    pub total:i32,
    pub oses:Vec<oses>
}
#[derive(Deserialize, Debug,Serialize)]   
pub struct oses{
      pub os_name:String ,
      pub count: i32
}
#[derive(Deserialize, Debug)]   

pub struct eventslist{
    pub events:Vec<eachevent>,
    total:i32,
    total_devices:i32
}
#[derive(Deserialize, Debug)]   

pub struct eachevent{
    pub id: String,
    pub name: String,
    pub device_count: i32,
    pub previous_device_count: i32,
    pub count: i32,
    pub previous_count: i32,
    pub count_per_device: f32,
    pub count_per_session: Option<f32>,
}