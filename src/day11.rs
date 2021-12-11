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

fn get_adiacent(x: usize, y: usize, sizex: usize, sizey: usize) -> Vec<(usize, usize)> {
    let mut ret = Vec::new();
    if x > 0 {
        if y > 0 {
            ret.push((x - 1, y - 1));
        }
        ret.push((x - 1, y));
        if y < sizey - 1 {
            ret.push((x - 1, y + 1));
        }
    }
    if y > 0 {
        ret.push((x, y - 1));
    }
    if y < sizey - 1 {
        ret.push((x, y + 1));
    }
    if x < sizex - 1 {
        if y > 0 {
            ret.push((x + 1, y - 1));
        }
        ret.push((x + 1, y));
        if y < sizey - 1 {
            ret.push((x + 1, y + 1));
        }
    }
    return ret;
}

fn apply_step(octs: &mut Vec<Vec<usize>>) -> usize {
    let sizex = octs[0].len();
    let sizey = octs.len();
    let mut flashes = 0;
    // First, the energy level of each octopus increases by 1.
    for x in 0..sizex {
        for y in 0..sizey {
            octs[y][x] += 1;
        }
    }
    // Then, any octopus with an energy level greater than 9 flashes.
    loop {
        let mut any_flashes = false;
        for x in 0..sizex {
            for y in 0..sizey {
                if octs[y][x] > 9 {
                    flashes += 1;
                    any_flashes = true;
                    let adj = get_adiacent(x, y, sizex, sizey);
                    for el in adj {
                        if octs[el.1][el.0] > 0 {
                            octs[el.1][el.0] += 1;
                        }
                    }
                    octs[y][x] = 0;
                }
            }
        }
        if !any_flashes {
            break;
        }
    }
    flashes
}

pub fn part1(input: String) {
    let inp = parse_input(&input);
    let mut octs = inp.clone();
    let mut res = 0;
    for _ in 0..100 {
        let step = apply_step(&mut octs);
        res += step;
    }

    println!("Solution part 1: {:?}", res);
}

fn is_done(octs: &mut Vec<Vec<usize>>) -> bool {
    let sizex = octs[0].len();
    let sizey = octs.len();
    let val = octs[0][0];
    for x in 0..sizex {
        for y in 0..sizey {
            if octs[y][x] != val {
                return false;
            }
        }
    }
    return true;
}

pub fn part2(input: String) {
    let inp = parse_input(&input);
    let mut octs = inp.clone();
    let mut res = 0;
    loop {
        apply_step(&mut octs);
        res += 1;
        if is_done(&mut octs) {
            println!("Solution part 2: {:?}", res);
            break;
        }
    }
}
