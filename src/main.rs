// use chrono::{TimeZone, Utc};
use clap::{crate_version, App, Arg, ArgMatches};
// use std::time::{Duration, UNIX_EPOCH};
use textplots::{utils, Chart, Plot, Shape};
use yahoo_finance_api as yahoo;

#[tokio::main]
async fn main() {
    let matches = arguments();
    let ticker = matches.value_of("ticker").unwrap();
    let provider = yahoo::YahooConnector::new();

    let quotes = provider.get_latest_quotes(ticker, "1h").await.unwrap();
    let last_quote = quotes.last_quote().unwrap();
    println!("{}: ${:.2?}", ticker.to_uppercase(), last_quote.close);

    let response = provider.get_quote_range(ticker, "1d", "1mo").await.unwrap();
    let quotes = response.quotes().unwrap();
    let points: Vec<(f32, f32)> = quotes
        .iter()
        .enumerate()
        .map(|(index, quote)| (index as f32 + 1.0, quote.close as f32))
        .collect();
    Chart::new(180, 60, 0.0, 30.0)
        .lineplot(&Shape::Lines(&points))
        .display();
}

fn arguments<'a>() -> ArgMatches<'a> {
    App::new("bid")
        .version(crate_version!())
        .author("Noah Masur <noahmasur@gmail.com>")
        .about("Retrieve stock quotes")
        .arg(
            Arg::with_name("ticker")
                .required(true)
                .value_name("TICKER")
                .help("Specify a ticker"),
        )
        .get_matches()
}
