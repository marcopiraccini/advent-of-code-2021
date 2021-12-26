use core::mem::swap;

fn convert(inp: Vec<isize>) -> usize {
    inp.iter()
        .rev()
        .enumerate()
        .map(|(i, &x)| x as usize * 10usize.pow(i as u32))
        .sum()
}

pub fn part1(input: String) {
    let mut res = vec![0; 14];
    let mut stack = vec![];
    let inp = input
        .split("inp w\n")
        .skip(1)
        .map(|s| s.lines().collect::<Vec<_>>());

    for (curr, ins) in inp.enumerate() {
        // div can be 1 or 26
        let div = ins[3]
            .strip_prefix("div z ")
            .unwrap()
            .parse::<isize>()
            .unwrap();
        if div == 1 {
            let f = ins[14]
                .strip_prefix("add y ")
                .unwrap()
                .parse::<isize>()
                .unwrap();
            stack.push((curr, f));
        } else if div == 26 {
            let (mut prev, x) = stack.pop().unwrap();
            let mut diff = x + ins[4]
                .strip_prefix("add x ")
                .unwrap()
                .parse::<isize>()
                .unwrap();
            let mut curr = curr;
            // we have to change the sign if negative
            if diff < 0 {
                // https://doc.rust-lang.org/stable/core/mem/fn.swap.html
                swap(&mut curr, &mut prev);
                diff = -diff;
            }
            res[curr] = 9;
            res[prev] = 9 - diff;
        }
    }

    println!("Solution part 1: {:?} ", convert(res));
}

pub fn part2(input: String) {
    let mut res = vec![0; 14];
    let mut stack = vec![];
    let inp = input
        .split("inp w\n")
        .skip(1)
        .map(|s| s.lines().collect::<Vec<_>>());

    for (curr, ins) in inp.enumerate() {
        // div can be 1 or 26
        let div = ins[3]
            .strip_prefix("div z ")
            .unwrap()
            .parse::<isize>()
            .unwrap();
        if div == 1 {
            let f = ins[14]
                .strip_prefix("add y ")
                .unwrap()
                .parse::<isize>()
                .unwrap();
            stack.push((curr, f));
        } else if div == 26 {
            let (mut prev, x) = stack.pop().unwrap();
            let mut diff = x + ins[4]
                .strip_prefix("add x ")
                .unwrap()
                .parse::<isize>()
                .unwrap();
            let mut curr = curr;
            // we have to change the sign if negative
            if diff < 0 {
                // https://doc.rust-lang.org/stable/core/mem/fn.swap.html
                swap(&mut curr, &mut prev);
                diff = -diff;
            }
            res[curr] = 1 + diff;
            res[prev] = 1;
        }
    }

    println!("Solution part 2: {:?} ", convert(res));
}
