use clap::Parser;
use time::OffsetDateTime;
use time::macros::format_description;

#[derive(Debug, Parser)]
struct Cli {
    #[arg(
        long,
        short,
        default_value_t = OffsetDateTime::now_utc(),
        value_parser = parse_timestamp,)]
    timestamp: OffsetDateTime,
}

fn parse_timestamp(s: &str) -> anyhow::Result<OffsetDateTime> {
    use time::format_description::well_known::Rfc3339;

    // Try RFC3339 first (most common)
    if let Ok(dt) = OffsetDateTime::parse(s, &Rfc3339) {
        return Ok(dt);
    }

    // Try Unix timestamp
    if let Ok(timestamp) = s.parse::<i64>() {
        if let Ok(dt) = OffsetDateTime::from_unix_timestamp(timestamp) {
            return Ok(dt);
        }
    }

    // Try default value which is '2025-11-07 9:08:26.739054 +00:00:00' format
    let format = format_description!(
        "[year]-[month]-[day] [hour padding:none]:[minute]:[second].[subsecond] [offset_hour sign:mandatory]:[offset_minute]:[offset_second]"
    );

    if let Ok(d) = OffsetDateTime::parse(s, &format) {
        return Ok(d);
    }

    Err(anyhow::anyhow!(
        "Invalid timestamp format: {}. Expected RFC3339 (e.g., '2024-01-15T12:30:00Z') or Unix timestamp",
        s
    ))
}

pub fn main() -> anyhow::Result<()> {
    let cli = Cli::parse();
    println!("{:?}", &cli);
    Ok(())
}
