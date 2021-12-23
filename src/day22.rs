use hashbrown::HashSet;
use itertools::Itertools;
use std::collections::VecDeque;

#[derive(Clone, Copy, Debug)]
struct Cuboid {
    x: (isize, isize),
    y: (isize, isize),
    z: (isize, isize),
}

#[derive(Clone, Copy, Debug)]
struct Rule {
    on: bool,
    cuboid: Cuboid,
}

fn parse_range(in_string: &str) -> (isize, isize) {
    let mut splitter = in_string[2..].split("..");
    let low = splitter.next().unwrap().parse::<isize>().unwrap();
    let high = splitter.next().unwrap().parse::<isize>().unwrap() + 1;
    (low, high)
}

fn parse_cuboid(in_string: &str) -> Cuboid {
    let mut splitter = in_string.split(",");
    let xstr = splitter.next().unwrap();
    let ystr = splitter.next().unwrap();
    let zstr = splitter.next().unwrap();
    let x = parse_range(xstr);
    let y = parse_range(ystr);
    let z = parse_range(zstr);
    Cuboid { x, y, z }
}

fn parse_rule(in_string: &str) -> Rule {
    let mut splitter = in_string.splitn(2, " ");
    let start = splitter.next().unwrap();
    let end = splitter.next().unwrap();
    let on = if start == "on" { true } else { false };
    let cuboid = parse_cuboid(end);
    Rule { on, cuboid }
}

fn parse_input(input: &String) -> Vec<Rule> {
    input.lines().map(|l| parse_rule(l)).collect_vec()
}

fn apply_rule(r: Rule, cubes: &mut HashSet<(isize, isize, isize)>) {
    for x in r.cuboid.x.0..r.cuboid.x.1 {
        for y in r.cuboid.y.0..r.cuboid.y.1 {
            for z in r.cuboid.z.0..r.cuboid.z.1 {
                if r.on {
                    cubes.insert((x, y, z));
                } else {
                    cubes.remove(&(x, y, z));
                }
            }
        }
    }
}

fn overlaps(range: (isize, isize)) -> bool {
    let limit = -50..50;
    limit.contains(&range.0) || limit.contains(&range.1)
}

fn skip_rule(r: Rule) -> bool {
    !overlaps(r.cuboid.x) || !overlaps(r.cuboid.y) || !overlaps(r.cuboid.z)
}

fn intersect(a: &Cuboid, b: &Cuboid) -> bool {
    a.x.0 <= b.x.1
        && a.x.1 >= b.x.0
        && a.y.0 <= b.y.1
        && a.y.1 >= b.y.0
        && a.z.0 <= b.z.1
        && a.z.1 >= b.z.0
}

fn is_included(a: &Cuboid, b: &Cuboid) -> bool {
    a.x.0 <= b.x.0
        && a.x.1 >= b.x.1
        && a.y.0 <= b.y.0
        && a.y.1 >= b.y.1
        && a.z.0 <= b.z.0
        && a.z.1 >= b.z.1
}

pub fn part1(input: String) {
    let rules = parse_input(&input);
    let mut cubes: HashSet<(isize, isize, isize)> = HashSet::new();
    for rule in rules {
        if !skip_rule(rule) {
            apply_rule(rule, &mut cubes);
        }
    }
    let res = cubes.iter().count();
    println!("Solution part 1: {:?} ", res);
}

fn subtract(a: Cuboid, b: Cuboid) -> Vec<Cuboid> {
    if !intersect(&a, &b) {
        return vec![a];
    }

    if is_included(&b, &a) {
        return vec![];
    }

    // get the points where to create the new cuboids
    let mut xs = vec![b.x.0, b.x.1]
        .into_iter()
        .filter(|&x| a.x.0 <= x && x <= a.x.1)
        .collect::<VecDeque<isize>>();
    xs.push_front(a.x.0);
    xs.push_back(a.x.1);

    let mut ys = vec![b.y.0, b.y.1]
        .into_iter()
        .filter(|&y| a.y.0 <= y && y <= a.y.1)
        .collect::<VecDeque<isize>>();
    ys.push_front(a.y.0);
    ys.push_back(a.y.1);

    let mut zs = vec![b.z.0, b.z.1]
        .into_iter()
        .filter(|&z| a.z.0 <= z && z <= a.z.1)
        .collect::<VecDeque<isize>>();
    zs.push_front(a.z.0);
    zs.push_back(a.z.1);

    let mut new_cuboids = Vec::new();
    for i in 0..xs.len() - 1 {
        for j in 0..ys.len() - 1 {
            for k in 0..zs.len() - 1 {
                new_cuboids.push(Cuboid {
                    x: (xs[i], xs[i + 1]),
                    y: (ys[j], ys[j + 1]),
                    z: (zs[k], zs[k + 1]),
                });
            }
        }
    }
    new_cuboids
        .into_iter()
        .filter(|cuboid| !is_included(&b, cuboid))
        .collect_vec()
}

pub fn part2(input: String) {
    let rules = parse_input(&input);
    let mut cuboids: Vec<Cuboid> = vec![];
    for rule in rules {
        // I always subtract the cuboids, then I add the new one if "on"
        cuboids = cuboids
            .into_iter()
            .map(|c| subtract(c, rule.cuboid))
            .flatten()
            .collect_vec();
        if rule.on {
            cuboids.push(rule.cuboid);
        }
    }
    let res: isize = cuboids
        .iter()
        .map(|cuboid| {
            (cuboid.x.1 - cuboid.x.0) * (cuboid.y.1 - cuboid.y.0) * (cuboid.z.1 - cuboid.z.0)
        })
        .sum();
    println!("Solution part 2: {:?}", res);
}

#[cfg(test)]
mod tests {

    use super::*;
    use itertools::Itertools;

    #[test]
    fn day22_part1_overlaps_1() {
        let range = (-54112, -39298);
        let overlap = overlaps(range);
        assert_eq!(false, overlap);
    }

    #[test]
    fn day22_part1_overlaps_2() {
        let range = (-16, 35);
        let overlap = overlaps(range);
        assert_eq!(true, overlap);
    }

    #[test]
    fn day22_part1_skip_1() {
        let rule: Rule = Rule {
            on: true,
            cuboid: Cuboid {
                x: (-54112, -39298),
                y: (-85059, -49293),
                z: (-27449, 7877),
            },
        };
        let skip = skip_rule(rule);
        assert_eq!(true, skip);
    }

    #[test]
    fn day22_part1_skip_2() {
        let rule: Rule = Rule {
            on: true,
            cuboid: Cuboid {
                x: (-16, 35),
                y: (-41, 10),
                z: (-47, 6),
            },
        };
        let skip = skip_rule(rule);
        assert_eq!(false, skip);
    }
}
