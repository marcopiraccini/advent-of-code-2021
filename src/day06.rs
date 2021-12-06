fn parse_input(mut input: String) -> Vec<usize> {
    input.pop(); // removes the trailing \n
    let f = input
        .split(',')
        .map(|x| x.parse::<usize>().unwrap())
        .collect::<Vec<usize>>();
    f.iter().fold(vec![0; 9], |mut acc, f| {
        acc[*f] += 1;
        return acc;
    })
}

fn run_gen(f: Vec<usize>, gen: usize) -> usize {
    let mut fish = f.clone();
    for _ in 0..gen {
        let mut new_gen = vec![0; 9];
        for age in 0..9 {
            let n = fish[age];
            if age == 0 {
                // if age is 0, reset one countdown for a fish an create a new one with countdown 8
                new_gen[6] += n;
                new_gen[8] += n;
            } else {
                // ... otherwise, simply proceed with the countdown
                new_gen[age - 1] += n;
            }
        }
        fish = new_gen
    }
    return fish.iter().sum();
}

pub fn part1(input: String) {
    let fish = parse_input(input.clone());
    let res = run_gen(fish, 80);
    println!("Solution part 1: {:?}", res);
}

pub fn part2(input: String) {
    let fish = parse_input(input.clone());
    let res = run_gen(fish, 256);
    println!("Solution part 2: {:?}", res);
}
