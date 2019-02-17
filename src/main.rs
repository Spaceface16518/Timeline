#![deny(clippy::all)]

use serde_json::{to_string, to_string_pretty};
use serde_yaml::to_string as to_yml;
use structopt::StructOpt;
use timeline::Entry;

fn main() {
    let input = App::from_args();
    match input.subcmd {
        Command::Parse {
            yaml,
            pretty,
            entry_parse,
        } => {
            let entry = Entry::new(
                entry_parse.label,
                entry_parse.tag,
                entry_parse.start,
                entry_parse.end,
            );

            if pretty {
                if yaml {
                    println!(
                        "{}",
                        to_yml(&entry)
                            .expect("Could not convert this entry to yaml")
                    )
                } else {
                    println!(
                        "{}",
                        to_string_pretty(&entry).expect(
                            "Could not convert this entry to pretty json"
                        )
                    )
                }
            } else if yaml {
                print!(
                    "{}",
                    to_yml(&entry)
                        .expect("Could not convert this entry to yaml")
                )
            } else {
                print!(
                    "{}",
                    to_string(&entry)
                        .expect("Could not convert this entry to json")
                )
            }
        },
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
        #[structopt(flatten)]
        entry_parse: EntryParse,
    },
}

#[derive(Debug, StructOpt)]
struct EntryParse {
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
