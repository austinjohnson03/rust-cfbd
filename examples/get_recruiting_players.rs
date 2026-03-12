use dotenvy::dotenv;
use rust_cfbd::client::CfbdClient;
use rust_cfbd::config::Config;
use rust_cfbd::models::cfb::recruit_player_query::RecruitPlayerQueryBuilder;
use std::env;

#[tokio::main]
async fn main() {
    dotenv().ok();
    let params = RecruitPlayerQueryBuilder::new().year(2024).build();

    let api_key = env::var("CFBD_API_KEY").expect("CFBD_API_KEY not set");

    let client = CfbdClient::new(Config::new(api_key));

    let resp = client.get_recruiting_players(&params).await.unwrap();

    for r in resp {
        println!("{:#?}", r);
    }
}
