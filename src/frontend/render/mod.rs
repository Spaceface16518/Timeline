use rayon::prelude::{IntoParallelIterator, ParallelIterator};
use serde_yaml::from_reader;
use std::{fs::File, io::BufReader, path::PathBuf};
use structopt::StructOpt;
use text::text_handler;
use timeline::Entry;

mod text;

#[derive(Debug, StructOpt)]
pub struct Render {
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

pub fn render(render: Render) {
    let Render {
        path,
        filter,
        search,
        text,
    } = render;

    let file = File::open(path).expect("Could not open file at specified path");
    let reader = BufReader::new(file);
    let entries: Vec<Entry> = {
        let e: Vec<Entry> = from_reader(reader)
            .expect("Could not convert this yaml file into Timeline entries");
        e.into_par_iter()
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
            .collect()
    };

    if text {
        println!("{}", text_handler(entries));
    } else {
        unimplemented!()
    }
}
