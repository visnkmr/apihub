use std::env;

use crate::getcommits::get_recent_commits;
use dotenv::dotenv;

// #[test]
pub fn commitstojson() {
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
    
    // // println!("codeberg+gitea={:?}",gtr.len());

    gtr.sort_by(|a, b|{
        b.time.cmp(&a.time)
    });

    let mut gtrl=r1.1;
    gtrl.extend(r2.1);
    
    println!("codeberg+gitea={:?}",gtr.len());

    gtrl.sort_by(|a, b|{
        b.time.cmp(&a.time)
    });
    // // // for i in &gtr{
    // // //     println!("{}",i.time)
    // // // }

    prefstore::save_else_where("./gtr.json",serde_json::to_string(&gtr).unwrap());
    prefstore::save_else_where("./gtrl.json",serde_json::to_string(&gtrl).unwrap());
    // prefstore::appendcustom("gtr","gtr.json",serde_json::to_string(&gtr1).unwrap());
}
