// #![allow(unused_variables)]
// #![allow(unused_imports)]
// #![allow(unused_mut)]
// #![allow(dead_code)]
// #![allow(unused_assignments)]
// #![allow(unreachable_code)]

/*
    Advent of Code 2023: Day 17
        part1 answer:   814
        part2 answer:   974


 */


use std::cmp::Ordering;
use std::collections::{BinaryHeap, HashSet};
use std::fmt::Debug;
use std::hash::Hash;
use std::time::Instant;

use advent_2023::Direction;

const ANSWER: (&str, &str) = ("814", "974");

fn main() {
    let _filename_test = "data/day17/test_input_01.txt";
    let _filename_test2 = "data/day17/test_input_02.txt";
    let _filename_test3 = "data/day17/test_input_03.txt";

    let filename_part1 = "data/day17/part1_input.txt";
    let filename_part2 = "data/day17/part2_input.txt";

    let start1 = Instant::now();
    let answer1 = part1(filename_part1);
    let duration1 = start1.elapsed();

    let start2 = Instant::now();
    let answer2 = part2(filename_part2);
    let duration2 = start2.elapsed();

    //println!("Advent of Code, Day 17");
    println!("    ---------------------------------------------");

    println!("\t Part 1: {:14} time: {:?}", answer1, duration1);
    if ANSWER.0 != answer1 {
        println!("\t\t ERROR: Answer is WRONG. Got: {}, Expected {}", answer1, ANSWER.0);
    }

    println!("\t Part 2: {:14} time: {:?}", answer2, duration2);
    if ANSWER.1 != answer2 {
        println!("\t\t ERROR: Answer is WRONG. Got: {}, Expected {}", answer2, ANSWER.1);
    }
    println!("    ---------------------------------------------");
}


#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Clone, Copy)]
struct Coord {
    row: usize,
    col: usize,
}

impl Coord {
    fn forward(&self, dir: &Direction, rows: usize, cols: usize) -> Option<Self> {
        let coord = match dir {
            Direction::Up if self.row > 0 => Coord {
                row: self.row - 1,
                col: self.col,
            },
            Direction::Down if self.row + 1 < rows  => Coord {
                row: self.row + 1,
                col: self.col,
            },
            Direction::Left if self.col > 0 => Coord {
                row: self.row,
                col: self.col - 1,
            },
            Direction::Right if self.col +1  < cols => Coord {
                row: self.row,
                col: self.col + 1,
            },
            _ => return None,
        };
        Some(coord)
    }
}


#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
struct Crucible {
    cost: u32,
    pos: Coord,
    dir: Direction,
    moves_in_dir: u32,
}


impl Ord for Crucible {
    fn cmp(&self, other: &Self) -> Ordering {
        other.cost.cmp(&self.cost)
    }
}


impl PartialOrd for Crucible {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Crucible {
    fn successors(&self, grid: &Vec<Vec<u32>>) -> Vec<Self> {
        let max_rows = grid.len();
        let max_cols = grid[0].len();

        let mut successors = Vec::new();

        for dir in advent_2023::DIRECTION_ARRAY {
            if self.dir == dir && self.moves_in_dir == 3 {
                //max inertia
                continue;
            }

            if self.dir.opposite() == dir {
                //can't go backwards
                continue;
            }

            if let Some(pos) = self.pos.forward(&dir, max_rows, max_cols) {
                let cost = self.cost + grid[pos.row][pos.col];
                let moves_in_dir = if self.dir == dir {
                    self.moves_in_dir + 1
                } else {
                    1
                };
                successors.push(Crucible {
                    pos,
                    cost,
                    dir,
                    moves_in_dir,
                });
            }
        }
        return successors;
    }
}


#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
struct UltraCrucible {
    cost: u32,
    pos: Coord,
    dir: Direction,
    moves_in_dir: u32,
}


impl Ord for UltraCrucible {
    fn cmp(&self, other: &Self) -> Ordering {
        other.cost.cmp(&self.cost)
    }
}


impl PartialOrd for UltraCrucible {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl UltraCrucible {
    fn successors(&self, grid: &Vec<Vec<u32>>) -> Vec<Self> {
        let max_rows = grid.len();
        let max_cols = grid[0].len();

        let mut successors = Vec::new();

        for dir in advent_2023::DIRECTION_ARRAY {
            if self.moves_in_dir < 4 && dir != self.dir {
                continue;
            }

            if self.dir == dir && self.moves_in_dir == 10 {
                //max inertia
                continue;
            }


            if self.dir.opposite() == dir {
                //can't go backwards
                continue;
            }

            if let Some(pos) = self.pos.forward(&dir, max_rows, max_cols) {
                let cost = self.cost + grid[pos.row][pos.col];
                let moves_in_dir = if self.dir == dir {
                    self.moves_in_dir + 1
                } else {
                    1
                };
                successors.push(UltraCrucible {
                    pos,
                    cost,
                    dir,
                    moves_in_dir,
                });
            }
        }
        return successors;
    }
}


fn part1(input_file: &str) -> String {
    let lines = advent_2023::file_to_lines(input_file);
    let grid = advent_2023::parse_grid(&lines);
    let grid = advent_2023::convert_grid_using(&grid, |ch| (ch as u8 - '0' as u8) as u32);


    let goal = Coord {
        row: grid.len() - 1,
        col: grid[0].len() - 1,
    };
    println!("Goal Coord: {:?}", goal);

    let mut pq = BinaryHeap::new();
    let mut seen = HashSet::new();


    let right = Crucible {
        cost: grid[0][1],
        dir: Direction::Right,
        pos: Coord { row: 0, col: 1 },
        moves_in_dir: 1,
    };

    let down = Crucible {
        cost: grid[1][0],
        dir: Direction::Down,
        pos: Coord { row: 1, col: 0 },
        moves_in_dir: 1,
    };

    // two possible moves from start, right & down. Cost of cell we start in only matters if we back into it.
    pq.push(right);
    pq.push(down);

    while let Some(crucible) = pq.pop() {
        if crucible.pos == goal {
            println!("Found goal @ {:?} with total heat cost: {}", crucible.pos, crucible.cost);
            return crucible.cost.to_string();
        }

        for successor in crucible.successors(&grid) {
            if seen.insert((successor.pos, successor.dir, successor.moves_in_dir)) {
                // if it was already in the set, the insert would return false
                pq.push(successor);
            }
        }
    }


    let answer = 0;
    return answer.to_string();
}

fn part2(input_file: &str) -> String {
    let lines = advent_2023::file_to_lines(input_file);
    let grid = advent_2023::parse_grid(&lines);
    let grid = advent_2023::convert_grid_using(&grid, |ch| (ch as u8 - '0' as u8) as u32);


    let goal = Coord {
        row: grid.len() - 1,
        col: grid[0].len() - 1,
    };
    println!("Goal Coord: {:?}", goal);

    let mut pq: BinaryHeap<UltraCrucible> = BinaryHeap::new();
    let mut seen = HashSet::new();


    let right = UltraCrucible {
        cost: grid[0][1],
        dir: Direction::Right,
        pos: Coord { row: 0, col: 1 },
        moves_in_dir: 1,
    };

    let down = UltraCrucible {
        cost: grid[1][0],
        dir: Direction::Down,
        pos: Coord { row: 1, col: 0 },
        moves_in_dir: 1,
    };

    // two possible moves from start, right & down. Cost of cell we start in only matters if we back into it.
    pq.push(right);
    pq.push(down);

    while let Some(crucible) = pq.pop() {
        if crucible.pos == goal {
            println!("Found goal @ {:?} with total heat cost: {}", crucible.pos, crucible.cost);
            return crucible.cost.to_string();
        }

        for successor in crucible.successors(&grid) {
            if seen.insert((successor.pos, successor.dir, successor.moves_in_dir)) {
                // if it was already in the set, the insert would return false
                pq.push(successor);
            }
        }
    }


    let answer = 0;
    return answer.to_string();
}