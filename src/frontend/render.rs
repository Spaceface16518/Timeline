use rayon::prelude::{
    IndexedParallelIterator,
    IntoParallelIterator,
    ParallelIterator,
};
use serde_yaml::{from_reader,};
use std::{
    cell::RefCell,
    collections::BinaryHeap,
    fs::File,
    io::BufReader,
    path::PathBuf,
};
use structopt::StructOpt;
use timeline::Entry;
use vec_map::VecMap;

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
        let mut entries = entries;
        entries.sort_unstable();
        let end = entries.last().unwrap().end();
        let start = entries.first().unwrap().start();
        let interval = (end - start) as usize / (entries.len() * 3 / 2);

        let mut indices = VecMap::<RefCell<BinaryHeap<Entry>>>::new();
        for entry in entries {
            let i = (entry.start() - start) as usize / interval;
            if let Some(v) = indices.get_mut(i) {
                let v = v.get_mut();
                v.push(entry);
            } else {
                indices.insert(
                    i,
                    RefCell::new({
                        let mut tmp = BinaryHeap::with_capacity(1);
                        tmp.push(entry);
                        tmp
                    }),
                );
            }
        }

        for i in (start..=end).step_by(interval) {
            let mut outputs = String::new();
            if let Some(v) = indices.remove((i - start) as usize / interval) {
                outputs += "| ";
                let line: String = v
                    .into_inner()
                    .into_par_iter()
                    .rev()
                    .enumerate()
                    .map(|(indent, e)| {
                        let mut pre = String::new();
                        if indent > 0 {
                            pre.push_str(" \\");
                            let mut curr_indent_pos = 0;
                            while curr_indent_pos < indent {
                                pre.push('\\');
                                curr_indent_pos += 1;
                            }
                        }
                        pre.push(' ');
                        pre + &e.to_string() + "\n"
                    })
                    .collect();
                outputs += &line;
            } else {
                outputs += "|\n";
            }
            println!("{}", outputs);
        }
    } else {
        unimplemented!()
    }
}
