#![deny(clippy::all)]

use serde_json::{to_string, to_string_pretty};
use serde_yaml::to_string as to_yml;
use structopt::StructOpt;
use timeline::Entry;

fn main() {
    let input = App::from_args();
    match input.subcmd {
        Command::Parse { input
        } => parse(input),
    };
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
        #[structopt(flatten)]
        input: Parse
    },
}

#[derive(Debug, StructOpt)]
struct Parse {
#[structopt(
            short = "y",
            long = "yaml",
            help = "Outputs in YAML instead of JSON"
        )]
        yaml: bool,
        #[structopt(
            short = "p",
            long = "pretty",
            help = "Pretty prints outputs"
        )]
        pretty: bool,
        #[structopt(
            short = "l",
            long = "label",
            help = "The label for this entry"
        )]
        label: String,

        #[structopt(
            short = "t",
            long = "tag",
            help = "An optional tag for the entry"
        )]
        tag: Option<String>,

        #[structopt(
            short = "s",
            long = "start",
            help = "The start year or point year"
        )]
        start: i32,

        #[structopt(short = "e", long = "end", help = "The end year")]
        end: i32,
}

fn parse(parse: Parse) {
    let entry = Entry::new(parse.label, parse.tag, parse.start, parse.end);

    if parse.pretty {
        if parse.yaml {
            println!(
                "{}",
                to_yml(&entry).expect("Could not convert this entry to yaml")
            )
        } else {
            println!(
                "{}",
                to_string_pretty(&entry)
                    .expect("Could not convert this entry to pretty json")
            )
        }
    } else if parse.yaml {
        print!(
            "{}",
            to_yml(&entry).expect("Could not convert this entry to yaml")
        )
    } else {
        print!(
            "{}",
            to_string(&entry).expect("Could not convert this entry to json")
        )
    }
}
