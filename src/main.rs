use serde_json::to_string;
use structopt::StructOpt;
use timeline::Entry;

fn main() {
    let input = App::from_args();
    let entry = match input.subcmd {
        Command::Parse { label, start, end } => Entry::new(label, start, end),
    };
    println!("{}", to_string(&entry).unwrap());
}

#[derive(Debug, StructOpt)]
struct App {
    #[structopt(subcommand)]
    /// The subcommand to run
    pub subcmd: Command,
}

#[derive(StructOpt, Debug)]
enum Command {
    #[structopt(
        name = "parse",
        about = "Parse some options into a serializable format (currently, \
                 JSON)"
    )]
    Parse {
        /// The label for this entry
        #[structopt(short = "l", long = "label")]
        label: String,
        /// The start year or point year
        #[structopt(short = "s", long = "start")]
        start: i32,
        /// The end year
        #[structopt(short = "e", long = "end")]
        end: Option<i32>,
    },
}
