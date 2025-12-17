use anyhow::{Context, Result, bail};
use serde::Deserialize;

#[derive(Debug)]
struct CsvConfig {
    #[allow(dead_code)]
    base_currency: String,
}

impl Default for CsvConfig {
    fn default() -> Self {
        Self {
            base_currency: "USD".to_string(),
        }
    }
}

/// Parse comment line for base_currency
fn parse_metadata_comment(line: &str) -> Result<CsvConfig> {
    // let mut spl_iter = line.splitn(2, ":");
    // let val = spl_iter.next().ok_or(anyhow!("no value found"))?;
    // if !val.contains("base_currency") {
    //     return Err(anyhow!("expected base_currency"));
    // }
    // let currency = spl_iter.next().ok_or(anyhow!("no currency found"))?;

    // Explicit validation, must start with "# base_currency:" in this exact format
    let content = line
        .strip_prefix("# base_currency:")
        .context("Line must starts with '# base_currency:")?;
    // .context vs .ok_or(anyhow!("Line must starts with '# base_currency:"))?;

    let currency = content.trim();

    if currency.is_empty() {
        bail!("currency can't be empty");
    }

    Ok(CsvConfig {
        base_currency: currency.to_ascii_uppercase(),
    })
}

/// Extracts optional metadata as config and returns remaining CSV data
fn extract_csv_config(csv_data: &str) -> Result<(CsvConfig, &str)> {
    let first_line = csv_data.lines().next().context("no data")?;

    if let Ok(metadata) = parse_metadata_comment(first_line) {
        // we have a metadata commment
        let remain = csv_data
            .split_once("\n")
            .map(|(_, r)| r)
            .context("no ramaining csv data")?;
        Ok((metadata, remain))
    } else {
        // we assume there is no metadata, directly using csv data
        Ok((CsvConfig::default(), csv_data))
    }
}

// alternative - no real perf increase, less idiomatic, not recommended
#[allow(dead_code)]
fn extract_csv_config_v2(csv_data: &str) -> Result<(CsvConfig, &str)> {
    let first_line_index = csv_data.find("\n").context("missing first line")?;
    let first_line = &csv_data[..first_line_index];

    if let Ok(metadata) = parse_metadata_comment(first_line) {
        Ok((metadata, &csv_data[first_line_index + 1..]))
    } else {
        Ok((CsvConfig::default(), csv_data))
    }
}

#[allow(dead_code)]
#[derive(Debug, Deserialize)]
struct CsvRow {
    ticker: String,
    amount: f64,
    price: f64,
}

fn main() -> Result<()> {
    let input_data = "# base_currency: USD
ticker,amount,price
BTC,0.001,100000.0
BTC,0.002,80000.0";

    let (csv_config, csv_data) = extract_csv_config(input_data)?;
    println!("{:?}", csv_config);

    let mut reader = csv::ReaderBuilder::new().from_reader(csv_data.as_bytes());
    // let trades: Vec<CsvRow> = reader.deserialize().collect::<Result<_, csv::Error>>()?;
    let res: Result<Vec<CsvRow>, csv::Error> = reader.deserialize().collect();
    let trades = res?;

    println!("{:?}", trades);
    Ok(())
}
