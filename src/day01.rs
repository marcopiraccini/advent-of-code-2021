pub fn part1(input: String) {
    let vals = input
        .lines()
        .map(|x| x.parse::<u32>().unwrap())
        .collect::<Vec<u32>>();

    let mut res = -1; // first one doen't count
    let mut prev = 0;
    for el in vals {
        if el > prev {
            res = res + 1;
        }
        prev = el;
    }
    println!("Solution part 1: {:?}", res);
}

pub fn part2(input: String) {
    let vals = input
        .lines()
        .map(|x| x.parse::<u32>().unwrap())
        .collect::<Vec<u32>>();
    let mut res = -1;
    let mut prev = 0;
    // Can be done with a fold on an enumerate with a tuple accumulator, but why??
    for i in 2..vals.len() {
        let window = (&vals[i - 2..=i]).iter().sum();
        if window > prev {
            res = res + 1;
        }
        prev = window;
    }
    println!("Solution part 2: {:?}", res);
}
