fn parse_input(mut input: String) -> Vec<usize> {
    input.pop(); // removes the trailing \n
    input
        .split(',')
        .map(|x| x.parse::<usize>().unwrap())
        .collect::<Vec<usize>>()
}

pub fn part1(input: String) {
    let mut crabs = parse_input(input.clone());
    crabs.sort();
    let median = crabs[crabs.len() / 2];

    let res = crabs
        .iter()
        .fold(0, |acc, c| acc + isize::abs(*c as isize - median as isize));
    println!("Solution part 1: {:?}", res);
}

fn cost(n: isize) -> usize {
    let mut res = 0;
    for i in 0..isize::abs(n) + 1 {
        res += i;
    }
    res as usize
}

pub fn part2(input: String) {
    let mut crabs = parse_input(input.clone());
    crabs.sort();
    let max = *(crabs.iter().max().unwrap());
    let mut min = 10000000000;
    // Yes, this can be optimized, but is not really necessary in this case...
    for n in 0..max {
        let total_cost = crabs
            .iter()
            .fold(0, |acc, c| acc + cost(*c as isize - n as isize));
        if total_cost < min {
            min = total_cost;
        }
    }
    println!("Solution part 2: {:?}", min);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn day07_part2() {
        let mut res = cost(16 - 5);
        assert_eq!(res, 66);
    }
}
