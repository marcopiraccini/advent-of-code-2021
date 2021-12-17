use itertools::Itertools;
use std::collections::HashSet;

fn parse_input(input: String) -> ((isize, isize), (isize, isize)) {
    let s2 = input.replace("target area: ", "");
    let s3 = s2.split(',').collect_vec();
    let xp = s3[0].replace("x=", "");
    let yp = s3[1].replace("y=", "");
    let sx = xp.split("..").collect_vec();
    let sy = yp.split("..").collect_vec();
    let x1 = sx[0].trim().parse::<isize>().unwrap();
    let x2 = sx[1].trim().parse::<isize>().unwrap();
    let y1 = sy[0].trim().parse::<isize>().unwrap();
    let y2 = sy[1].trim().parse::<isize>().unwrap();
    ((x1, x2), (y1, y2))
}

fn step(current: ((isize, isize), (isize, isize))) -> ((isize, isize), (isize, isize)) {
    let ((px, py), (vx, vy)) = current;
    let new_vx = if vx == 0 {
        0
    } else if vx > 0 {
        vx - 1
    } else {
        vx + 1
    };
    ((px + vx, py + vy), (new_vx, vy - 1))
}

fn hit(pos: (isize, isize), target: ((isize, isize), (isize, isize))) -> bool {
    let tx = target.0;
    let ty = target.1;
    if tx.0 <= pos.0 && pos.0 <= tx.1 && ty.0 <= pos.1 && pos.1 <= ty.1 {
        true
    } else {
        false
    }
}

pub fn part1(input: String) {
    let target = parse_input(input);
    let initial_position = (0, 0);
    const LOWER_VX: isize = 0;
    const HIGHER_VX: isize = 300;
    const LOWER_VY: isize = -300;
    const HIGHER_VY: isize = 300;
    let mut res = -100;

    for vx in LOWER_VX..HIGHER_VX {
        let mut max_y = LOWER_VY;
        for vy in LOWER_VY..HIGHER_VY {
            let mut current = (initial_position, (vx, vy));
            loop {
                current = step(current);
                if current.0 .1 > max_y {
                    max_y = current.0 .1;
                }
                if hit(current.0, target) {
                    if res < max_y {
                        res = max_y;
                    }
                    break;
                }
                if current.0 .1 < target.1 .1 {
                    break;
                }
            }
        }
    }

    println!("Solution part 1: {:?} ", res);
}

pub fn part2(input: String) {
    let target = parse_input(input);
    let initial_position = (0, 0);
    // Here I just played with ranges until I found the correct answer :)
    const LOWER_VX: isize = 0;
    const HIGHER_VX: isize = 400;
    const LOWER_VY: isize = -300;
    const HIGHER_VY: isize = 400;

    let mut res = HashSet::new();

    for vx in LOWER_VX..HIGHER_VX {
        for vy in LOWER_VY..HIGHER_VY {
            let mut current = (initial_position, (vx, vy));
            loop {
                current = step(current);
                if hit(current.0, target) {
                    res.insert((vx, vy));
                    break;
                }
                if current.0 .1 < target.1 .1 {
                    break;
                }
            }
        }
    }

    println!("Solution part 2: {:?} ", res.len());
}
