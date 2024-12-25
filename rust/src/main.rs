use clap::Parser;

#[derive(Debug, clap::Parser)]
#[command(long_about = None)]
struct Args {
    /// Year to run.
    #[arg(short, long)]
    year: u16,

    /// Day in year to run.
    #[arg(short, long, value_parser = clap::value_parser!(u8).range(1..=25))]
    day: u8,

    /// Part of day to run.
    #[arg(short, long, value_parser = clap::value_parser!(u8).range(1..=2))]
    part: u8,

    /// Which input to use.
    /// Use "e" or "e1" for "day{day}.example01.txt", "e2" for the second example if present,
    /// or leave empty to use the real input.
    #[arg()]
    input: Option<String>,
}

fn main() {
    let args = Args::parse();

    const SRC_DIR: &str = concat!(env!("CARGO_MANIFEST_DIR"), "/src");

    let input_file = match args.input.as_deref() {
        None => format!("{SRC_DIR}/year{}/input/day{:0>2}.txt", args.year, args.day),
        Some("e") | Some("e1") => format!("{SRC_DIR}/year{}/input/day{:0>2}.example01.txt", args.year, args.day),
        Some("e2") => format!("{SRC_DIR}/year{}/input/day{:0>2}.example02.txt", args.year, args.day),
        Some(other) => panic!("unknown input {other}"),
    };

    let Ok(input) = std::fs::read_to_string(&input_file) else {
        panic!("Unable to read input file {input_file}")
    };

    let function = rust_aoc::get_solution(args.year, args.day, args.part);

    function(&input)
}
