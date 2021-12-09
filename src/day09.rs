use std::collections::HashSet;
use std::collections::VecDeque;

fn get_adiacent(x: usize, y: usize, sizex: usize, sizey: usize) -> Vec<(usize, usize)> {
    let mut ret = Vec::new();
    if x > 0 {
        ret.push((x - 1, y));
    }
    if y > 0 {
        ret.push((x, y - 1));
    }
    if x < sizex - 1 {
        ret.push((x + 1, y));
    }
    if y < sizey - 1 {
        ret.push((x, y + 1));
    }
    ret
}

fn parse_input(input: &String) -> Vec<Vec<usize>> {
    input
        .lines()
        .map(|l| {
            l.chars()
                .map(|x| x.to_digit(10).unwrap() as usize)
                .collect::<Vec<usize>>()
        })
        .collect::<Vec<_>>()
}

pub fn part1(input: String) {
    let map = parse_input(&input);
    let sizex = map[0].len();
    let sizey = map.len();
    let mut res = 0;
    for x in 0..sizex {
        for y in 0..sizey {
            let val = map[y][x];
            let adj = get_adiacent(x, y, sizex, sizey);
            let adj_min = adj.iter().map(|(x, y)| map[*y][*x]).min().unwrap();
            if val < adj_min {
                res += val + 1;
            }
        }
    }
    println!("Solution part 1: {:?}", res);
}

fn get_next(
    x: usize,
    y: usize,
    sizex: usize,
    sizey: usize,
    map: &Vec<Vec<usize>>,
    explored: &HashSet<(usize, usize)>,
) -> VecDeque<(usize, usize)> {
    let mut ret = VecDeque::new();
    if x > 0 && map[y][x - 1] != 9 && !explored.contains(&(x - 1, y)) {
        ret.push_back((x - 1, y));
    }
    if y > 0 && map[y - 1][x] != 9 && !explored.contains(&(x, y - 1)) {
        ret.push_back((x, y - 1));
    }
    if x < sizex - 1 && map[y][x + 1] != 9 && !explored.contains(&(x + 1, y)) {
        ret.push_back((x + 1, y));
    }
    if y < sizey - 1 && map[y + 1][x] != 9 && !explored.contains(&(x, y + 1)) {
        ret.push_back((x, y + 1));
    }
    return ret;
}

fn get_basin_size(x: usize, y: usize, sizex: usize, sizey: usize, map: &Vec<Vec<usize>>) -> usize {
    let mut explored = HashSet::<(usize, usize)>::new();
    explored.insert((x, y));
    let mut to_explore = get_next(x, y, sizex, sizey, map, &explored).clone();
    while to_explore.len() != 0 {
        let el = to_explore.pop_front().unwrap();
        let near_el = get_next(el.0, el.1, sizex, sizey, map, &explored);
        to_explore.extend(near_el);
        explored.insert(el);
    }
    explored.len()
}

pub fn part2(input: String) {
    let map = parse_input(&input);
    let sizex = map[0].len();
    let sizey = map.len();
    let mut basins_sizes = Vec::<usize>::new();
    for x in 0..sizex {
        for y in 0..sizey {
            let val = map[y][x];
            let adj = get_adiacent(x, y, sizex, sizey);
            let adj_min = adj.iter().map(|(x, y)| map[*y][*x]).min().unwrap();
            if val < adj_min {
                basins_sizes.push(get_basin_size(x, y, sizex, sizey, &map));
            }
        }
    }

    basins_sizes.sort();
    let res: Vec<usize> = basins_sizes.into_iter().rev().collect();
    println!("Solution part 2: {:?}", res[0] * res[1] * res[2]);
}
