use clap::Clap;

/// This tool runs my solutions to the advent of code problems
/// (https://adventofcode.com/). It can download and cache problem inputs
/// automatically.
#[derive(Clap)]
#[clap(version = "1.0", author = "Kevin K. <kbknapp@gmail.com>")]
struct Opts {
    /// Solve specific day. If no day is specified, all days will be solved.
    #[clap(short, long)]
    day: Option<usize>,

    /// The session ID for your https://adventofcode.com/ user.
    #[clap(short, long)]
    session: Option<String>,
}

fn main() {
    let opts: Opts = Opts::parse();

    // Gets a value for config if supplied by user, or defaults to "default.conf"
    println!("Value for session: {:?}", opts.session);
    println!("Using input file: {:?}", opts.day);

    aoc::run(opts.day, opts.session)
}
