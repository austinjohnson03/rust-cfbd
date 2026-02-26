use dotenvy::dotenv;
use rust_cfbd::client::CfbdClient;
use rust_cfbd::config::Config;
use std::env;

#[tokio::main]
async fn main() {
    dotenv().ok();

    let api_key = env::var("CFBD_API_KEY").expect("CFBD_API_KEY not set");

    let client = CfbdClient::new(Config::new(api_key));

    let resp = client.get_raw_json("info").await.unwrap();
    println!("{}", resp);
}
