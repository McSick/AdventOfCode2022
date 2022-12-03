
use clap::{Parser, Subcommand};

mod helpers;
mod day1;
mod day2;
mod day3;

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Option<Commands>,
}

#[derive(Subcommand)]
enum Commands {
    Day1 {
        #[arg(long)]
        inputfile: String
    },
    Day2 {
        #[arg(long)]
        inputfile: String
    },
    Day3 {
        #[arg(long)]
        inputfile: String
    }
}

fn main() {
    let cli = Cli::parse();
    // You can check for the existence of subcommands, and if found use their
    // matches just as you would the top level cmd
    match &cli.command {
        Some(Commands::Day1 { inputfile }) => {
            day1::main::run(inputfile.to_string());
        },
        Some(Commands::Day2 { inputfile }) => {
            day2::main::run(inputfile.to_string());
        },
        Some(Commands::Day3 { inputfile }) => {
            day3::main::run(inputfile.to_string());
        },
        None => {}
    }
}
