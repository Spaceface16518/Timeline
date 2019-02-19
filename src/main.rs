#![deny(clippy::all)]
use serde_json::{to_string, to_string_pretty};
use serde_yaml::{from_reader, to_string as to_yml};
use std::{fs::File, io::BufReader, path::PathBuf};
use structopt::StructOpt;
use timeline::Entry;

fn main() {
    let input = App::from_args();
    match input.subcmd {
        Command::Parse { input } => parse(input),
        Command::Render { input } => render(input),
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
        input: Parse,
    },
    #[structopt(name = "render", about = "Render a timeline file")]
    Render {
        #[structopt(flatten)]
        input: Render,
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
    #[structopt(short = "p", long = "pretty", help = "Pretty prints outputs")]
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
    let Parse {
        label,
        tag,
        start,
        end,
        pretty,
        yaml,
    } = parse;
    let entry = if start == end {
        Entry::point(label, tag, start)
    } else {
        Entry::range(label, tag, start, end)
    };

    if pretty {
        if yaml {
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
    } else if yaml {
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

#[derive(Debug, StructOpt)]
struct Render {
    #[structopt(
        short = "p",
        long = "path",
        help = "Path to the .yml file to load from",
        parse(from_os_str)
    )]
    path: PathBuf,
    #[structopt(
        short = "f",
        long = "filter",
        help = "Filter results by their tag"
    )]
    filter: Option<String>,
    #[structopt(
        short = "s",
        long = "search",
        help = "Filter results by their label"
    )]
    search: Option<String>,
    #[structopt(
        short = "t",
        long = "text",
        help = "Print outputs rather than rendering outputs as HTML"
    )]
    text: bool,
}

fn render(render: Render) {
    let Render {
        path,
        filter,
        search,
        text,
    } = render;

    let file = File::open(path).expect("Could not open file at specified path");
    let reader = BufReader::new(file);
    let entries: Vec<Entry> = from_reader(reader)
        .expect("Could not convert this yaml file into Timeline entries");

    if text {
        let mut entries = entries;
        entries.sort_unstable();
        entries
            .into_iter()
            .filter(|e| {
                if let (Some(m), Some(t)) = (&filter, &e.tag()) {
                    t.contains(m)
                } else {
                    true
                }
            })
            .filter(|e| {
                if let Some(m) = &search {
                    e.label().contains(m)
                } else {
                    true
                }
            })
            .for_each(|e| println!("{}", e));
    } else {
        unimplemented!()
    }
}
