use dotenvy::dotenv;
use rust_cfbd::client::CfbdClient;
use rust_cfbd::config::Config;
use std::env;

#[tokio::main]
async fn main() {
    dotenv().ok();

    let api_key = env::var("CFBD_API_KEY").expect("CFBD_API_KEY not set");

    let client = CfbdClient::new(Config::new(api_key));

    match client.get_conferences().await {
        Ok(conferences) => {
            for conference in &conferences {
                println!("{:#?}", conference);
            }
        }
        Err(e) => eprintln!("Error: {:#?}", e),
    }
}
