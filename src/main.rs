// use chrono::{TimeZone, Utc};
use clap::{crate_version, App, Arg, ArgMatches};
// use std::time::{Duration, UNIX_EPOCH};
use yahoo_finance_api as yahoo;

#[tokio::main]
async fn main() {
    let matches = arguments();
    let ticker = matches.value_of("ticker").unwrap();
    let provider = yahoo::YahooConnector::new();

    let quotes = provider.get_latest_quotes(ticker, "1h").await.unwrap();
    let last_quote = quotes.last_quote().unwrap();
    println!("{}: ${:.2?}", ticker.to_uppercase(), last_quote.close);

    // let range_of_quotes = provider.get_quote_range(ticker, "1d", "1mo").await.unwrap();
    // println!("{:?}", range_of_quotes);
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
