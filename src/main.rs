use std::{fs::File, io::Write};

use clap::{command, Arg, Command};
use color_eyre::eyre::{Context, Result};
use csv::{QuoteStyle, Terminator};
use rand::Rng;
use serde::Serialize;

#[macro_use]
extern crate clap;

const LIMIT_HELP: &str = "The maximum number of codes to generate.";
const CSV_HELP: &str = "Path to comma separated file to store results into";
const DEFAULT_LIMIT: &str = "10";

#[derive(Debug, Serialize)]
struct Record {
    id: usize,
    num: String,
    aeskey: String,
}

fn main() -> Result<()> {
    color_eyre::install()?;
    let app = build_cli();
    let matches = app.get_matches();
    let default_limit: usize = DEFAULT_LIMIT.parse().unwrap_or_default();
    let limit = matches.get_one::<usize>("limit").unwrap_or(&default_limit);
    let csv = matches.get_one::<String>("csv");
    if let Some(path) = csv {
        let mut f = File::create(path)?;
        f.write_all(r#""id","num","aeskey""#.as_bytes())?;
        f.write_all("\r\n".as_bytes())?;
        let mut wtr = csv::WriterBuilder::new()
            .quote_style(QuoteStyle::Necessary)
            .has_headers(false)
            .terminator(Terminator::CRLF)
            .from_writer(f);
        for i in 0..*limit {
            let secret = rand::rng().random::<[u8; 32]>();
            wtr.serialize(Record {
                id: i + 1,
                num: String::new(),
                aeskey: hex::encode_upper(secret),
            })
            .wrap_err_with(|| "Failed to serialize record")?;
        }
        wtr.flush()?;
    } else {
        for _ in 0..*limit {
            let secret = rand::rng().random::<[u8; 32]>();
            println!("{}", hex::encode_upper(secret));
        }
    }
    Ok(())
}

fn build_cli() -> Command {
    #![allow(non_upper_case_globals)]
    command!(crate_name!())
        .arg_required_else_help(false)
        .version(crate_version!())
        .author(crate_authors!("\n"))
        .about(crate_description!())
        .arg(limit_arg())
        .arg(csv_file_path_arg())
}

fn limit_arg() -> Arg {
    arg!(-l --limit <NUMBER>)
        .required(false)
        .default_value(DEFAULT_LIMIT)
        .value_parser(value_parser!(usize))
        .help(LIMIT_HELP)
}

fn csv_file_path_arg() -> Arg {
    arg!(-c --csv <PATH>).required(false).help(CSV_HELP)
}
