
use std::env;

use mysql::{Pool,  QueryResult, prelude::Queryable, Params, Row, params, PooledConn};
use serde::*;

use crate::commitstruct::sessioncount;




pub fn getconn(url:String)->Pool{

    let builder = mysql::OptsBuilder::from_opts(mysql::Opts::from_url(&url).unwrap());

    let pool = mysql::Pool::new(builder.ssl_opts(mysql::SslOpts::default())).unwrap();

    
    pool
}
pub fn pscalewrite()->Pool{
    let url = env::var("DATAW").unwrap();
    getconn(url)
}
pub fn pscaleread()->Pool{
    let url = env::var("DATAR").unwrap();
    getconn(url)
}
pub fn planetscaleapi(datatoadd:Vec<sessioncount>){
    
    // println!("Successfully connected to Write to PlanetScale!");
    insertintodb(&pscalewrite(), &datatoadd);
        
   
    // println!("Successfully connected to Read from PlanetScale!");
    printdata(&pscaleread());

}
pub fn printdata(pool: &Pool)
-> Result<Vec<sessioncount>,()>
{
    let mut _conn = pool.get_conn().unwrap();
    let mut results = _conn .query_map(
        "SELECT * from ac_events",
        |(datetime,count)| {
            // let g:i32=id;
            sessioncount{datetime,count }
        },
    ).unwrap();
    for eacha in &results{

        println!("{:?}",eacha);
    }
    Ok(results)
}
fn addtodb(mut conn:&mut PooledConn,esc:&sessioncount)->Result<(),()>{
    let mut saved=false;
    if let Ok(res) = conn.exec(
        r"INSERT INTO ac_events (date,count)
          VALUES (?, ?)",
        (esc.datetime.clone(),esc.count.clone())
    ) {
        let vc:Vec<(String,i32)>=res;
        println!("new record");
        saved=true;
    }
    if !saved {

        println!("gone through");
    }
    Ok(())
}
pub fn insertintodb(pool: &Pool, sc:&Vec<sessioncount>) {
    // let payments = vec![
    //     sessioncount { datetime: "2023-05-07".to_string(), count: 0 },
    // ];
    let mut conn = pool.get_conn().unwrap();
    for esc in sc{
        addtodb(&mut conn,esc);
    }
    // conn.exec_batch(
    //     r"INSERT INTO ac_events (date,count)
    //       VALUES (:date, :count)",
    //     sc.iter().map(|p| params! {
    //         "date" => p.datetime.clone(),
    //         "count" => p.count,
    //     })
    // ).unwrap();
    
    // Ok(results)
}