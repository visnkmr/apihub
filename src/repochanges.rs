use serde::*;

use crate::{getrepolist::get_repos, getcommits::get_commits};

#[derive(Serialize, Deserialize, Default, Debug, Clone)]
pub struct codechanges{
    pub reponame:String,
    pub additions:u64,
    pub deletions:u64,
    // total:u64,
}
pub fn changesincodeperrepo(server_url: &str, user: &str, access_token: &str) -> (Vec<codechanges>) {
    let repos = get_repos(server_url, user, access_token);
    let mut codec = Vec::new();
    let iop:()=repos.iter().map(|repo| {
        let mut additions=0;
        let mut deletions=0;
        match(get_commits(server_url, &repo, access_token)){
            Ok(getclist) => {
                additions=getclist.0.iter().map(|ic|{
                    ic.additions
                }).sum();
                deletions=getclist.0.iter().map(|ic|{
                    ic.deletions
                }).sum();
                codec.push(codechanges{
                    reponame: repo.clone(),
                    additions,
                    deletions,
                    // total: additions+deletions,
                })
                // commits.extend(getclist.0.);
                
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
    (codec)


}