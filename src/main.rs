use std::env;

use chrono::{DateTime, FixedOffset, NaiveDateTime};
use reqwest::{blocking::Client, header::{HeaderMap, CONTENT_TYPE, AUTHORIZATION}};
use serde::{Serialize, Deserialize, Deserializer};
use serde_json::{Value, json};
use dotenv::dotenv;
fn fetchnsavereponames(){

    let api_key = env::var("API_KEY").unwrap();
    let server_url = env::var("URL").unwrap();
    let un="12core1";

    let c_un="visnk";
    let c_api_key = env::var("CODEBERG").unwrap();
    let c_server_url = env::var("CODEBERG_URL").unwrap();

    let repolist=get_repos(&server_url, un, &api_key);
    println!("{:?}",repolist.len());

    let mut serializedData = String::new();

    for repo in repolist {
        serializedData=[serializedData,repo,"\n".to_string()].concat();
    }

    let repolist2=get_repos(&c_server_url, c_un, &c_api_key);
    println!("{:?}",repolist2.len());

    for repo in repolist2 {
        serializedData=[serializedData,repo,"\n".to_string()].concat();
    }

    prefstore::savecustom("gtr", "repos.txt", serializedData);
}
fn main() {
    dotenv().ok();
    let api_key = env::var("API_KEY").unwrap();
    let server_url = env::var("URL").unwrap();
    let un="12core1";
    let c_un="visnk";
    let c_api_key = env::var("CODEBERG").unwrap();
    let c_server_url = env::var("CODEBERG_URL").unwrap();
    
    

    // let repolist=get_repos(&server_url, un, &api_key);
    // println!("{:?}",repolist.len());

    let r1=get_recent_commits(&server_url, un, &api_key);
    let r2=get_recent_commits(&c_server_url, c_un, &c_api_key);
    let mut gtr=r1.0;
    println!("gitea={:?}",gtr.len());
    gtr.extend(r2.0);
    
    // println!("codeberg+gitea={:?}",gtr.len());

    gtr.sort_by(|a, b|{
        b.time.cmp(&a.time)
    });

    let mut gtrl=r1.1;
    gtrl.extend(r2.1);
    
    // println!("codeberg+gitea={:?}",gtr.len());

    gtrl.sort_by(|a, b|{
        b.time.cmp(&a.time)
    });
    // // for i in &gtr{
    // //     println!("{}",i.time)
    // // }

    prefstore::savecustom("gtr","gtr.json",serde_json::to_string(&gtr).unwrap());
    prefstore::savecustom("gtr","gtrl.json",serde_json::to_string(&gtrl).unwrap());
    // prefstore::appendcustom("gtr","gtr.json",serde_json::to_string(&gtr1).unwrap());
}
fn get_repos(server_url: &str, user: &str, access_token: &str) -> Vec<String> {
    let client = Client::new();

    let mut headers = HeaderMap::new();
    headers.insert(CONTENT_TYPE, "application/json".parse().unwrap());
    headers.insert(AUTHORIZATION, format!("{}", access_token).parse().unwrap());

    let url = format!("{}/api/v1/users/{}/repos?limit={}", server_url, user,50);
    // println!("{}",url);
    let responsept = 
    // client.get(&url).send().unwrap();
    client
        .get(url)
        .headers(headers)
        // .json(&serde_json::json!({
        //     // "limit": 50,
        //     // "page": 1,
        // }))
        .send().unwrap();
    // println!("{:?}",responsept);
    let response = responsept.json::<Vec<Value>>().unwrap();
    response.into_iter().map(|repo| repo["full_name"].as_str().unwrap().to_string()).collect()
}

#[derive(Serialize, Deserialize, Default, Debug, Clone)]

struct commits{
    reponame:String,
    additions:u64,
    deletions:u64,
    total:u64,
    message:String,
    time:i64,
    committer:String,
    commit:String,
}
fn get_commits(server_url: &str, repo: &str, access_token: &str) -> Result<(Vec<commits>,commits),()> {
    let client = Client::new();

    let mut headers = HeaderMap::new();
    headers.insert(CONTENT_TYPE, "application/json".parse().unwrap());
    headers.insert(AUTHORIZATION, format!("{}", access_token).parse().unwrap());

    let url = format!("{}/api/v1/repos/{}/commits?limit={}", server_url, repo,100);
    // println!("{}",url);
    let responsept = 
    // client.get(&url).send().unwrap();
    client
        .get(url)
        .headers(headers)
    //     .json(
    //     &serde_json::json!({
    //     // "limit": 50,
    //     // "page": 1,
    //     }
    //     )
    // )
        .send().unwrap();
    // println!("{:?}",url);
    //TODO: skip empty repo
    let mut y:Vec<commits>=Vec::new();
    match responsept.json::<Vec<Value>>(){
        Ok(response) => {
            y=response.into_iter().map(|commit|{
                // println!("{:?}",commit);
                commits{
                    reponame:repo.to_string(),
                    additions:json!(commit["stats"])["additions"].as_u64().unwrap(),
                    deletions:json!(commit["stats"])["deletions"].as_u64().unwrap(),
                    total:json!(commit["stats"])["total"].as_u64().unwrap(),
                    message:json!(commit["commit"])["message"].as_str().unwrap().to_string(),
                    time:{
                        // format!("{}", 
                        match&(commit["created"]){
                            Value::String(date_string) => {
                                // println!("{:?}",commit);
                                let date_time = 
                                if !date_string.ends_with("Z") {
                                    DateTime::parse_from_str(&date_string, "%Y-%m-%dT%H:%M:%S%z")
                                    .unwrap()
                                    .with_timezone(&FixedOffset::east_opt(5*3600+30*60).unwrap()).timestamp()}
                                else{
                                        NaiveDateTime::parse_from_str(&date_string, "%Y-%m-%dT%H:%M:%SZ")
                                    .unwrap().timestamp()
                                    };
                                  date_time
                                // date_string.clone()
                            },
                            _ => {
                                0
                                // String::new()
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
            // .take(1)
            .collect();
        Ok((y.clone(),y[0].clone()))

        },
        Err(err) => {
            eprintln!("Failed to deserialize JSON response: {}", err);
            Err(())
        },
    }

}
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
fn get_recent_commits(server_url: &str, user: &str, access_token: &str) -> (Vec<commits>,Vec<commits>) {
    let repos = get_repos(server_url, user, access_token);
    let mut commits = Vec::new();
    let mut lastcommits = Vec::new();
    let iop:()=repos.iter().map(|repo| {
        match(get_commits(server_url, &repo, access_token)){
            Ok(getclist) => {
                commits.extend(getclist.0);
                lastcommits.push(getclist.1);
                
            },
            Err(_) => {

            },
        }
    }).collect();
//     commits.sort_by(|a, b|{
        
//         a.time.cmp(&b.time)
//     }
// );
    // commits
    // .into_iter()
    // // .take(1)
    // .collect()
    (commits,lastcommits)
}

