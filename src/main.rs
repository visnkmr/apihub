use std::env;

use chrono::{DateTime, FixedOffset};
use reqwest::blocking::Client;
use serde_json::{Value, json};
use dotenv::dotenv;

fn main() {
    dotenv().ok();
    let api_key = env::var("API_KEY").unwrap();
    let server_url = env::var("URL").unwrap();

    let repolist=get_repos(&server_url, "12core1", &api_key);
    println!("{:?}",repolist.len());
    for i in get_recent_commits(&server_url, "12core1", &api_key){
        println!("{}",i.time)
    }
}
fn get_repos(server_url: &str, user: &str, access_token: &str) -> Vec<String> {
    let client = Client::new();

    let url = format!("{}/api/v1/users/{}/repos?access_token={}", server_url, user, access_token);
    // println!("{}",url);
    let responsept = client.get(&url).send().unwrap();
    // println!("{:?}",responsept);
    let response = responsept.json::<Vec<Value>>().unwrap();
    response.into_iter().map(|repo| repo["full_name"].as_str().unwrap().to_string()).collect()
}
struct commits{
    additions:u64,
    deletions:u64,
    total:u64,
    message:String,
    time:DateTime<FixedOffset>,
    committer:String,
    commit:String,
}
fn get_commits(server_url: &str, repo: &str, access_token: &str) -> Vec<commits> {
    let client = Client::new();
    let url = format!("{}/api/v1/repos/{}/commits?access_token={}", server_url, repo, access_token);
    // println!("{:?}",url);
    //TODO: skip empty repo
    let mut y:Vec<commits>=Vec::new();
    match client.get(&url).send().unwrap().json::<Vec<Value>>(){
        Ok(response) => {
            y=response.into_iter().map(|commit|{
                commits{
                    
                    additions:json!(commit["stats"])["additions"].as_u64().unwrap(),
                    deletions:json!(commit["stats"])["deletions"].as_u64().unwrap(),
                    total:json!(commit["stats"])["total"].as_u64().unwrap(),
                    message:json!(commit["commit"])["message"].as_str().unwrap().to_string(),
                    time:{
                        // format!("{}", 
                        match&(commit["created"]){
                            Value::String(date_string) => {
                                let date_time = DateTime::parse_from_str(&date_string, "%Y-%m-%dT%H:%M:%S%z")
                                    .unwrap()
                                    .with_timezone(&FixedOffset::east_opt(5*3600+30*60).unwrap());
                                  date_time
                            },
                            _ => {
                                DateTime::default()
                            },
                        }
                        
                        // )
                    },
                    committer:format!("{}",json!(commit["committer"])["username"]),
                    commit:commit["html_url"].as_str().unwrap().to_string(),
                }
        
        
                // print_key_value_pairs(&commit);
                // "".to_string()
            }
            )
            // .take(2)
            .collect();
        },
        Err(err) => {
            eprintln!("Failed to deserialize JSON response: {}", err);
        },
    }
    y

}
fn print_key_value_pairs(value: &Value) {
    if let Some(object) = value.as_object() {
        for (key, value) in object.iter() {
            println!("{}: {}", key, value);
        }
    }
}
fn get_recent_commits(server_url: &str, user: &str, access_token: &str) -> Vec<commits> {
    let repos = get_repos(server_url, user, access_token);
    let mut commits = Vec::new();
    for repo in repos {
        commits.extend(get_commits(server_url, &repo, access_token));
    }
    commits.sort_by(|a, b| b.time.cmp(&a.time));
    commits.into_iter().collect()
}
