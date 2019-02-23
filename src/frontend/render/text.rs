use rayon::iter::{
    IndexedParallelIterator,
    IntoParallelIterator,
    ParallelIterator,
};
use std::{cell::RefCell, collections::BinaryHeap};
use timeline::Entry;
use vec_map::{
    Entry::{Occupied, Vacant},
    VecMap,
};

const INDENT: &str = " ";
const PRELUDE: &str = "| ";
const EMPTY_LINE: &str = "|\n";
const SAME_LINE_INDENT: &str = "\\";
const SAME_LINE_PRELUDE: &str = " \\";
const NEWLINE: &str = "\n";
const INDENT_SIZE: usize = 1;
const PRELUDE_SIZE: usize = 2;

const fn interval(start: i32, end: i32, len: usize) -> usize {
    (end - start) as usize / (len * 3 / 2)
}

const fn index(start: i32, end: i32, interval: usize) -> usize {
    (end - start) as usize / interval
}

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
        let i = index(start, entry.start(), interval);
        match indices.entry(i) {
            Occupied(mut e) => {
                e.get_mut().get_mut().push(entry);
            },
            Vacant(e) => {
                e.insert(RefCell::new({
                    let mut tmp = BinaryHeap::with_capacity(1);
                    tmp.push(entry);
                    tmp
                }));
            },
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
        if let Some(v) = indices.remove(index(start, i, interval)) {
            outputs += PRELUDE;
            let line: String = v
                .into_inner()
                .into_par_iter()
                .rev()
                .enumerate()
                .map(|(indent, e)| {
                    let s = e.to_string();
                    let mut pre =
                        String::with_capacity(s.len() + indent + PRELUDE_SIZE);
                    if indent > 0 {
                        pre += SAME_LINE_PRELUDE;
                        let mut curr_indent_pos = 0;
                        while curr_indent_pos < indent {
                            pre += SAME_LINE_INDENT;
                            curr_indent_pos += INDENT_SIZE;
                        }
                    }
                    pre += INDENT;
                    pre + &s + NEWLINE
                })
                .collect();
            outputs += &line;
        } else {
            outputs += EMPTY_LINE;
        }
    }
    outputs
}

#[cfg(test)]
mod const_tests {
    use super::{
        INDENT,
        INDENT_SIZE,
        PRELUDE,
        PRELUDE_SIZE,
        SAME_LINE_INDENT,
        SAME_LINE_PRELUDE,
    };

    #[test]
    fn assert_indent_size() {
        assert_eq!(INDENT.len(), INDENT_SIZE);
        assert_eq!(SAME_LINE_INDENT.len(), INDENT_SIZE);
    }

    #[test]
    fn assert_prelude_size() {
        assert_eq!(PRELUDE.len(), PRELUDE_SIZE);
        assert_eq!(SAME_LINE_PRELUDE.len(), PRELUDE_SIZE);
    }
}
