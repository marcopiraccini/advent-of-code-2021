const SIZE: usize = 10;

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

fn get_adiacent(xx: usize, yy: usize) -> Vec<(usize, usize)> {
    let mut ret = Vec::new();
    let x = xx as isize;
    let y = yy as isize;
    ret.push((x - 1, y - 1));
    ret.push((x - 1, y));
    ret.push((x - 1, y + 1));
    ret.push((x, y - 1));
    ret.push((x, y + 1));
    ret.push((x + 1, y - 1));
    ret.push((x + 1, y));
    ret.push((x + 1, y + 1));
    ret.into_iter()
        .filter(|el| el.0 >= 0 && el.0 < SIZE as isize && el.1 >= 0 && el.1 < SIZE as isize)
        .map(|el| (el.0 as usize, el.1 as usize))
        .collect::<Vec<_>>()
}

fn apply_step(octs: &mut Vec<Vec<usize>>) -> usize {
    let mut flashes = 0;
    // First, the energy level of each octopus increases by 1.
    for x in 0..SIZE {
        for y in 0..SIZE {
            octs[y][x] += 1;
        }
    }
    // Then, any octopus with an energy level greater than 9 flashes.
    loop {
        let mut any_flashes = false;
        for x in 0..SIZE {
            for y in 0..SIZE {
                if octs[y][x] > 9 {
                    flashes += 1;
                    any_flashes = true;
                    let adj = get_adiacent(x, y);
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
    let mut octs = parse_input(&input);
    let mut res = 0;
    for _ in 0..100 {
        res += apply_step(&mut octs);
    }
    println!("Solution part 1: {:?}", res);
}

fn is_done(octs: &mut Vec<Vec<usize>>) -> bool {
    let val = octs[0][0];
    for x in 0..SIZE {
        for y in 0..SIZE {
            if octs[y][x] != val {
                return false;
            }
        }
    }
    return true;
}

pub fn part2(input: String) {
    let mut octs = parse_input(&input);
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
