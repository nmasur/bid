use anyhow::{anyhow, Context, Result};
use clap::{crate_version, App, Arg, ArgMatches};
use regex::Regex;
use std::str::FromStr;
use textplots::{Chart, Plot, Shape};
use yahoo_finance_api as yahoo;

#[tokio::main]
async fn main() -> Result<()> {
    let matches = arguments();
    let ticker = matches
        .value_of("ticker")
        .context("Must provide stock ticker as argument")?;
    let time = match matches.value_of("time") {
        Some(time) => parse_time(time.to_string()),
        None => Ok(TimeRange {
            scalar: 1,
            unit: TimeUnit::Day,
        }),
    }
    .context("Time range cannot be determined")?;
    let interval = calculate_interval(&time);

    let provider = yahoo::YahooConnector::new();
    let response = provider
        .get_quote_range(ticker, &interval.to_string(), &time.to_string())
        .await
        .context("Error from Yahoo Finance!")?; //TODO: better errors
    let last_quote = response.last_quote().context("No stock quote available!")?;
    println!("{}: ${:.2?}", ticker.to_uppercase(), last_quote.close);

    let quotes = response
        .quotes()
        .context("Quotes not found for this stock!")?;
    let points: Vec<(f32, f32)> = quotes
        .iter()
        .enumerate()
        .map(|(index, quote)| (index as f32 + 1.0, quote.close as f32))
        .collect();
    Chart::new(180, 60, 0.0, points.len() as f32)
        .lineplot(&Shape::Lines(&points))
        .display();

    Ok(())
}

#[derive(Debug, PartialEq, Copy, Clone)]
enum TimeUnit {
    Minute,
    Hour,
    Day,
    // Week,
    Month,
    Year,
}

impl FromStr for TimeUnit {
    type Err = anyhow::Error;
    fn from_str(s: &str) -> Result<Self> {
        match s {
            "h" => Ok(TimeUnit::Hour),
            "d" => Ok(TimeUnit::Day),
            // "w" => Ok(TimeUnit::Week),
            "m" => Ok(TimeUnit::Month),
            "y" => Ok(TimeUnit::Year),
            other => Err(anyhow!("{} is not valid time unit", other)),
        }
    }
}

#[derive(Debug, PartialEq)]
struct TimeRange {
    scalar: u64,
    unit: TimeUnit,
}

impl ToString for TimeRange {
    fn to_string(&self) -> String {
        format!(
            "{}{}",
            self.scalar,
            match self.unit {
                TimeUnit::Minute => "m",
                TimeUnit::Hour => "h",
                TimeUnit::Day => "d",
                // TimeUnit::Week => "w",
                TimeUnit::Month => "mo",
                TimeUnit::Year => "y",
            },
        )
    }
}

fn parse_time(input: String) -> Result<TimeRange> {
    let time_regex = Regex::new(r"^([1-9]\d*)*([hdwmy])$").context("Failed to compile regex")?;
    let captures = time_regex
        .captures(&input)
        .context("Not valid time range; try h, d, m, y")?;
    let scalar = match captures.get(1) {
        Some(regex_match) => regex_match
            .as_str()
            .parse::<u64>()
            .context("Not a number")?,
        None => 1 as u64,
    };
    // let scalar = captures
    //     .get(1)
    //     .or_else(|| Some(1))
    //     .context("Failed to parse")?
    //     .as_str()
    //     .parse::<u8>()
    //     .context("Not a number")?;
    let unit = captures
        .get(2)
        .context("Time range unit not provided")?
        .as_str()
        .parse::<TimeUnit>()?;
    Ok(TimeRange { scalar, unit })
    // Ok(String::from("1d"))
}

fn calculate_interval(time_range: &TimeRange) -> TimeRange {
    let multiplier = match time_range.unit {
        TimeUnit::Minute => 60,
        TimeUnit::Hour => 60 * 60,
        TimeUnit::Day => 60 * 60 * 24,
        // TimeUnit::Week => 60 * 60 * 24 * 7,
        TimeUnit::Month => 60 * 60 * 24 * 30,
        TimeUnit::Year => 60 * 60 * 24 * 365,
    };
    let total_time: u64 = time_range.scalar as u64 * multiplier as u64;
    let interval_unit = match total_time {
        0..=3_600 => TimeUnit::Minute,
        3_601..=172_800 => TimeUnit::Hour,
        // 172_801..=2_592_000 => TimeUnit::Day,
        // 172_801..=7_776_000 => TimeUnit::Week,
        172_801..=20_000_000 => TimeUnit::Day,
        _ => TimeUnit::Month,
    };
    TimeRange {
        scalar: 1,
        unit: interval_unit,
    }
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
        .arg(
            Arg::with_name("time")
                .value_name("TIME")
                .long("time")
                .short("t")
                .help("Specify time period"),
        )
        .get_matches()
}
