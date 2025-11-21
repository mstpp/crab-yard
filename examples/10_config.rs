// Overrides:
// built-in defaults → dotfile → env var → CLI args
// load → layer → validate (at startup)
// validation: if it's not valid, use built-in default and send some warning message
use config::Config;

use clap::Parser;

#[derive(Debug, Parser)]
struct Cli {
    #[arg(short, long)]
    debug: bool,
    #[arg(long)]
    data_dir: Option<String>,
}

fn main() -> anyhow::Result<()> {
    // CLI
    let cli = Cli::parse();
    println!("{:?}", &cli);

    // Config
    //
    // # config.toml:
    // base_currency = "USD"
    // data_dir = "portfolio_dir"
    let dotfile_path = shellexpand::tilde("~/.local/share/csvpt/false_config.toml").to_string();
    let dotfile_exists = std::fs::exists(&dotfile_path)?;
    let dotfile = if dotfile_exists {
        dotfile_path.as_str()
    } else {
        "data/config"
    };
    println!("Loading: {dotfile}");
    let settings: Config = Config::builder()
        // it will first try to find a config file w/o extension .toml
        // it fill fail if file not found
        .add_source(config::File::with_name(dotfile))
        // if same seetings are present in file, it will override them
        .add_source(config::Environment::with_prefix("CSVPT"))
        // override optional cli arg if present
        .set_override_option("data_dir", cli.data_dir)?
        .build()?;

    println!("{:?}", &settings);

    Ok(())
}
