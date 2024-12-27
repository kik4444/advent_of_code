use std::path::PathBuf;

use clap::{Parser, Subcommand};

#[derive(Debug, Parser)]
#[command(long_about = None)]
struct Args {
    #[command(subcommand)]
    command: Command,
}

#[derive(Debug, Subcommand)]
enum Command {
    /// Run an Advent of Code solution.
    Run(Run),
    /// Generate an Advent of Code day.
    Generate(Generate),
}

#[derive(Debug, Parser)]
struct Run {
    /// Year to run.
    #[arg(short, long)]
    year: u16,

    /// Day in year to run.
    #[arg(short, long, value_parser = clap::value_parser!(u8).range(1..=25))]
    day: u8,

    /// Part of day to run.
    #[arg(short, long, value_parser = clap::value_parser!(u8).range(1..=2), default_value_t = 1)]
    part: u8,

    /// Which input to use.
    /// Use "e" or "e1" for "day{day}.example01.txt", "e2" for the second example if present,
    /// or leave empty to use the real input.
    #[arg()]
    input: Option<String>,
}

#[derive(Debug, Parser)]
struct Generate {
    /// Year to generate.
    #[arg(short, long)]
    year: u16,

    /// Day in year to generate.
    #[arg(short, long, value_parser = clap::value_parser!(u8).range(1..=25))]
    day: u8,
}

fn main() {
    let args = Args::parse();

    match args.command {
        Command::Run(run_cmd) => run(run_cmd),
        Command::Generate(generate_cmd) => generate(generate_cmd),
    }
}

const SRC_DIR: &str = concat!(env!("CARGO_MANIFEST_DIR"), "/src");

fn run(run_cmd: Run) {
    let input_file = match run_cmd.input.as_deref() {
        None => format!("{SRC_DIR}/year{}/input/day{:0>2}.txt", run_cmd.year, run_cmd.day),
        Some("e") | Some("e1") => format!("{SRC_DIR}/year{}/input/day{:0>2}.example01.txt", run_cmd.year, run_cmd.day),
        Some("e2") => format!("{SRC_DIR}/year{}/input/day{:0>2}.example02.txt", run_cmd.year, run_cmd.day),
        Some(other) => panic!("unknown input {other}"),
    };

    let Ok(input) = std::fs::read_to_string(&input_file) else {
        panic!("Unable to read input file {input_file}")
    };

    let function = aoc::get_solution(run_cmd.year, run_cmd.day, run_cmd.part);

    function(&input)
}

fn generate(gen_cmd: Generate) {
    let day_string = format!("day{:0>2}", gen_cmd.day);
    let rs_file = PathBuf::from(format!("{SRC_DIR}/year{}/{}.rs", gen_cmd.year, day_string));

    if std::fs::exists(&rs_file).expect("Failed checking if file exists") {
        panic!("File {} already exists", rs_file.display());
    }

    let year_dir = rs_file.parent().expect("Parent must exist");
    if !year_dir.exists() {
        std::fs::create_dir(year_dir).expect("Failed creating year directory");
        std::fs::write(year_dir.join("mod.rs"), format!("aoc_macros::import_days!({});\n", gen_cmd.year)).expect("Failed creating mod.rs file");
    }

    let input_dir = year_dir.join("input");
    if !input_dir.exists() {
        std::fs::create_dir(&input_dir).expect("Failed creating input directory");
    }

    std::fs::write(
        rs_file,
        "pub fn part1(input: &str) {
    // TODO
}

pub fn part2(input: &str) {
    // TODO
}

fn parse(input: &str) {
    // TODO
}
",
    )
    .expect("Failed creating file.");

    for file in [
        input_dir.join(format!("{day_string}.txt")),
        input_dir.join(format!("{day_string}.example01.txt")),
    ] {
        std::fs::write(file, "").expect("Failed creating input file");
    }
}
