use reqwest::blocking::{Client, get};
use serde::Deserialize;
use std::collections::HashMap;

type Error = Box<dyn std::error::Error>;

#[derive(Debug, Deserialize)]
struct PriceResponse {
    usd: f64,
}

#[derive(Debug, Deserialize)]
struct CoinInfo {
    id: String,
    symbol: String,
    name: String,
}

pub fn tickers() -> Result<(), Error> {
    // v1
    let all_coins: Vec<CoinInfo> = get("https://api.coingecko.com/api/v3/coins/list")?.json()?;
    println!("Found {} coins", all_coins.len()); // 20k coins

    // v2
    // Get top 100 coins by market cap
    println!("Get ticker to id list for top 100 coins");
    let api_key = std::env::var("CGECKO_API_KEY").unwrap();
    let url = "https://api.coingecko.com/api/v3/coins/markets?\
               vs_currency=usd&order=market_cap_desc&per_page=250&page=1";

    let client = Client::new();
    let response = client
        .get(url)
        .header("x-cg-demo-api-key", api_key)
        .send()?;

    println!("Response status code: {}", response.status());

    let coins: Vec<CoinInfo> = response.json()?;

    let mut wrt = csv::Writer::from_path("coingecko_top_250.csv")?;

    // write header first
    wrt.write_record(&["id", "symbol", "name"])?;

    for coin in coins.iter() {
        wrt.write_record(&[&coin.id, &coin.symbol.to_uppercase(), &coin.name])?;
    }

    wrt.flush()?;

    Ok(())
}

pub fn reqwest_example() -> Result<(), Error> {
    // coins can be comma-separated: bitcoin,ethereum,solana,...
    let ids = "bitcoin,ethereum";
    // let ids = "BTC,ETH";
    let vs_currency = "usd";

    let url = format!(
        "https://api.coingecko.com/api/v3/simple/price?ids={}&vs_currencies={}",
        ids, vs_currency
    );

    println!("Fetching crypto quotes from CoinGecko...");

    let resp = get(&url)?.json::<HashMap<String, PriceResponse>>()?;

    for (coin, data) in resp {
        println!("{:<10} → {:>10.2} USD", coin, data.usd);
    }

    Ok(())
}

fn main() {
    println!("=========================");
    let _ = reqwest_example();
    let _ = tickers();
}
