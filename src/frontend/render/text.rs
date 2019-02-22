use rayon::iter::{
    IndexedParallelIterator,
    IntoParallelIterator,
    ParallelIterator,
};
use std::{cell::RefCell, collections::BinaryHeap};
use timeline::Entry;
use vec_map::VecMap;

pub fn text_handler(mut entries: Vec<Entry>) -> String {
    entries.sort_unstable();
    let end = entries.last().unwrap().end();
    let start = entries.first().unwrap().start();
    let interval = interval(start, end, entries.len());

    let indices = indices(entries, start, interval);
    outputs(indices, start, end, interval)
}

fn indices(
    entries: Vec<Entry>,
    start: i32,
    interval: usize,
) -> VecMap<RefCell<BinaryHeap<Entry>>> {
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
    indices
}

fn outputs(
    mut indices: VecMap<RefCell<BinaryHeap<Entry>>>,
    start: i32,
    end: i32,
    interval: usize,
) -> String {
    let mut outputs = String::new();
    for i in (start..=end).step_by(interval) {
        if let Some(v) = indices.remove((i - start) as usize / interval) {
            outputs += "| ";
            let line: String = v
                .into_inner()
                .into_par_iter()
                .rev()
                .enumerate()
                .map(|(indent, e)| {
                    let s = e.to_string();
                    let mut pre = String::with_capacity(s.len() + indent + 2);
                    if indent > 0 {
                        pre.push_str(" \\");
                        let mut curr_indent_pos = 0;
                        while curr_indent_pos < indent {
                            pre.push('\\');
                            curr_indent_pos += 1;
                        }
                    }
                    pre.push(' ');
                    pre + &s + "\n"
                })
                .collect();
            outputs += &line;
        } else {
            outputs += "|\n";
        }
        outputs += "\n";
    }
    outputs
}

const fn interval(start: i32, end: i32, len: usize) -> usize {
    // (end - start) as usize / (len * 3 / 2)
    index(start, end, len * 3 / 2)
}

const fn index(start: i32, end: i32, interval: usize) -> usize {
    (end - start) as usize / interval
}
