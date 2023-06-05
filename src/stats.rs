use std::env;
use crate::repochanges::{self, changesincodeperrepo};
use dotenv::dotenv;
fn completestats(){
    dotenv().ok();
    let api_key = env::var("API_KEY").unwrap();
    let server_url = env::var("URL").unwrap();
    let un="12core1";
    let c_un="visnk";
    let c_api_key = env::var("CODEBERG").unwrap();
    let c_server_url = env::var("CODEBERG_URL").unwrap();
    let mut r1=changesincodeperrepo(&server_url, un, &api_key);
    r1.extend(changesincodeperrepo(&c_server_url, c_un, &c_api_key));
    let mut taar=0;//total addition across repos
    let mut tdar=0;//total deletions across repos
    r1.sort_by(|a, b|{
        b.additions.cmp(&a.additions)
    });
    for o in r1{
        println!("{:?}",o);
        taar+=o.additions;
        tdar+=o.deletions;
    }
    println!("LOCs written: +{} -{}",taar,tdar);
}