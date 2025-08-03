use std::{any::TypeId, env, fs};
mod stats;
mod repochanges;
use chrono::{DateTime, FixedOffset, NaiveDateTime, Utc};
// use commitstojson::commitstojson;
// use pscale::*;
use rayon::prelude::{IntoParallelRefIterator, ParallelIterator};
use reqwest::{blocking::Client, header::{HeaderMap, AUTHORIZATION, CONTENT_TYPE, USER_AGENT}};
use serde::{Serialize, Deserialize, Deserializer};
use serde_json::{Value, json};
use dotenv::dotenv;
mod getrepolist;
mod getcommits;
mod commitstruct;
mod reponames;
mod commitstojson;
// mod pscale;
// mod acenter;
// #[test]
mod pscale;
mod acenter;

//create a struct based on content of sample.json

#[derive(Serialize, Deserialize, Debug)]
struct Issue {
    url: String,
    repository_url: String,
    labels_url: String,
    comments_url: String,
    events_url: String,
    html_url: String,
    id: u64,
    node_id: String,
    number: u32,
    title: String,
    user: User,
    labels: Vec<::serde_json::Value>, // Simplified since it's empty
    state: String,
    locked: bool,
    assignee: Option<::serde_json::Value>,
    assignees: Vec<::serde_json::Value>,
    milestone: Option<::serde_json::Value>,
    comments: u32,
    created_at: String,
    updated_at: String,
    closed_at: Option<String>,
    author_association: String,
    active_lock_reason: Option<::serde_json::Value>,
    sub_issues_summary: SubIssuesSummary,
    body: String,
    closed_by: Option<::serde_json::Value>,
    reactions: Reactions,
    timeline_url: String,
    performed_via_github_app: Option<::serde_json::Value>,
    state_reason: Option<::serde_json::Value>,
}

#[derive(Serialize, Deserialize, Debug)]
struct User {
    login: String,
    id: u64,
    node_id: String,
    avatar_url: String,
    gravatar_id: String,
    url: String,
    html_url: String,
    followers_url: String,
    following_url: String,
    gists_url: String,
    starred_url: String,
    subscriptions_url: String,
    organizations_url: String,
    repos_url: String,
    events_url: String,
    received_events_url: String,
    #[serde(rename = "type")]
    user_type: String,
    user_view_type: String,
    site_admin: bool,
}

#[derive(Serialize, Deserialize, Debug)]
struct SubIssuesSummary {
    total: u32,
    completed: u32,
    percent_completed: u32,
}

#[derive(Serialize, Deserialize, Debug)]
struct Reactions {
    url: String,
    total_count: u32,
    #[serde(rename = "+1")]
    plus_one: u32,
    #[serde(rename = "-1")]
    minus_one: u32,
    laugh: u32,
    hooray: u32,
    confused: u32,
    heart: u32,
    rocket: u32,
    eyes: u32,
}

#[derive(Deserialize)]
struct Owner {
    login: String,
    id: i64,
    node_id: String,
    avatar_url: String,
    gravatar_id: String,
    url: String,
    html_url: String,
    followers_url: String,
    following_url: String,
    gists_url: String,
    starred_url: String,
    subscriptions_url: String,
    organizations_url: String,
    repos_url: String,
    events_url: String,
    received_events_url: String,
    #[serde(rename = "type")]
    owner_type: String,
    user_view_type: String,
    site_admin: bool,
}

#[derive(Deserialize)]
struct License {
    key: String,
    name: String,
    spdx_id: String,
    url: Option<String>,
    node_id: String,
}

#[derive(Deserialize)]
struct Repository {
    id: i64,
    node_id: String,
    name: String,
    full_name: String,
    private: bool,
    owner: Owner,
    html_url: String,
    description: Option<String>,
    fork: bool,
    url: String,
    forks_url: String,
    keys_url: String,
    collaborators_url: String,
    teams_url: String,
    hooks_url: String,
    issue_events_url: String,
    events_url: String,
    assignees_url: String,
    branches_url: String,
    tags_url: String,
    blobs_url: String,
    git_tags_url: String,
    git_refs_url: String,
    trees_url: String,
    statuses_url: String,
    languages_url: String,
    stargazers_url: String,
    contributors_url: String,
    subscribers_url: String,
    subscription_url: String,
    commits_url: String,
    git_commits_url: String,
    comments_url: String,
    issue_comment_url: String,
    contents_url: String,
    compare_url: String,
    merges_url: String,
    archive_url: String,
    downloads_url: String,
    issues_url: String,
    pulls_url: String,
    milestones_url: String,
    notifications_url: String,
    labels_url: String,
    releases_url: String,
    deployments_url: String,
    created_at: String,
    updated_at: String,
    pushed_at: String,
    git_url: String,
    ssh_url: String,
    clone_url: String,
    svn_url: String,
    homepage: Option<String>,
    size: i32,
    stargazers_count: i32,
    watchers_count: i32,
    language: Option<String>,
    has_issues: bool,
    has_projects: bool,
    has_downloads: bool,
    has_wiki: bool,
    has_pages: bool,
    has_discussions: bool,
    forks_count: i32,
    mirror_url: Option<String>,
    archived: bool,
    disabled: bool,
    open_issues_count: i32,
    license: Option<License>,
    allow_forking: bool,
    is_template: bool,
    web_commit_signoff_required: bool,
    topics: Vec<String>,
    visibility: String,
    forks: i32,
    open_issues: i32,
    watchers: i32,
    default_branch: String,
}
// #[test]
fn getissuescount(){
    // Read the JSON file
    let json_data = fs::read_to_string("sample.json").unwrap();
    
   
    let mut repos: Vec<Repository> = serde_json::from_str(&json_data).unwrap();
    //sort by no of open issues
    repos.sort_by(|a, b|{
        b.open_issues_count.cmp(&a.open_issues_count)
    });
     // Parse the JSON data into a vector of Repository structs
     for repo in repos.iter().clone() {
         println!("{}================== {}", repo.name, repo.open_issues_count);
                 
     }
    //collect repo names of top 7 repos with most issues
    let mut top_repos = repos.iter().clone().take(7).map(|repo| repo.name.clone()).collect::<Vec<String>>();
    top_repos.push("visnkmr".to_string());
    println!("{:?}",top_repos);
    // Print the open issues count for each repository
    let client = Client::new();
    let mut headers = HeaderMap::new();
    headers.insert(CONTENT_TYPE, "application/json".parse().unwrap());
    headers.insert(USER_AGENT, "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/58.0.3029.110 Safari/537.36".parse().unwrap());
    // headers.insert(AUTHORIZATION, format!("token {}", env::var("GITHUB_TOKEN").unwrap()).parse().unwrap());

    // let mut issues=vec![];
    let mut issues_count_per_repo = Vec::new();
    let mut issues_per_repo = Vec::new();

    //fetch issues per repo using the github api @ url https://api.github.com/repos/{owner}/{repo}/issues
    for repo in top_repos.iter().clone() {
        let url = format!("https://api.github.com/repos/visnkmr/{}/issues", repo);
        println!("{}================== {}", repo, url);
        let response = client.get(&url).headers(headers.clone()).send().unwrap();
        // println!("{}================== {}", repo, response.status());
let issues = match response.json::<Vec<Issue>>() {
            Ok(issues) => issues,
            Err(e) => {
                println!("Failed to parse JSON for repo {}: {}", repo, e);
                // println!("Response status: {}", response.status());
                continue;
            }
        };
        issues_count_per_repo.push((repo, issues.iter().clone().take(1).map(|issue| issue.number.clone()).collect::<Vec<u32>>()));
        issues_per_repo.push((repo, issues));
    }
    //get last 5 issues from each repo
    for issues in issues_per_repo.iter().clone() {
            for eachissue in issues.1.iter().clone().take(5) {
            println!("{}================== {}", eachissue.title, eachissue.html_url);
        }
    }
    println!("{:?}",issues_per_repo);
    // Save issues count per repo as JSON
    let mut issues_count_json: Vec<serde_json::Value> = issues_count_per_repo.iter().map(|(repo_name, issue_numbers,)| {
        json!({
            "repo_name":  repos.iter().find(|r|r.name==**repo_name).map(|r| format!("{}",r.full_name )).unwrap_or_default(),
            "issue_count": issue_numbers.first().unwrap_or(&0),
            "html_url": repos.iter().find(|r|r.name==**repo_name).map(|r| format!("{}",r.html_url )).unwrap_or_default()
        })
    }).collect();
    issues_count_json.push(
        json!({
            "repo_name": "visnkmr/visnkmr",
            "issue_count": issues_count_per_repo.iter().map(|(_, issue_numbers)| issue_numbers.first().unwrap_or(&0)).sum::<u32>(),
            "html_url": "https://github.com/visnkmr/visnkmr"
        })
    );
    
    fs::write("issues_count.json", serde_json::to_string_pretty(&issues_count_json).unwrap()).unwrap();
    
    // Save top 5 issues per repo as JSON
    let top_issues_json: Vec<serde_json::Value> = issues_per_repo.iter().map(|(repo_name, issues)| {
        let top_5_issues: Vec<serde_json::Value> = issues.iter().take(5).map(|issue| {
            json!({
                "title": issue.title,
                "comment_count": issue.comments,
                "url": issue.html_url
            })
        }).collect();
        
        json!({
            "issues": top_5_issues,
"repo_name":  repos.iter().find(|r|r.name==**repo_name).map(|r| r.full_name.clone()).unwrap_or_else(|| repo_name.to_string()),
"html_url": repos.iter().find(|r|r.name==**repo_name).map(|r| format!("{}",r.html_url )).unwrap_or_else(|| format!("https://github.com/visnkmr/{}", repo_name))
        })
    }).collect();
    
    fs::write("top_issues.json", serde_json::to_string_pretty(&top_issues_json).unwrap()).unwrap();


    
}


//the codeberg and gitea server stats getting api




use crate::{getrepolist::*, commitstruct::*};
// #[tokio::main]
// async
 fn main(){

    dotenv().ok();
    getissuescount();
    commitstojson::commitstojson();
    // let today = Utc::now();
    // let date_28_days_ago = &(today - chrono::Duration::days(27)).format("%Y-%m-%d").to_string();
    // let date_yesterday = &(today - chrono::Duration::days(1)).format("%Y-%m-%d").to_string();
    // let date_today = &(today ).format("%Y-%m-%d").to_string();
    // // commitstojson::commitstojson();
    // // println!("{:?}",TypeId::of::<sessioncount>());

    // // //add commits to json.
    // // commitstojson();

    // // adding session count per day from appcenter to planetscale.
    // let vecssc:Vec<sessioncount>=appcentervecapi("session_counts",&date_28_days_ago,&date_yesterday).await?;
    // addtosessiondb(vecssc);

    // for i in 1..27{  
    //     let datetofetch=&(today - chrono::Duration::days(i)).format("%Y-%m-%d").to_string();
    //     let (vecsevents)=eventsapi("events",&datetofetch,&datetofetch).await?;
    //     // println!("{:?}---{}",serde_json::to_string(&vecsevents).unwrap(),serde_json::to_string(&vecsevents).unwrap().len());
    //     addtoeventdb(&datetofetch,vecsevents);
    // }

    // //adding os versions per day from appcenter to planetscale.
    // for i in 1..27{
    //     println!("checking {} day before",i);
    //     let datetofetch=&(today - chrono::Duration::days(i)).format("%Y-%m-%d").to_string();
    //     let vecstoadd=osapi("oses",&datetofetch,&datetofetch).await?;
    //     // println!("{}",serde_json::to_string(&vecstoadd.oses).unwrap().len());
    //     addtoosdb(datetofetch,vecstoadd);
    // }
    // println!("{:?}",vecstoadd);
    
    
    // Ok(())
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

