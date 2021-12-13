#[derive(Clone, Debug)]
pub struct Map {
    map: Vec<Vec<usize>>,
    pub width: usize,
    pub height: usize,
}

impl Map {
    pub fn init(coords: &Vec<(usize, usize)>) -> Map {
        // We can probably do better here, but dosen't matter
        let sizex = coords.iter().map(|el| el.0).max().unwrap() + 10;
        let sizey = coords.iter().map(|el| el.1).max().unwrap() + 10;
        let mut map = vec![vec![0; sizex]; sizey];
        for el in coords {
            map[el.1][el.0] = 1;
        }
        Map {
            width: sizex,
            height: sizey,
            map,
        }
    }

    pub fn foldy(&mut self, pivot: usize) {
        for i in 0..pivot {
            for k in 0..self.width {
                if self.map[2 * pivot - i][k] == 1 {
                    self.map[i][k] = 1;
                }
            }
        }
        self.map.truncate(pivot);
        self.height = self.map.len();
    }

    pub fn foldx(&mut self, pivot: usize) {
        for i in 0..pivot {
            for k in 0..self.map.len() {
                if self.map[k][2 * pivot - i] == 1 {
                    self.map[k][i] = 1;
                }
            }
        }

        for k in 0..self.map.len() {
            self.map[k].truncate(pivot);
        }
        self.width = self.map[0].len();
    }

    pub fn count(&self) -> usize {
        let mut res = 0;
        for y in 0..self.map.len() {
            for x in 0..self.map[0].len() {
                res += self.map[y][x];
            }
        }
        res
    }

    pub fn print(&self) {
        let printable_map = self
            .map
            .iter()
            .map(|row| {
                let row_str = row.iter().fold(String::from(""), |mut acc, el| {
                    if *el > 0 {
                        acc.push_str("#");
                    } else {
                        acc.push_str(" ");
                    }
                    return acc;
                });

                return row_str;
            })
            .collect::<Vec<String>>();

        for line in printable_map {
            println!("{:?}", line);
        }
    }
}

fn parse_input(input: &String) -> (Vec<(usize, usize)>, Vec<(usize, usize)>) {
    let coords = input
        .lines()
        .filter(|l| !l.starts_with("fold") && !l.is_empty())
        .map(|l| {
            let l = l.split(',').collect::<Vec<&str>>();
            (
                l[0].parse::<usize>().unwrap(),
                l[1].parse::<usize>().unwrap(),
            )
        })
        .collect::<Vec<(usize, usize)>>();

    let folds = input
        .lines()
        .filter(|l| l.starts_with("fold along"))
        .map(|l| {
            let l = l.split("=").collect::<Vec<&str>>();
            let dim = l[0];
            let val = l[1].parse::<usize>().unwrap();
            match dim {
                "fold along x" => return (val, 0_usize),
                "fold along y" => return (0_usize, val),
                _ => panic!(),
            };
        })
        .collect::<Vec<(usize, usize)>>();
    (coords, folds)
}

pub fn part1(input: String) {
    let (coords, folds) = parse_input(&input);
    let mut map = Map::init(&coords);
    for (i, fold) in folds.iter().enumerate() {
        if fold.0 > 0 {
            map.foldx(fold.0);
        } else {
            map.foldy(fold.1);
        }
        if i == 0 {
            println!("Solution part 1: {:?}", map.count());
        }
    }
    map.print();
}
