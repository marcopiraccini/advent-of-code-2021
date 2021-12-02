fn parse_input(input: &String) -> Vec<(&str, u32)> {
    input
        .lines()
        .map(|l| {
            let l = l.split(' ').collect::<Vec<&str>>();
            (l[0], l[1].parse::<u32>().unwrap())
        })
        .collect::<Vec<(&str, u32)>>()
}

pub fn part1(input: String) {
    let commands = parse_input(&input);
    let res = commands.iter().fold((0, 0), |acc, c| match c.0 {
        "forward" => (acc.0 + c.1, acc.1),
        "up" => (acc.0, acc.1 - c.1),
        "down" => (acc.0, acc.1 + c.1),
        _ => panic!("unknown"),
    });
    println!("Solution part 1: {:?}", res.0 * res.1);
}

pub fn part2(input: String) {
    let commands = parse_input(&input);
    let res = commands.iter().fold((0, 0, 0), |pos, c| match c.0 {
        "forward" => (pos.0 + c.1, pos.1 + pos.2 * c.1, pos.2),
        "up" => (pos.0, pos.1, pos.2 - c.1),
        "down" => (pos.0, pos.1, pos.2 + c.1),
        _ => panic!("unknown"),
    });
    println!("Solution part 2: {:?}", res.0 * res.1);
}
