use std::env;

use serde::*;

use crate::getrepolist::get_repos;

#[derive(Deserialize, Debug)]
struct App {
    id: String,
    app_secret: String,
    description: Option<String>,
    display_name: String,
    name: String,
    os: String,
    platform: String,
    origin: String,
    icon_url: Option<String>,
    created_at: String,
    updated_at: String,
    release_type: Option<String>,
    owner: Owner,
    azure_subscription: Option<String>,
    member_permissions: Vec<String>,
}

#[derive(Deserialize, Debug)]
struct Owner {
    id: String,
    avatar_url: Option<String>,
    display_name: String,
    email: String,
    name: String,
    r#type: String,
}


pub fn fetchnsavereponames(){

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