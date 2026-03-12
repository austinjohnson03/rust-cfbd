use dotenvy::dotenv;
use rust_cfbd::client::CfbdClient;
use rust_cfbd::config::Config;
use rust_cfbd::models::cfb::stat_season_query::StatSeasonQueryBuilder;
use std::env;

#[tokio::main]
async fn main() {
    dotenv().ok();
    let params = StatSeasonQueryBuilder::new().year(2024).build();

    let api_key = env::var("CFBD_API_KEY").expect("CFBD_API_KEY not set");

    let client = CfbdClient::new(Config::new(api_key));

    let resp = client.get_season_stats(&params).await.unwrap();

    for r in resp {
        println!("{:#?}", r);
    }
}
