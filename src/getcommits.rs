
use chrono::{DateTime, FixedOffset, NaiveDateTime};
use reqwest::{blocking::Client, header::{HeaderMap, CONTENT_TYPE, AUTHORIZATION}};
use serde_json::{Value, json};

use crate::{commitstruct::commits, getrepolist::get_repos};
//codeberg and gitea api
pub fn get_commits(server_url: &str, repo: &str, access_token: &str) -> Result<(Vec<commits>,commits),()> {
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
pub fn get_recent_commits(server_url: &str, user: &str, access_token: &str) -> (Vec<commits>,Vec<commits>) {
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
