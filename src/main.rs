use std::env;
mod stats;
mod repochanges;
use chrono::{DateTime, FixedOffset, NaiveDateTime, Utc};
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
    // commitstojson::commitstojson();
    let vecstoadd:Vec<sessioncount>=appcentervecapi("session_counts").await?;
    // let vecstoadd:Vec<eachevent>=eventsapi("events").await?;
    // let vecstoadd:Vec<oses>=osapi("oses").await?;
    // println!("{:?}",vecstoadd);
    planetscaleapi(
    vecstoadd
    );
    
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

