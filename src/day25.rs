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

    pub fn set(&mut self, x: usize, y: usize, val: usize) {
        self.map[y][x] = val;
    }

    fn move_east(&mut self, x: usize, y: usize, new_map: &mut VecDeque<VecDeque<usize>>) -> bool {
        if self.map[y][x] != 1 {
            return false;
        };
        let next = if x + 1 == self.lenx() { 0 } else { x + 1 };
        if self.map[y][next] == 0 {
            new_map[y][x] = 0;
            new_map[y][next] = 1;
            return true;
        }
        return false;
    }

    fn move_south(&mut self, x: usize, y: usize, new_map: &mut VecDeque<VecDeque<usize>>) -> bool {
        if self.map[y][x] != 2 {
            return false;
        };
        let next = if y + 1 == self.leny() { 0 } else { y + 1 };
        if self.map[next][x] == 0 {
            new_map[y][x] = 0;
            new_map[next][x] = 2;
            return true;
        }
        return false;
    }

    fn move_all(&mut self) -> bool {
        let mut moved = false;

        let mut new_map = self.map.clone();
        // first east
        for y in 0..self.leny() {
            for x in 0..self.lenx() {
                let res = self.move_east(x, y, &mut new_map);
                if res {
                    moved = true;
                }
            }
        }
        self.map = new_map;
        new_map = self.map.clone();
        // then south
        for y in 0..self.leny() {
            for x in 0..self.lenx() {
                let res = self.move_south(x, y, &mut new_map);
                if res {
                    moved = true;
                }
            }
        }
        self.map = new_map;
        moved
    }

    pub fn print(&self) {
        let printable_map = self
            .map
            .iter()
            .map(|row| {
                let row_str = row.iter().fold(String::from(""), |mut acc, el| {
                    if *el == 1 {
                        acc.push_str(">");
                    } else if *el == 2 {
                        acc.push_str("v");
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
        println!("\n");
    }
}

fn parse_input(input: String) -> VecDeque<VecDeque<usize>> {
    let map = input
        .lines()
        .map(|l| {
            l.chars()
                .map(|s| {
                    if s == '>' {
                        1 as usize
                    } else if s == 'v' {
                        2 as usize
                    } else {
                        0 as usize
                    }
                })
                .collect::<VecDeque<usize>>()
        })
        .collect::<VecDeque<VecDeque<usize>>>();
    map
}

pub fn part1(input: String) {
    let m = parse_input(input.clone());
    let mut map = Map::init(&m);
    // map.print();
    let mut count = 0;
    loop {
        count += 1;
        let res = map.move_all();
        // map.print();
        if !res {
            break;
        }
    }
    println!("Solution part 1: {:?} ", count);
}
