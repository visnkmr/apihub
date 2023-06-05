use reqwest::{blocking::Client, header::{HeaderMap, CONTENT_TYPE, AUTHORIZATION}};
use serde_json::Value;
//gitea or codeberg api
pub fn get_repos(server_url: &str, user: &str, access_token: &str) -> Vec<String> {
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