const SIZE: usize = 5;

#[derive(Clone, Debug)]
struct Board {
    board: Vec<Vec<(usize, bool)>>,
    won: bool,
}

impl Board {
    fn init(lines: &[Vec<(usize, bool)>]) -> Board {
        let board: Vec<Vec<(usize, bool)>> = lines.to_vec();
        Board { board, won: false }
    }

    fn did_win(&self) -> bool {
        for r in 0..5 {
            if self.board[r][0].1
                && self.board[r][1].1
                && self.board[r][2].1
                && self.board[r][3].1
                && self.board[r][4].1
            {
                return true;
            }
        }
        for c in 0..5 {
            if self.board[0][c].1
                && self.board[1][c].1
                && self.board[2][c].1
                && self.board[3][c].1
                && self.board[4][c].1
            {
                return true;
            }
        }
        return false;
    }
    // return the solution if won, 0 otherwise
    fn draw(&mut self, n: usize) -> usize {
        for r in 0..5 {
            for c in 0..5 {
                if self.board[r][c].0 == n {
                    self.board[r][c] = (n, true);
                }
            }
        }
        if self.did_win() {
            self.won = true;
            let mut unmarked_sum = 0;
            for r in 0..5 {
                for c in 0..5 {
                    if !self.board[r][c].1 {
                        unmarked_sum += self.board[r][c].0;
                    }
                }
            }
            return n * unmarked_sum;
        }
        return 0;
    }
}

fn parse_drawns(input: &String) -> Vec<usize> {
    input
        .lines()
        .nth(0)
        .unwrap()
        .split(',')
        .map(|x| x.parse::<usize>().unwrap())
        .collect::<Vec<usize>>()
}

fn parse_boards(input: &String) -> Vec<Board> {
    let numbers = input
        .lines()
        .skip(1)
        .filter(|&x| !x.is_empty())
        .map(|x| {
            x.split(" ")
                .filter(|&x| !x.is_empty())
                .map(|y| (y.trim().parse::<usize>().unwrap(), false))
                .collect::<Vec<(usize, bool)>>()
        })
        .collect::<Vec<Vec<(usize, bool)>>>();
    let boards = numbers.chunks(SIZE).map(|x| Board::init(x)).collect();
    boards
}

pub fn part1(input: String) {
    let drawns = parse_drawns(&input);
    let mut boards = parse_boards(&input);
    'outer: for n in drawns {
        for i in 0..boards.len() {
            let board = &mut boards[i];
            let res = board.draw(n);
            if res > 0 {
                println!("Solution part 1: {:?}", res);
                break 'outer;
            }
        }
    }
}

pub fn part2(input: String) {
    let drawns = parse_drawns(&input);
    let mut boards = parse_boards(&input);
    let mut res = 0;
    for n in drawns {
        for i in 0..boards.len() {
            let board = &mut boards[i];
            if !board.won {
                res = board.draw(n);
            }
        }
    }
    println!("Solution part 2: {:?}", res);
}
