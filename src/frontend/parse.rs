use serde_json::{to_string, to_string_pretty};
use serde_yaml::to_string as to_yml;
use structopt::StructOpt;
use timeline::Entry;

#[derive(Debug, StructOpt)]
pub struct Parse {
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

pub fn parse(parse: Parse) {
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
