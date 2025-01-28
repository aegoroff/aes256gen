use clap::{command, Arg, Command};
use rand::Rng;

#[macro_use]
extern crate clap;

const LIMIT_HELP: &str = "The maximum number of codes to generate.";

fn main() {
    let app = build_cli();
    let matches = app.get_matches();
    let limit = matches.get_one::<usize>("limit");
    if let Some(limit) = limit {
        for _ in 0..*limit {
            let secret = rand::rng().random::<[u8; 32]>();
            println!("{}", hex::encode_upper(secret));
        }
    }
}

fn build_cli() -> Command {
    #![allow(non_upper_case_globals)]
    command!(crate_name!())
        .arg_required_else_help(false)
        .version(crate_version!())
        .author(crate_authors!("\n"))
        .about(crate_description!())
        .arg(limit_arg())
}

fn limit_arg() -> Arg {
    arg!(-l --limit <NUMBER>)
        .required(false)
        .default_value("10")
        .value_parser(value_parser!(usize))
        .help(LIMIT_HELP)
}
