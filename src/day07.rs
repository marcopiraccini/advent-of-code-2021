fn parse_input(mut input: String) -> Vec<isize> {
    input.pop(); // removes the trailing \n
    input
        .split(',')
        .map(|x| x.parse::<isize>().unwrap())
        .collect::<Vec<isize>>()
}

pub fn part1(input: String) {
    let mut crabs = parse_input(input.clone());
    crabs.sort();
    let median = crabs[crabs.len() / 2];
    let res = crabs.iter().fold(0, |acc, &c| acc + isize::abs(c - median));
    println!("Solution part 1: {:?}", res);
}

fn cost(n: isize) -> isize {
    let mut res = 0;
    for i in 0..isize::abs(n) + 1 {
        res += i;
    }
    res as isize
}

pub fn part2(input: String) {
    let mut crabs = parse_input(input.clone());
    crabs.sort();
    let max = *(crabs.iter().max().unwrap());
    let mut min = isize::MAX;
    // Yes, this can be optimized, but is not really necessary in this case...
    for n in 0..max {
        let total_cost = crabs.iter().fold(0, |acc, &c| acc + cost(c - n));
        if total_cost < min {
            min = total_cost;
        }
    }
    println!("Solution part 2: {:?}", min);
}

#[cfg(test)]
mod tests {
    #[test]
    fn day07_part2() {
        assert_eq!(super::cost(16 - 5), 66);
    }
}
