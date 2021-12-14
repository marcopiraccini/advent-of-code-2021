use itertools::Itertools;
use std::collections::{BinaryHeap, HashMap};

fn parse_input(input: String) -> (Vec<char>, HashMap<(char, char), char>) {
    let template = input.lines().nth(0).unwrap().chars().collect();

    let rules = input
        .lines()
        .filter(|l| l.contains("->"))
        .fold(HashMap::new(), |mut acc, p| {
            let pp = p.split(" -> ").collect::<Vec<&str>>();
            let left = pp[0].chars().collect::<Vec<char>>();
            let right = pp[1].chars().nth(0).unwrap();
            acc.insert((left[0], left[1]), right);
            return acc;
        });
    (template, rules)
}

pub fn frequency<T: std::hash::Hash + std::cmp::Eq + std::cmp::Ord>(array: &[T], m: bool) -> isize {
    let mut map = HashMap::new();
    for value in array {
        let counter = map.entry(value).or_insert(0);
        *counter += 1 as isize;
    }
    let mut heap: BinaryHeap<_> = map.values().collect();
    if m {
        return *heap.pop().unwrap();
    } else {
        for _ in 0..heap.len() - 1 {
            heap.pop();
        }
        return *heap.pop().unwrap();
    }
}

pub fn part1(input: String) {
    let (template, rules) = parse_input(input);
    let mut temp = template.clone();
    for _ in 0..10 {
        let mut next = Vec::new();
        for i in 0..temp.len() - 1 {
            let pair = (temp[i], temp[i + 1]);
            let rule = rules.get(&pair).unwrap();
            next.push(temp[i]);
            next.push(*rule);
            // the last one  is not covered by any "next" rule, so mut be added.
            if i == temp.len() - 2 {
                next.push(temp[i + 1]);
            }
        }

        temp = next.clone();
    }
    let most_freq = frequency(&temp, true);
    let least_freq = frequency(&temp, false);
    println!("Solution part 1: {:?} ", most_freq - least_freq);
}

pub fn part2(input: String) {
    let (template, rules) = parse_input(input);
    let mut pairs: HashMap<(char, char), usize> = HashMap::new();
    for (a, b) in template.iter().tuple_windows() {
        (*pairs.entry((*a, *b)).or_default()) += 1;
    }

    for _ in 0..40 {
        let mut next: HashMap<(char, char), usize> = HashMap::new();
        // For each pair, chek if a rule apply.
        // If so, add other 2 pairs, e.g.:
        // AB->C
        // ...and AB ->12
        // ...produces: AC, CB
        // (each will start with 12)
        for (&(first, second), v) in &pairs {
            let target = *rules.get(&(first, second)).unwrap();
            (*next.entry((first, target)).or_default()) += v;
            (*next.entry((target, second)).or_default()) += v;
        }
        pairs = next;
    }

    let mut freq: HashMap<char, usize> = HashMap::new();
    for (el, n) in pairs {
        *freq.entry(el.0).or_default() += n;
    }
    // We need to add 1 to the last char
    (*freq.entry(template[template.len() - 1]).or_default()) += 1;
    let res = freq.values().max().unwrap() - freq.values().min().unwrap();
    println!("Solution part 2: {:?} ", res);
}
