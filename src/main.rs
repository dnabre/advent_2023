use std::collections::{BTreeSet, HashMap, HashSet, VecDeque};
use std::fmt::{Display, Formatter};
use std::time::Instant;

use advent_2023::{Direction, parse_grid};

/*
    Advent of Code 2023: Day 23
        part1 answer:   2502
        part2 answer:


*/
const ANSWER: (&str, &str) = ("2502", "6726");

fn main() {
    let _filename_test = "data/day23/test_input_01.txt";
    let _filename_test2 = "data/day23/test_input_02.txt";

    let filename_part1 = "data/day23/part1_input.txt";
    let filename_part2 = "data/day23/part2_input.txt";

    println!("Advent of Code, Day 23");
    println!("    ---------------------------------------------");
    let start1 = Instant::now();
    let answer1 = part1(filename_part1);
    let duration1 = start1.elapsed();

    println!("\t Part 1: {:14} time: {:?}", answer1, duration1);
    if ANSWER.0 != answer1 {
        println!("\t\t ERROR: Answer is WRONG. Got: {}, Expected {}", answer1, ANSWER.0);
    }

    let start2 = Instant::now();
    let answer2 = part2(filename_part2);
    let duration2 = start2.elapsed();

    println!("\t Part 2: {:14} time: {:?}", answer2, duration2);
    if ANSWER.1 != answer2 {
        println!("\t\t ERROR: Answer is WRONG. Got: {}, Expected {}", answer2, ANSWER.1);
    }
    println!("    ---------------------------------------------");
}


#[derive(Debug, PartialOrd, Ord, PartialEq, Eq, Hash, Clone, Copy)]
struct Coord {
    row: usize,
    col: usize,
}
impl Coord {
    fn neighbors(&self, grid: &Vec<Vec<Tile>>) -> Vec<Coord> {
        let mut res = Vec::new();
        let rows = grid.len();
        let cols = grid[0].len();

        if self.row > 0 {
            let pos = Coord {
                row: self.row - 1,
                col: self.col,
            };
            let tile = grid[pos.row][pos.col];
            let possible = match tile {
                Tile::Open => true,
                Tile::Slope(Direction::Up) => true,
                _ => false
            };
            if possible {
                res.push(pos);
            }
        }

        if self.row < rows - 1 {
            let pos = Coord {
                row: self.row + 1,
                col: self.col,
            };
            let tile = grid[pos.row][pos.col];
            let possible = match tile {
                Tile::Open => true,
                Tile::Slope(Direction::Down) => true,
                _ => false
            };
            if possible {
                res.push(pos);
            }
        }

        if self.col > 0 {
            let pos = Coord {
                row: self.row,
                col: self.col - 1,
            };
            let tile = grid[pos.row][pos.col];
            let possible = match tile {
                Tile::Open => true,
                Tile::Slope(Direction::Left) => true,
                _ => false,
            };
            if possible {
                res.push(pos);
            }
        }

        if self.col < cols - 1 {
            let pos = Coord {
                row: self.row,
                col: self.col + 1,
            };
            let tile = grid[pos.row][pos.col];
            let possible = match tile {
                Tile::Open => true,
                Tile::Slope(Direction::Right) => true,
                _ => false
            };
            if possible {
                res.push(pos);
            }
        }

        return res;
    }
    fn neighbours2(self, grid: &Vec<Vec<Tile>>) -> impl Iterator<Item=Self> + '_ {
        let rows = grid.len();
        let cols = grid[0].len();

        let up = if self.row > 0 {
            Some(Self {
                row: self.row - 1,
                col: self.col,
            })
        } else {
            None
        };

        let down = if self.row < rows - 1 {
            Some(Self {
                row: self.row + 1,
                col: self.col,
            })
        } else {
            None
        };

        let left = if self.col > 0 {
            Some(Self {
                row: self.row,
                col: self.col - 1,
            })
        } else {
            None
        };

        let right = if self.col < cols - 1 {
            Some(Self {
                row: self.row,
                col: self.col + 1,
            })
        } else {
            None
        };

        [up, down, left, right].into_iter().filter_map(|pos| pos).filter(|pos| grid[pos.row][pos.col] != Tile::Rock)
    }
}




#[derive(PartialEq, Eq, Hash, Clone, Copy)]
enum Tile {
    Rock,
    Open,
    Slope(advent_2023::Direction),

}

impl Display for Tile {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", match self {
            Tile::Rock => { '#' }
            Tile::Open => { '.' }
            Tile::Slope(d) => {
                match d {
                    Direction::Up => { '^' }
                    Direction::Down => { 'v' }
                    Direction::Left => { '<' }
                    Direction::Right => { '>' }
                }
            }
        })
    }
}


const START_POINT: Coord = Coord { row: 0, col: 1 };

#[derive(Debug, PartialEq, Eq, Hash, Clone)]
struct State {
    pos: Coord,
    distance: usize,
    visited_squares: BTreeSet<Coord>,
}


fn part1(input_file: &str) -> String {
    let lines = advent_2023::file_to_lines(input_file);
    let grid = parse_grid(&lines);
    let grid = advent_2023::convert_grid_using(&grid, |ch| parse_tile(ch));
    let start = START_POINT;

    let max_row = grid.len();
    let max_col = grid[0].len();
    let mut end = Coord { row: max_row - 1, col: max_col - 1 };
    let mut ch = &grid[end.row][end.col];
    while *ch != Tile::Open {
        end.col -= 1;
        ch = &grid[end.row][end.col];
    }


    let mut q: VecDeque<(Coord, usize, HashSet<Coord>)> = VecDeque::new();
    let mut max = 0;
    q.push_back((start, 0, HashSet::from([start])));

    while let Some((pos, cost, mut seen)) = q.pop_front() {
        if pos == end {
            max = cost.max(max);
            continue;
        }
        for n in pos.neighbors(&grid) {
            if seen.insert(n) {
                q.push_back((n, cost + 1, seen.clone()));
            }
        }
    }


    let answer = max;
    return answer.to_string();
}

fn part2(input_file: &str) -> String {
    let lines = advent_2023::file_to_lines(input_file);
    let grid = parse_grid(&lines);
    let grid = advent_2023::convert_grid_using(&grid, |ch| parse_tile(ch));
    let start = START_POINT;

    let max_row = grid.len();
    let max_col = grid[0].len();
    let mut end = Coord { row: max_row - 1, col: max_col - 1 };
    let mut ch = &grid[end.row][end.col];
    while *ch != Tile::Open {
        end.col -= 1;
        ch = &grid[end.row][end.col];
    }

    let mut points = all_forks(&grid);
    points.insert(start);
    points.insert(end);


    let mut points = all_forks(&grid);
    points.insert(start);
    points.insert(end);

    let costmap = crate::costmap(&points, &grid);

    return longest(start, end, &costmap).to_string();
}



fn all_forks(grid: &Vec<Vec<Tile>>) -> HashSet<Coord> {
    let mut res = HashSet::new();

    for row in 0..grid.len() {
        for col in 0..grid[0].len() {
            let pos = Coord { row, col };
            let tile = grid[pos.row][pos.col];
            if tile != Tile::Rock && pos.neighbours2(grid).count() > 2 {
                res.insert(pos);
            }
        }
    }

    res
}

fn costmap(points: &HashSet<Coord>, grid: &Vec<Vec<Tile>>) -> HashMap<Coord, HashMap<Coord, usize>> {
    let initial = HashMap::from_iter(points.iter().map(|node| (*node, HashMap::new())));
    points.iter().fold(initial, |mut acc, point| {
        let mut q: VecDeque<(Coord, usize)> = VecDeque::new();
        let mut seen: HashSet<Coord> = HashSet::new();
        q.push_back((*point, 0));
        while let Some((pos, cost)) = q.pop_front() {
            if points.contains(&pos) && cost != 0 {
                *acc.entry(*point).or_default().entry(pos).or_default() = cost;
                continue;
            }
            for n in pos.neighbours2(grid) {
                if seen.insert(n) {
                    q.push_back((n, cost + 1));
                }
            }
            seen.insert(pos);
        }
        return   acc;
    })
}

fn longest(from: Coord, to: Coord, grid: &HashMap<Coord, HashMap<Coord, usize>>) -> usize {
    let mut q = VecDeque::new();
    let mut max = 0;

    q.push_back((from, 0, HashSet::from([from])));

    while let Some((pos, cost, seen)) = q.pop_front() {
        if pos == to {
            max = cost.max(max);
            continue;
        }

        for (n, add) in grid.get(&pos).unwrap().iter().filter(|(pos, _)| !seen.contains(pos)) {
            let mut new_seen = seen.clone();
            new_seen.insert(*n);
            q.push_back((*n, cost + add, new_seen))
        }
    }

    max
}

fn parse_tile(ch: char) -> Tile {
    match ch {
        '.' => Tile::Open,
        '#' => Tile::Rock,
        '^' => Tile::Slope(Direction::Up),
        'v' => Tile::Slope(Direction::Down),
        '<' => Tile::Slope(Direction::Left),
        '>' => Tile::Slope(Direction::Right),
        x => panic!("bad tile: {}", x),
    }
}
