use dotenvy::dotenv;
use rust_cfbd::client::CfbdClient;
use rust_cfbd::config::Config;
use rust_cfbd::models::cfb::roster_query::RosterQueryBuilder;
use std::env;

#[tokio::main]
async fn main() {
    dotenv().ok();
    let params = RosterQueryBuilder::new()
        .year(2024)
        .team("Ohio State")
        .build();

    let api_key = env::var("CFBD_API_KEY").expect("CFBD_API_KEY not set");

    let client = CfbdClient::new(Config::new(api_key));

    let resp = client.get_roster(&params).await.unwrap();

    for r in resp {
        println!("{:#?}", r);
    }
}
