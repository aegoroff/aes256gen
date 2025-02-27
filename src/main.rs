use bugreport::{
    bugreport,
    collector::{CompileTimeInformation, EnvironmentVariables, OperatingSystem, SoftwareVersion},
    format::Markdown,
};
use std::{fs::File, io::Write};

use clap::{Arg, ArgAction, Command, command};
use color_eyre::eyre::{Context, Result};
use csv::{QuoteStyle, Terminator};
use rand::Rng;
use serde::Serialize;

#[macro_use]
extern crate clap;

const LIMIT_HELP: &str = "The maximum number of codes to generate.";
const CSV_HELP: &str = "Path to comma separated file to store results into";
const DEFAULT_LIMIT: &str = "10";
const KEY_LEN: usize = 32;

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
    let bugreport = matches.get_flag("bugreport");
    if bugreport {
        print_bugreport();
        return Ok(());
    }

    let default_limit: usize = DEFAULT_LIMIT.parse().unwrap_or_default();
    let limit = matches.get_one::<usize>("limit").unwrap_or(&default_limit);
    let csv = matches.get_one::<String>("csv");
    let records = (1..=*limit).map(|id| Record {
        id,
        num: String::new(),
        aeskey: generate_secret(),
    });
    if let Some(path) = csv {
        let mut f = File::create(path)?;
        f.write_all(r#""id","num","aeskey""#.as_bytes())?;
        f.write_all("\r\n".as_bytes())?;
        let mut wtr = csv::WriterBuilder::new()
            .quote_style(QuoteStyle::Necessary)
            .has_headers(false)
            .terminator(Terminator::CRLF)
            .from_writer(f);
        for r in records {
            wtr.serialize(r)
                .wrap_err_with(|| "Failed to serialize record")?;
        }
        wtr.flush()?;
    } else {
        for r in records {
            println!("{}", r.aeskey);
        }
    }
    Ok(())
}

fn generate_secret() -> String {
    hex::encode_upper(rand::rng().random::<[u8; KEY_LEN]>())
}

fn build_cli() -> Command {
    #![allow(non_upper_case_globals)]
    command!(crate_name!())
        .arg_required_else_help(false)
        .version(crate_version!())
        .author(crate_authors!("\n"))
        .about(crate_description!())
        .arg(limit_arg())
        .arg(bugreport_arg())
        .arg(csv_file_path_arg())
}

fn print_bugreport() {
    bugreport!()
        .info(SoftwareVersion::default())
        .info(OperatingSystem::default())
        .info(EnvironmentVariables::list(&["SHELL", "TERM"]))
        .info(CompileTimeInformation::default())
        .print::<Markdown>();
}

fn limit_arg() -> Arg {
    arg!(-l --limit <NUMBER>)
        .required(false)
        .default_value(DEFAULT_LIMIT)
        .value_parser(value_parser!(usize))
        .help(LIMIT_HELP)
}

fn bugreport_arg() -> Arg {
    arg!(--bugreport)
        .required(false)
        .action(ArgAction::SetTrue)
        .help("Collect information about the system and the environment that users can send along with a bug report")
}

fn csv_file_path_arg() -> Arg {
    arg!(-c --csv <PATH>).required(false).help(CSV_HELP)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn generate_secret_test() {
        // Arrange

        // Act
        let secret = generate_secret();

        // Assert
        assert_eq!(64, secret.len());
    }
}
