use dotenvy::dotenv;
use rust_cfbd::client::CfbdClient;
use rust_cfbd::config::Config;
use rust_cfbd::models::cfb::game_media_params::GameMediaParams;
use std::env;

#[tokio::main]
async fn main() {
    dotenv().ok();
    let params = GameMediaParams::new().year(2025).week(2).build();

    let api_key = env::var("CFBD_API_KEY").expect("CFBD_API_KEY not set");

    let client = CfbdClient::new(Config::new(api_key));

    let resp = client.get_game_media(&params).await.unwrap();

    for r in resp {
        println!("{:#?}", r);
    }
}
