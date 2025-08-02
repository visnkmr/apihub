use std::{any::TypeId, env, fs};
mod stats;
mod repochanges;
use chrono::{DateTime, FixedOffset, NaiveDateTime, Utc};
// use commitstojson::commitstojson;
// use pscale::*;
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
// mod pscale;
// mod acenter;
// #[test]
mod pscale;
mod acenter;

//create a struct based on content of sample.json
// {
//     "id": 631697041,
//     "node_id": "R_kgDOJabukQ",
//     "name": ".github",
//     "full_name": "visnkmr/.github",
//     "private": false,
//     "owner": {
//         "login": "visnkmr",
//         "id": 12533753,
//         "node_id": "MDQ6VXNlcjEyNTMzNzUz",
//         "avatar_url": "https://avatars.githubusercontent.com/u/12533753?v=4",
//         "gravatar_id": "",
//         "url": "https://api.github.com/users/visnkmr",
//         "html_url": "https://github.com/visnkmr",
//         "followers_url": "https://api.github.com/users/visnkmr/followers",
//         "following_url": "https://api.github.com/users/visnkmr/following{/other_user}",
//         "gists_url": "https://api.github.com/users/visnkmr/gists{/gist_id}",
//         "starred_url": "https://api.github.com/users/visnkmr/starred{/owner}{/repo}",
//         "subscriptions_url": "https://api.github.com/users/visnkmr/subscriptions",
//         "organizations_url": "https://api.github.com/users/visnkmr/orgs",
//         "repos_url": "https://api.github.com/users/visnkmr/repos",
//         "events_url": "https://api.github.com/users/visnkmr/events{/privacy}",
//         "received_events_url": "https://api.github.com/users/visnkmr/received_events",
//         "type": "User",
//         "user_view_type": "public",
//         "site_admin": false
//     },
//     "html_url": "https://github.com/visnkmr/.github",
//     "description": null,
//     "fork": false,
//     "url": "https://api.github.com/repos/visnkmr/.github",
//     "forks_url": "https://api.github.com/repos/visnkmr/.github/forks",
//     "keys_url": "https://api.github.com/repos/visnkmr/.github/keys{/key_id}",
//     "collaborators_url": "https://api.github.com/repos/visnkmr/.github/collaborators{/collaborator}",
//     "teams_url": "https://api.github.com/repos/visnkmr/.github/teams",
//     "hooks_url": "https://api.github.com/repos/visnkmr/.github/hooks",
//     "issue_events_url": "https://api.github.com/repos/visnkmr/.github/issues/events{/number}",
//     "events_url": "https://api.github.com/repos/visnkmr/.github/events",
//     "assignees_url": "https://api.github.com/repos/visnkmr/.github/assignees{/user}",
//     "branches_url": "https://api.github.com/repos/visnkmr/.github/branches{/branch}",
//     "tags_url": "https://api.github.com/repos/visnkmr/.github/tags",
//     "blobs_url": "https://api.github.com/repos/visnkmr/.github/git/blobs{/sha}",
//     "git_tags_url": "https://api.github.com/repos/visnkmr/.github/git/tags{/sha}",
//     "git_refs_url": "https://api.github.com/repos/visnkmr/.github/git/refs{/sha}",
//     "trees_url": "https://api.github.com/repos/visnkmr/.github/git/trees{/sha}",
//     "statuses_url": "https://api.github.com/repos/visnkmr/.github/statuses/{sha}",
//     "languages_url": "https://api.github.com/repos/visnkmr/.github/languages",
//     "stargazers_url": "https://api.github.com/repos/visnkmr/.github/stargazers",
//     "contributors_url": "https://api.github.com/repos/visnkmr/.github/contributors",
//     "subscribers_url": "https://api.github.com/repos/visnkmr/.github/subscribers",
//     "subscription_url": "https://api.github.com/repos/visnkmr/.github/subscription",
//     "commits_url": "https://api.github.com/repos/visnkmr/.github/commits{/sha}",
//     "git_commits_url": "https://api.github.com/repos/visnkmr/.github/git/commits{/sha}",
//     "comments_url": "https://api.github.com/repos/visnkmr/.github/comments{/number}",
//     "issue_comment_url": "https://api.github.com/repos/visnkmr/.github/issues/comments{/number}",
//     "contents_url": "https://api.github.com/repos/visnkmr/.github/contents/{+path}",
//     "compare_url": "https://api.github.com/repos/visnkmr/.github/compare/{base}...{head}",
//     "merges_url": "https://api.github.com/repos/visnkmr/.github/merges",
//     "archive_url": "https://api.github.com/repos/visnkmr/.github/{archive_format}{/ref}",
//     "downloads_url": "https://api.github.com/repos/visnkmr/.github/downloads",
//     "issues_url": "https://api.github.com/repos/visnkmr/.github/issues{/number}",
//     "pulls_url": "https://api.github.com/repos/visnkmr/.github/pulls{/number}",
//     "milestones_url": "https://api.github.com/repos/visnkmr/.github/milestones{/number}",
//     "notifications_url": "https://api.github.com/repos/visnkmr/.github/notifications{?since,all,participating}",
//     "labels_url": "https://api.github.com/repos/visnkmr/.github/labels{/name}",
//     "releases_url": "https://api.github.com/repos/visnkmr/.github/releases{/id}",
//     "deployments_url": "https://api.github.com/repos/visnkmr/.github/deployments",
//     "created_at": "2023-04-23T20:34:48Z",
//     "updated_at": "2023-04-23T20:34:49Z",
//     "pushed_at": "2023-12-23T06:06:52Z",
//     "git_url": "git://github.com/visnkmr/.github.git",
//     "ssh_url": "git@github.com:visnkmr/.github.git",
//     "clone_url": "https://github.com/visnkmr/.github.git",
//     "svn_url": "https://github.com/visnkmr/.github",
//     "homepage": null,
//     "size": 16,
//     "stargazers_count": 0,
//     "watchers_count": 0,
//     "language": null,
//     "has_issues": true,
//     "has_projects": true,
//     "has_downloads": true,
//     "has_wiki": true,
//     "has_pages": false,
//     "has_discussions": false,
//     "forks_count": 0,
//     "mirror_url": null,
//     "archived": false,
//     "disabled": false,
//     "open_issues_count": 1,
//     "license": {
//         "key": "mit",
//         "name": "MIT License",
//         "spdx_id": "MIT",
//         "url": "https://api.github.com/licenses/mit",
//         "node_id": "MDc6TGljZW5zZTEz"
//     },
//     "allow_forking": true,
//     "is_template": false,
//     "web_commit_signoff_required": false,
//     "topics": [],
//     "visibility": "public",
//     "forks": 0,
//     "open_issues": 1,
//     "watchers": 0,
//     "default_branch": "main"
// },
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
    
    // Parse the JSON data into a vector of Repository structs
    let mut repos: Vec<Repository> = serde_json::from_str(&json_data).unwrap();
    //sort by no of open issues
    repos.sort_by(|a, b|{
        b.open_issues_count.cmp(&a.open_issues_count)
    });
    //collect repo names of top 7 repos with most issues
    let top_repos = repos.iter().take(7).map(|repo| repo.name.clone()).collect::<Vec<String>>();
    println!("{:?}",top_repos);
    // Print the open issues count for each repository
    // for repo in repos {
    //     println!("{}================== {}", repo.name, repo.open_issues_count);
                
    // }
}


//the codeberg and gitea server stats getting api




use crate::{getrepolist::*, commitstruct::*};
// #[tokio::main]
// async
 fn main(){

    dotenv().ok();
    getissuescount();
    // commitstojson::commitstojson();
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

