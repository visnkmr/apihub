use std::{env, any::TypeId};

use chrono::Utc;
use reqwest::Response;

use crate::commitstruct::{sessioncount, osl, oses, eventslist, eachevent};
pub async fn acinit(whattofetch: &str,date_start:&str,date_end:&str)->Response{
    let ac_key = env::var("APPCENTER_KEY").unwrap();
    let ac_uname = env::var("APPCENTER_UNAME").unwrap();
    let ac_appname = env::var("APPCENTER_APPNAME").unwrap();

    
    

    // println!("{}",date_28_days_ago);
    let get_apps_url = format!("https://api.appcenter.ms/v0.1/apps/{}/{}/analytics/{}?start={}&end={}&%24top=0",ac_uname,ac_appname,whattofetch,date_start,date_end);
    // let get_apps_url = format!("https://api.appcenter.ms/v0.1/apps/{}/{}/analytics/{}?start={}&end={}&%24top=0",ac_uname,ac_appname,whattofetch,date_28_days_ago,date_today);
    let client = reqwest::Client::new();
    let response = client
        .get(get_apps_url)
        .header("accept", "application/json")
        .header("X-API-Token", ac_key)
        .send()
        .await; 
    // print!("{:?}",response.unwrap().text().await);

    response.unwrap()
}
pub async fn appcentervecapi<T: for<'a> serde::Deserialize<'a>+ 'static>(whattofetch: &str,date_start:&str,date_end:&str) -> Result<Vec<T>, Box<dyn std::error::Error>> {
    println!("{:?}",TypeId::of::<T>());
     
    // let search_results: Vec<T> = vec![];
    let search_results: Vec<T> = acinit(whattofetch,date_start,date_end).await.json().await?;
    // println!("{:?}",search_results);
    // for eacha in search_results{

    //         println!("{:?}", eacha.count);
    //     }
        Ok(search_results)
}

pub async fn osapi(whattofetch: &str,date_start:&str,date_end:&str) -> Result<osl, Box<dyn std::error::Error>> {
    // let search_results: Vec<T> = vec![];
    let search_results: osl = acinit(whattofetch,date_start,date_end).await.json().await?;
    print!("{:?}",acinit(whattofetch,date_start,date_end).await.text().await);

    // println!("{:?}",search_results);
    // println!("{:?}",search_results.total);
    // for eacha in search_results{

    //         println!("{:?}", eacha.count);
    //     }
        Ok(search_results)
}

pub async fn eventsapi(whattofetch: &str,date_start:&str,date_end:&str) -> Result<Vec<eachevent>, Box<dyn std::error::Error>> {
    // let search_results: Vec<T> = vec![];
    let search_results: eventslist = acinit(whattofetch,date_start,date_end).await.json().await?;
    // println!("{:?}",search_results);
    // for eacha in search_results{

    //         println!("{:?}", eacha.count);
    //     }
        Ok(search_results.events)
}