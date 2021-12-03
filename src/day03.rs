const LENGTH: usize = 12;

fn parse_input(input: &String) -> Vec<usize> {
    input
        .lines()
        .map(|x| usize::from_str_radix(x, 2).unwrap())
        .collect::<Vec<usize>>()
}

pub fn part1(input: String) {
    let inp = parse_input(&input);
    let mut gamma = 0;
    let mut epsilon = 0;
    for n in 0..LENGTH {
        let mask = 1 << n;
        let mut ones = 0;
        for el in &inp {
            if el & mask != 0 {
                ones += 1
            }
        }
        if ones > inp.len() / 2 {
            gamma |= mask
        } else {
            epsilon |= mask
        }
    }
    println!("Solution part 1: {:?}", epsilon * gamma);
}

fn get_param(values: &Vec<usize>, ox: bool) -> usize {
    let mut inp = values.clone();
    fn retain(x: usize, mask: usize, ox: bool) -> bool {
        if ox {
            x & mask != 0
        } else {
            x & mask == 0
        }
    }

    for n in (0..LENGTH).rev() {
        let mask = 1 << n;
        let mut ones = 0;
        for el in &inp {
            if el & mask != 0 {
                ones += 1
            }
        }
        let zeros = inp.len() - ones;
        if ones >= zeros {
            inp.retain(|&x| retain(x, mask, ox));
        } else {
            inp.retain(|&x| !retain(x, mask, ox));
        }
        if inp.len() == 1 {
            break;
        }
    }
    *inp.get(0).unwrap()
}

pub fn part2(input: String) {
    let inp = parse_input(&input);
    let ox = get_param(&inp, true);
    let co2 = get_param(&inp, false);
    println!("Solution part 2: {:?}", ox * co2);
}
