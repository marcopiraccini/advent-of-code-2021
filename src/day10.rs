use std::collections::VecDeque;

fn parse_input(input: &String) -> Vec<VecDeque<char>> {
    input
        .lines()
        .map(|l| l.chars().collect::<VecDeque<char>>())
        .collect::<Vec<_>>()
}

fn closing(open: char, close: char) -> bool {
    match open {
        '(' => close == ')',
        '[' => close == ']',
        '{' => close == '}',
        '<' => close == '>',
        _ => panic!("unknonw"),
    }
}

fn score(ch: char) -> usize {
    match ch {
        ')' => 3,
        ']' => 57,
        '}' => 1197,
        '>' => 25137,
        _ => panic!("unknown"),
    }
}

pub fn part1(input: String) {
    let lines = parse_input(&input);
    let mut ret = 0;
    for line in &lines {
        let mut stack: VecDeque<char> = VecDeque::new();
        for ch in line {
            if *ch == '(' || *ch == '[' || *ch == '{' || *ch == '<' {
                stack.push_back(*ch);
            }
            if *ch == ')' || *ch == ']' || *ch == '}' || *ch == '>' {
                let last = stack.pop_back().unwrap();
                if !closing(last, *ch) {
                    ret += score(*ch);
                }
            }
        }
    }
    println!("Solution part 1: {:?}", ret);
}

fn score_stack(stack: &VecDeque<char>) -> usize {
    stack.into_iter().rev().fold(0, |acc, c| {
        let ch_score = match c {
            '(' => 1,
            '[' => 2,
            '{' => 3,
            '<' => 4,
            _ => panic!("unknonw"),
        };
        return acc * 5 + ch_score;
    })
}

pub fn part2(input: String) {
    let lines = parse_input(&input);
    let mut ret: Vec<usize> = Vec::new();
    for line in &lines {
        let mut stack: VecDeque<char> = VecDeque::new();
        let mut corrupted = false;
        for ch in line {
            if *ch == '(' || *ch == '[' || *ch == '{' || *ch == '<' {
                stack.push_back(*ch);
            }
            if *ch == ')' || *ch == ']' || *ch == '}' || *ch == '>' {
                let last = stack.pop_back().unwrap();
                if !closing(last, *ch) {
                    corrupted = true;
                }
            }
        }
        if stack.len() != 0 && !corrupted {
            let score = score_stack(&stack);
            ret.push(score);
        }
        ret.sort();
    }
    println!("Solution part 2: {:?}", ret[ret.len() / 2]);
}
