const SIZE: usize = 1000; // I know, I'm lazy
fn get_tuple(el: &Vec<&str>) -> (usize, usize) {
    (
        el[0].parse::<usize>().unwrap(),
        el[1].parse::<usize>().unwrap(),
    )
}

fn parse_input(input: String) -> Vec<((usize, usize), (usize, usize))> {
    input
        .lines()
        .map(|p| {
            let pp = p.split(" -> ").collect::<Vec<&str>>();
            let start = pp[0].split(",").collect::<Vec<&str>>();
            let end = pp[1].split(",").collect::<Vec<&str>>();
            (get_tuple(&start), get_tuple(&end))
        })
        .collect()
}

fn get_range(start: usize, end: usize) -> Vec<usize> {
    if start < end {
        (start..end + 1).collect::<Vec<usize>>()
    } else {
        (end..start + 1).rev().collect::<Vec<usize>>()
    }
}

fn check_vents(vents: &Vec<((usize, usize), (usize, usize))>, do_diagonal: bool) -> usize {
    let mut map = vec![vec![0; SIZE]; SIZE];
    let mut res = 0;
    for el in vents {
        let (start, end) = el;
        if start.0 != end.0 && start.1 != end.1 {
            if do_diagonal {
                // part 2
                let rangex = get_range(start.0, end.0);
                let rangey = get_range(start.1, end.1);
                for i in 0..rangex.len() {
                    map[rangex[i]][rangey[i]] = map[rangex[i]][rangey[i]] + 1;
                    if map[rangex[i]][rangey[i]] == 2 {
                        res += 1
                    }
                }
            } else {
                continue;
            }
        } else {
            if start.0 != end.0 {
                let range = get_range(start.0, end.0);
                for x in range {
                    map[x][start.1] = map[x][start.1] + 1;
                    if map[x][start.1] == 2 {
                        res += 1
                    }
                }
            }
            if start.1 != end.1 {
                let range = get_range(start.1, end.1);
                for y in range {
                    map[start.0][y] = map[start.0][y] + 1;
                    if map[start.0][y] == 2 {
                        res += 1
                    }
                }
            }
        }
    }
    res
}

pub fn part1(input: String) {
    let vents = parse_input(input);
    println!("Solution part 1: {:?}", check_vents(&vents, false));
}

pub fn part2(input: String) {
    let vents = parse_input(input);
    println!("Solution part 2: {:?}", check_vents(&vents, true));
}
