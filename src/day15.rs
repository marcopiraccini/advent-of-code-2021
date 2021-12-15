use std::cmp::Ordering;
use std::collections::{BinaryHeap, HashSet};

#[derive(PartialEq, Eq)]
struct Node {
    p: (usize, usize),
    c: usize,
}

impl Ord for Node {
    fn cmp(&self, other: &Self) -> Ordering {
        self.c.cmp(&other.c).reverse()
    }
}

impl PartialOrd for Node {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

fn parse_input(input: &String) -> Vec<Vec<usize>> {
    input
        .lines()
        .map(|l| {
            l.chars()
                .map(|x| x.to_digit(10).unwrap() as usize)
                // let mut queue: VecDeque<Node> = VecDeque::new();
                .collect::<Vec<usize>>()
        })
        .collect::<Vec<_>>()
}

fn get_next(pos: (usize, usize), size: usize) -> Vec<(usize, usize)> {
    let mut ret = Vec::new();
    let (x, y) = pos;
    if x > 0 {
        ret.push((x - 1, y));
    }
    if y > 0 {
        ret.push((x, y - 1));
    }
    if x < size - 1 {
        ret.push((x + 1, y));
    }
    if y < size - 1 {
        ret.push((x, y + 1));
    }
    ret
}

fn explore_map(map: Vec<Vec<usize>>) -> usize {
    // it's a square
    let size = map[0].len();
    let mut visited = HashSet::new();
    let mut queue = BinaryHeap::new();
    let start = (0, 0);
    let end = (size - 1, size - 1);
    queue.push(Node { p: start, c: 0 });
    loop {
        // pop in BinaryHeap is sorted (in reverse)
        // https://doc.rust-lang.org/stable/std/collections/struct.BinaryHeap.html
        let node = queue.pop().unwrap();
        if node.p == end {
            return node.c;
        }
        // insert returns false if the value is already there, so if already
        // visiterd, we continue
        if !visited.insert(node.p) {
            continue;
        }

        for adj in get_next((node.p.0, node.p.1), size) {
            queue.push(Node {
                p: adj,
                c: node.c + map[adj.0][adj.1],
            });
        }
    }
}

pub fn part1(input: String) {
    let map = parse_input(&input);
    let res = explore_map(map);
    println!("Solution part 1: {:?} ", res);
}

pub fn part2(input: String) {
    let map = parse_input(&input);
    // It's a square
    let size = map[0].len();
    let mut new_map = vec![vec![0; size * 5]; size * 5];

    for yy in 0..5 {
        for xx in 0..5 {
            for y in 0..size {
                for x in 0..size {
                    let el = map[y][x] + yy + xx;
                    new_map[size * yy + y][size * xx + x] = if el >= 10 { el - 9 } else { el };
                }
            }
        }
    }
    let res = explore_map(new_map);
    println!("Solution part 2: {:?} ", res);
}
