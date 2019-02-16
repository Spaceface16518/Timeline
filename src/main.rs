use bson::to_bson;
use serde_yaml::to_string;
use structopt::StructOpt;
use timeline::Entry;

fn main() {
    let input = App::from_args();
    let b: bool;
    let entry = match input.subcmd {
        Command::Parse { bson, entry_parse } => {
            b = bson;
            Entry::new(
                entry_parse.label,
                entry_parse.tag,
                entry_parse.start,
                entry_parse.end,
            )
        },
    };
    println!(
        "{}",
        if b {
            to_bson(&entry).unwrap().to_string()
        } else {
            to_string(&entry).unwrap()
        }
    );
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
        about = "Parse some options into a serializable format"
    )]
    Parse {
        /// Outputs in BSON instead of YAML
        #[structopt(short = "b", long = "bson")]
        bson: bool,
        #[structopt(flatten)]
        entry_parse: EntryParse,
    },
}

#[derive(Debug, StructOpt)]
struct EntryParse {
    /// The label for this entry
    #[structopt(short = "l", long = "label")]
    label: String,

    /// An optional tag for the entry
    #[structopt(short = "t", long = "tag")]
    tag: Option<String>,

    /// The start year or point year
    #[structopt(short = "s", long = "start")]
    start: i32,

    /// The end year
    #[structopt(short = "e", long = "end")]
    end: i32,
}
