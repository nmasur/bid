use chrono::{TimeZone, Utc};
use std::time::{Duration, UNIX_EPOCH};
use yahoo_finance_api as yahoo;

#[tokio::main]
async fn main() {
    let provider = yahoo::YahooConnector::new();
    let response = provider.get_quote_range("AAPL", "1d", "1mo").await.unwrap();
    let quotes = response.quotes().unwrap();
    println!("Apple's quotes of the last month: {:?}", quotes);
}
