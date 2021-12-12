use std::collections::{HashMap, HashSet};

fn parse_input(input: String) -> HashMap<&'static str, Vec<&'static str>> {
    let input_str = Box::leak(input.into_boxed_str());
    input_str
        .lines()
        .map(|l| {
            let l = l.split('-').collect::<Vec<&str>>();
            (l[0], l[1])
        })
        .fold(HashMap::new(), |mut acc, el| {
            if !acc.contains_key(el.0) {
                acc.insert(el.0, vec![el.1]);
            } else {
                let p = acc.get_mut(el.0).unwrap();
                p.push(el.1);
            }
            if !acc.contains_key(el.1) {
                acc.insert(el.1, vec![el.0]);
            } else {
                let p = acc.get_mut(el.1).unwrap();
                p.push(el.0);
            }
            return acc;
        })
}

fn count_paths<'a>(
    s: &'static str,
    e: &str,
    paths: &HashMap<&'static str, Vec<&'static str>>,
    seen: &mut HashSet<&'static str>,
    visited_twice: bool,
) -> usize {
    // Rust doesn't support optional arguments :(
    let mut seen_loc = seen.clone();
    if s == e {
        return 1;
    }
    let mut path_count = 0;
    let n = &paths.get(s);
    for next in n.unwrap() {
        let mut next_visited_twice = visited_twice;
        if (*next == next.to_lowercase()) && seen_loc.contains(next) {
            if visited_twice || *next == "start" || *next == "end" {
                continue;
            } else {
                next_visited_twice = true;
            }
        }
        seen_loc.insert(s);
        let c = count_paths(next, e, paths, &mut seen_loc, next_visited_twice);
        path_count += c;
    }
    return path_count;
}

pub fn part1(input: String) {
    let paths = parse_input(input);
    let mut seen = HashSet::<&'static str>::new();
    let res = count_paths("start", "end", &paths, &mut seen, true);
    println!("Solution part 1: {:?}", res);
}

pub fn part2(input: String) {
    let paths = parse_input(input);
    let mut seen = HashSet::<&'static str>::new();
    let res = count_paths("start", "end", &paths, &mut seen, false);
    println!("Solution part 2: {:?}", res);
}
