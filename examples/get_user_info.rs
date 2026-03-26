use dotenvy::dotenv;
use rust_cfbd::client::CfbdClient;
use rust_cfbd::config::Config;
use rust_cfbd::models::shared::entity::info::UserInfo;
use std::env;

#[tokio::main]
async fn main() {
    dotenv().ok();

    let api_key = env::var("CFBD_API_KEY").expect("CFBD_API_KEY not set");

    let client = CfbdClient::new(Config::new(api_key));

    let resp: UserInfo = client.get_user_info().await.unwrap();
    println!("{}", resp);
}
