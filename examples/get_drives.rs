use dotenvy::dotenv;
use rust_cfbd::client::CfbdClient;
use rust_cfbd::config::Config;
use rust_cfbd::models::cfb::query::drive_query::DriveQueryBuilder;
use std::env;

#[tokio::main]
async fn main() {
    dotenv().ok();
    let params = DriveQueryBuilder::new()
        .year(2024)
        .week(1)
        .team(String::from("Ohio State"))
        .build();

    let api_key = env::var("CFBD_API_KEY").expect("CFBD_API_KEY not set");

    let client = CfbdClient::new(Config::new(api_key));

    let resp = client.get_drives(&params).await.unwrap();

    for r in resp {
        println!("{:#?}", r);
    }
}
