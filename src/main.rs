use dotenv::dotenv;
use std::env;
pub mod banner;

#[tokio::main]
async fn main() {
    dotenv().ok();

    // User credentials.
    let username = env::var("TIDAL_USERNAME").unwrap();
    let password = env::var("TIDAL_PASSWORD").unwrap();
    let token = env::var("TIDAL_APP_TOKEN").unwrap();

    println!("{token}");
    println!("{username}");
    println!("{password}");
}
