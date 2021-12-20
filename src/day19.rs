use hashbrown::HashSet;
use itertools::Itertools;

fn parse_input(input: String) -> Vec<Vec<[isize; 3]>> {
    input
        .split("\n\n")
        .map(|s| {
            s.lines()
                .skip(1)
                .map(|l| {
                    let coords = l
                        .split(",")
                        .map(|x| x.parse::<isize>().unwrap())
                        .collect_vec();
                    [coords[0], coords[1], coords[2]]
                })
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>()
}

fn rotate([x, y, z]: [isize; 3], r: usize) -> [isize; 3] {
    match r {
        0 => [x, y, z],
        1 => [x, z, -y],
        2 => [x, -y, -z],
        3 => [x, -z, y],
        4 => [y, x, -z],
        5 => [y, z, x],
        6 => [y, -x, z],
        7 => [y, -z, -x],
        8 => [z, x, y],
        9 => [z, y, -x],
        10 => [z, -x, -y],
        11 => [z, -y, x],
        12 => [-x, y, -z],
        13 => [-x, z, y],
        14 => [-x, -y, z],
        15 => [-x, -z, -y],
        16 => [-y, x, z],
        17 => [-y, z, -x],
        18 => [-y, -x, -z],
        19 => [-y, -z, x],
        20 => [-z, x, -y],
        21 => [-z, y, x],
        22 => [-z, -x, y],
        23 => [-z, -y, -x],
        _ => panic!(),
    }
}

fn arrange(beacons: &mut HashSet<[isize; 3]>, scan: &[[isize; 3]]) -> Option<[isize; 3]> {
    for rot in 0..24 {
        // Apply the rotation
        let rotated = scan.iter().map(|&v| rotate(v, rot)).collect::<Vec<_>>();
        // https://docs.rs/itertools/0.10.3/itertools/trait.Itertools.html#method.cartesian_product
        // Calculate the distances
        let distances = beacons
            .iter()
            .cartesian_product(&rotated)
            .map(|([x1, y1, z1], [x2, y2, z2])| [x1 - x2, y1 - y2, z1 - z2]);

        for [dx, dy, dz] in distances {
            let translated = rotated.iter().map(|[x, y, z]| [x + dx, y + dy, z + dz]);
            // 12 beacons detected
            if translated.clone().filter(|v| beacons.contains(v)).count() >= 12 {
                beacons.extend(translated);
                return Some([dx, dy, dz]);
            }
        }
    }
    None
}

pub fn part1(input: String) {
    let mut scanners = parse_input(input.clone());
    let mut beacons = scanners.remove(0).into_iter().collect::<HashSet<_>>();
    let mut dists = Vec::new();
    // We arrange the sanners one by one.
    while !scanners.is_empty() {
        for i in (0..scanners.len()).rev() {
            if let Some(d) = arrange(&mut beacons, &scanners[i]) {
                dists.push(d);
                scanners.remove(i);
            }
        }
    }
    let p1 = beacons.len();
    println!("Solution part 1: {:?} ", p1);
    let p2 = dists
        .iter()
        .tuple_combinations()
        .map(|([x1, y1, z1], [x2, y2, z2])| (x1 - x2).abs() + (y1 - y2).abs() + (z1 - z2).abs())
        .max()
        .unwrap();
    println!("Solution part 2: {:?} ", p2);
}
