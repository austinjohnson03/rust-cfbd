use dotenvy::dotenv;
use rust_cfbd::client::CfbdClient;
use rust_cfbd::config::Config;
use rust_cfbd::models::cfb::stats_player_season_query::StatsPlayerSeasonQueryBuilder;
use std::env;

#[tokio::main]
async fn main() {
    dotenv().ok();
    let params = StatsPlayerSeasonQueryBuilder::new()
        .year(2024)
        .team("Alabama")
        .build();

    let api_key = env::var("CFBD_API_KEY").expect("CFBD_API_KEY not set");

    let client = CfbdClient::new(Config::new(api_key));

    let resp = client.get_player_season_stats(&params).await.unwrap();

    for r in resp {
        println!("{:#?}", r);
    }
}
