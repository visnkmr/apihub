use std::env;

use chrono::Utc;
use reqwest::Response;

use crate::commitstruct::{sessioncount, osl, oses, eventslist, eachevent};
pub async fn acinit(whattofetch: &str)->Response{
    let ac_key = env::var("APPCENTER_KEY").unwrap();
    let ac_uname = env::var("APPCENTER_UNAME").unwrap();
    let ac_appname = env::var("APPCENTER_APPNAME").unwrap();

    
    let today = Utc::now();
    let date_28_days_ago = (today - chrono::Duration::days(27)).format("%Y-%m-%d").to_string();

    println!("{}",date_28_days_ago);
    let get_apps_url = format!("https://api.appcenter.ms/v0.1/apps/{}/{}/analytics/{}?start={}",ac_uname,ac_appname,whattofetch,date_28_days_ago);
    let client = reqwest::Client::new();
    let response = client
        .get(get_apps_url)
        .header("accept", "application/json")
        .header("X-API-Token", ac_key)
        .send()
        .await; 
    // print!("{:?}",response.text().await);

    response.unwrap()
}
pub async fn appcentervecapi<T: for<'a> serde::Deserialize<'a>>(whattofetch: &str) -> Result<Vec<T>, Box<dyn std::error::Error>> {
     
    // let search_results: Vec<T> = vec![];
    let search_results: Vec<T> = acinit(whattofetch).await.json().await?;
    // println!("{:?}",search_results);
    // for eacha in search_results{

    //         println!("{:?}", eacha.count);
    //     }
        Ok(search_results)
}

pub async fn osapi(whattofetch: &str) -> Result<Vec<oses>, Box<dyn std::error::Error>> {
    // let search_results: Vec<T> = vec![];
    let search_results: osl = acinit(whattofetch).await.json().await?;
    // println!("{:?}",search_results);
    // for eacha in search_results{

    //         println!("{:?}", eacha.count);
    //     }
        Ok(search_results.oses)
}

pub async fn eventsapi(whattofetch: &str) -> Result<Vec<eachevent>, Box<dyn std::error::Error>> {
    // let search_results: Vec<T> = vec![];
    let search_results: eventslist = acinit(whattofetch).await.json().await?;
    // println!("{:?}",search_results);
    // for eacha in search_results{

    //         println!("{:?}", eacha.count);
    //     }
        Ok(search_results.events)
}