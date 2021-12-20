use itertools::Itertools;
use std::collections::VecDeque;

#[derive(Clone, Debug)]
pub struct Map {
    map: VecDeque<VecDeque<usize>>,
}

impl Map {
    pub fn init(m: &VecDeque<VecDeque<usize>>) -> Map {
        let map = m.clone();
        Map { map }
    }

    fn lenx(&self) -> usize {
        self.map[0].len()
    }

    fn leny(&self) -> usize {
        self.map.len()
    }

    fn resize(&mut self, i: usize) {
        let def = i % 2;
        for i in 0..self.leny() {
            self.map[i].push_front(def);
            self.map[i].push_back(def);
        }
        let line_len = self.lenx();
        self.map.push_back(VecDeque::from(vec![def; line_len]));
        self.map.push_front(VecDeque::from(vec![def; line_len]));
    }

    pub fn get(&self, x: isize, y: isize, index: usize) -> usize {
        let def = index % 2;
        if x < 0 || y < 0 || x == self.lenx() as isize || y == self.leny() as isize {
            // Out of the map
            return def;
        }
        self.map[y as usize][x as usize]
    }

    pub fn set(&mut self, x: usize, y: usize, val: usize) {
        self.map[y][x] = val;
    }

    pub fn get_adiacent_index(&self, xx: usize, yy: usize, index: usize) -> usize {
        let mut ret = Vec::new();
        let x = xx as isize;
        let y = yy as isize;
        ret.push(self.get(x - 1, y - 1, index));
        ret.push(self.get(x, y - 1, index));
        ret.push(self.get(x + 1, y - 1, index));

        ret.push(self.get(x - 1, y, index));
        ret.push(self.get(x, y, index));
        ret.push(self.get(x + 1, y, index));
        ret.push(self.get(x - 1, y + 1, index));
        ret.push(self.get(x, y + 1, index));
        ret.push(self.get(x + 1, y + 1, index));
        let res = ret
            .iter()
            .rev()
            .enumerate()
            .map(|(i, s)| (s * (2 as u32).pow(i as u32) as usize))
            .sum();
        res
    }

    pub fn count(&self) -> usize {
        let mut res = 0;
        for y in 0..self.map.len() {
            for x in 0..self.map[0].len() {
                res += self.map[y][x];
            }
        }
        res
    }

    pub fn print(&self) {
        let printable_map = self
            .map
            .iter()
            .map(|row| {
                let row_str = row.iter().fold(String::from(""), |mut acc, el| {
                    if *el > 0 {
                        acc.push_str("#");
                    } else {
                        acc.push_str(".");
                    }
                    return acc;
                });

                return row_str;
            })
            .collect::<Vec<String>>();

        for line in printable_map {
            println!("{:?}", line);
        }
    }
}

fn parse_input(input: String) -> (Vec<usize>, VecDeque<VecDeque<usize>>) {
    let alg = input
        .split("\n\n")
        .nth(0)
        .unwrap()
        .chars()
        .map(|s| if s == '#' { 1 as usize } else { 0 as usize })
        .collect_vec();
    let map = input
        .split("\n\n")
        .nth(1)
        .unwrap()
        .lines()
        .map(|l| {
            l.chars()
                .map(|s| if s == '#' { 1 as usize } else { 0 as usize })
                .collect::<VecDeque<usize>>()
        })
        .collect::<VecDeque<VecDeque<usize>>>();
    (alg, map)
}

pub fn part1(input: String) {
    let (alg, m) = parse_input(input.clone());
    let mut map = Map::init(&m);
    for i in 0..50 {
        map.resize(i);
        let mut image = map.clone();
        let lenx = map.lenx();
        let leny = map.leny();
        for y in 0..leny {
            for x in 0..lenx {
                let alg_index = map.get_adiacent_index(x, y, i);
                let val = alg[alg_index];
                image.set(x, y, val);
            }
        }
        map = image;
        if i == 1 {
            println!("Solution part 1: {:?} ", map.count());
        }
    }
    println!("Solution part 2: {:?} ", map.count());
}
