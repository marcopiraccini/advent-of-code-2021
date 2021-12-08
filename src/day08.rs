use itertools::Itertools;

fn parse_row(in_string: &str) -> (&str, &str) {
    let mut splitter = in_string.splitn(2, " | ");
    let start = splitter.next().unwrap();
    let end = splitter.next().unwrap();
    (start, end)
}

fn parse_input(input: &String) -> Vec<(Vec<&str>, Vec<&str>)> {
    input
        .lines()
        .map(|l| {
            let (patt, digit) = parse_row(l);
            let patts = patt.split_whitespace().collect::<Vec<_>>();
            let digits = digit.split_whitespace().collect::<Vec<_>>();
            (patts, digits)
        })
        .collect::<Vec<(Vec<&str>, Vec<&str>)>>()
}

pub fn part1(input: String) {
    let patterns = parse_input(&input);
    let res = patterns.iter().map(|(_, el)| el).fold(0, |acc, el| {
        let c = el
            .iter()
            .filter(|el| el.len() == 2 || el.len() == 4 || el.len() == 3 || el.len() == 7)
            .count();
        acc + c
    });
    println!("Solution part 1: {:?}", res);
}

// We check the single pattern (e.g. gebfc) on a perm.
// To do this, we map on the perm, the order and check if the digit is one of the below
fn match_digit(perm: &[char], pattern: &str) -> isize {
    let p = pattern
        .chars()
        .map(|c| perm[index(c)])
        .sorted()
        .collect::<String>();
    let digit = match p.as_str() {
        "abcdeg" => 0,
        "ab" => 1,
        "acdfg" => 2,
        "abcdf" => 3,
        "abef" => 4,
        "bcdef" => 5,
        "bcdefg" => 6,
        "abd" => 7,
        "abcdefg" => 8,
        "abcdef" => 9,
        _ => return -1,
    };
    digit
}

// We need to index a perm
fn index(ch: char) -> usize {
    match ch {
        'a' => 0,
        'b' => 1,
        'c' => 2,
        'd' => 3,
        'e' => 4,
        'f' => 5,
        'g' => 6,
        _ => panic!("unknown"),
    }
}

fn convert(perm: &[char], digits: &Vec<&str>) -> isize {
    digits
        .iter()
        .rev()
        .enumerate()
        // Convert digit to number in base 10
        .map(|(i, s)| match_digit(&perm, s) * (10 as isize).pow(i as u32))
        .sum()
}

pub fn part2(input: String) {
    let rules = parse_input(&input);
    let perms = "abcdefg".chars().permutations(7);
    let mut res = 0;
    for (patterns, digits) in rules {
        for perm in perms.clone() {
            let skip = patterns
                .iter()
                .map(|p| match_digit(&perm, p))
                .any(|f| f < 0);
            if !skip {
                res += convert(&perm, &digits);
            }
        }
    }

    println!("Solution part 2: {:?}", res);
}
