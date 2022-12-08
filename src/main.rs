
use clap::{Parser, Subcommand};

mod helpers;
mod day1;
mod day2;
mod day3;
mod day4;
mod day5;
mod day6;
mod day7;
mod day8;
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
    },
    Day4 {
        #[arg(long)]
        inputfile: String
    },
    Day5 {
        #[arg(long)]
        inputfile: String
    },
    Day6 {
        #[arg(long)]
        stream: String
    },
    Day7 {
        #[arg(long)]
        inputfile: String
    },
    Day8 {
        #[arg(long)]
        inputfile: String
    },
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
        Some(Commands::Day4 { inputfile }) => {
            day4::main::run(inputfile.to_string());
        },
        Some(Commands::Day5 { inputfile }) => {
            day5::main::run(inputfile.to_string());
        },
        Some(Commands::Day6 { stream }) => {
            day6::main::run(stream.to_string());
        },
        Some(Commands::Day7 { inputfile }) => {
            day7::main::run(inputfile.to_string());
        },
        Some(Commands::Day8 { inputfile }) => {
            day8::main::run(inputfile.to_string());
        },
        None => {}
    }
}
