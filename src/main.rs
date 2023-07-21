use std::{env, any::TypeId};
mod stats;
mod repochanges;
use chrono::{DateTime, FixedOffset, NaiveDateTime, Utc};
// use commitstojson::commitstojson;
use pscale::*;
use rayon::prelude::{IntoParallelRefIterator, ParallelIterator};
use reqwest::{blocking::Client, header::{HeaderMap, CONTENT_TYPE, AUTHORIZATION}};
use serde::{Serialize, Deserialize, Deserializer};
use serde_json::{Value, json};
use dotenv::dotenv;
mod getrepolist;
mod getcommits;
mod commitstruct;
mod reponames;
mod commitstojson;
mod pscale;
mod acenter;
// #[test]
//the codeberg and gitea server stats getting api




use crate::{getrepolist::*, commitstruct::*, acenter::*};
#[tokio::main]
async fn main()-> Result<(), Box<dyn std::error::Error>>{

    dotenv().ok();
    // commitstojson();
    let today = Utc::now();
    let date_28_days_ago = &(today - chrono::Duration::days(27)).format("%Y-%m-%d").to_string();
    let date_yesterday = &(today - chrono::Duration::days(1)).format("%Y-%m-%d").to_string();
    let date_today = &(today ).format("%Y-%m-%d").to_string();
    // commitstojson::commitstojson();
    // println!("{:?}",TypeId::of::<sessioncount>());

    // //add commits to json.
    // commitstojson();

    // adding session count per day from appcenter to planetscale.
    let vecssc:Vec<sessioncount>=appcentervecapi("session_counts",&date_28_days_ago,&date_yesterday).await?;
    addtosessiondb(vecssc);

    for i in 1..27{  
        let datetofetch=&(today - chrono::Duration::days(i)).format("%Y-%m-%d").to_string();
        let (vecsevents)=eventsapi("events",&datetofetch,&datetofetch).await?;
        // println!("{:?}---{}",serde_json::to_string(&vecsevents).unwrap(),serde_json::to_string(&vecsevents).unwrap().len());
        addtoeventdb(&datetofetch,vecsevents);
    }

    //adding os versions per day from appcenter to planetscale.
    for i in 1..27{
        println!("checking {} day before",i);
        let datetofetch=&(today - chrono::Duration::days(i)).format("%Y-%m-%d").to_string();
        let vecstoadd=osapi("oses",&datetofetch,&datetofetch).await?;
        // println!("{}",serde_json::to_string(&vecstoadd.oses).unwrap().len());
        addtoosdb(datetofetch,vecstoadd);
    }
    // println!("{:?}",vecstoadd);
    
    
    Ok(())
}




//gitea codeberg commit get api and sort by timestamp




fn print_key_value_pairs(value: &Value) {
    if let Some(object) = value.as_object() {
        for (key, value) in object.iter() {
            println!("{}: {}", key, value);
        }
    }
}
#[test]
fn datetest(){
    let g=NaiveDateTime::parse_from_str("2023-05-12T15:01:34+05:30","%Y-%m-%dT%H:%M:%S%z")
                                    .unwrap();
                                
    // let g1=DateTime::parse_from_str("2022-12-06T18:31:45","%Y-%m-%dT%H:%M:%S")
    //                                 .unwrap();

    let ndt = NaiveDateTime::parse_from_str("2022-12-06T18:31:45Z", "%Y-%m-%dT%H:%M:%SZ").unwrap();

                                    // .with_timezone(&FixedOffset::east_opt(5*3600+30*60).unwrap());
}

