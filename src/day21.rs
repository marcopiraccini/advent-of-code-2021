use hashbrown::HashMap;
use itertools::Itertools;

fn parse_input(input: String) -> (usize, usize) {
    let start = input
        .lines()
        .map(|l| {
            let ll = l.split(": ").collect::<Vec<&str>>();
            ll[1].parse::<usize>().unwrap()
        })
        .collect_vec();
    (start[0], start[1])
}

pub fn part1(input: String) {
    let (mut p1, mut p2) = parse_input(input);
    let mut dice_val: usize = 0;
    let mut dice = || {
        dice_val += 1;
        if dice_val == 101 {
            dice_val = 1;
        }
        dice_val
    };

    let mut p1score = 0;
    let mut p2score = 0;
    let mut throws = 0;
    loop {
        p1 = p1 + dice() + dice() + dice();
        p1 = ((p1 - 1) % 10) + 1;
        throws += 3;
        p1score += p1;
        if p1score >= 1000 {
            break;
        }
        p2 = p2 + dice() + dice() + dice();
        p2 = ((p2 - 1) % 10) + 1;
        throws += 3;
        p2score += p2;
        if p2score >= 1000 {
            break;
        }
    }
    let loser = if p1score >= 1000 { p2score } else { p1score };
    println!("Solution part 1: {:?} ", loser * throws);
}

fn get_combs() -> Vec<(usize, usize, usize)> {
    let mut ret = Vec::new();
    for a in 1..4 {
        for b in 1..4 {
            for c in 1..4 {
                ret.push((a, b, c));
            }
        }
    }
    ret
}

fn quantum_play(
    p1: usize,
    p2: usize,
    score1: usize,
    score2: usize,
    cache: &mut HashMap<(usize, usize, usize, usize), (usize, usize)>,
    rolls: &Vec<(usize, usize, usize)>,
) -> (usize, usize) {
    let key = (p1, p2, score1, score2);
    let found = cache.get(&key);
    if found.is_some() {
        return *found.unwrap();
    }
    // The player that just playes is always player 2 (see below)
    if score2 >= 21 {
        return (0, 1);
    }
    let mut res = (0, 0);
    for roll in rolls {
        // We calculate only one player move, then we just swap the,
        let mut p11 = p1 + roll.0 + roll.1 + roll.2;
        p11 = ((p11 - 1) % 10) + 1;
        let score11 = score1 + p11;
        let (n1, n2) = quantum_play(p2, p11, score2, score11, cache, &rolls);
        // We need to swap the count!
        res.0 += n2;
        res.1 += n1;
    }
    cache.insert(key, res);
    return res;
}

pub fn part2(input: String) {
    let (p1, p2) = parse_input(input);
    let combs = get_combs();
    let mut cache = HashMap::new();
    let (w1, w2) = quantum_play(p1, p2, 0, 0, &mut cache, &combs);
    println!("Solution part 2: {:?} ", [w1, w2].iter().max().unwrap());
}
