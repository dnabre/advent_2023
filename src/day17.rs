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

    println!("Advent of Code, Day 17");
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


fn part1(input_file: &str) -> String {
    let lines = advent_2023::file_to_lines(input_file);
    let grid = advent_2023::parse_grid(&lines);
    let grid = advent_2023::convert_grid_using(&grid, |ch| (ch as u8 - '0' as u8) as u32);


    let search_result = do_search(&grid, State::get_frontier);

    if let Some(answer) = search_result {
        return answer.to_string();
    } else {
        return String::from("ERROR");
    }
}

fn part2(input_file: &str) -> String {
    let lines = advent_2023::file_to_lines(input_file);
    let grid = advent_2023::parse_grid(&lines);
    let grid = advent_2023::convert_grid_using(&grid, |ch| (ch as u8 - '0' as u8) as u32);


    let search_result = do_search(&grid, State::get_ultra_frontier);

    if let Some(answer) = search_result {
        return answer.to_string();
    } else {
        return String::from("ERROR");
    }
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
            Direction::Down if self.row + 1 < rows => Coord {
                row: self.row + 1,
                col: self.col,
            },
            Direction::Left if self.col > 0 => Coord {
                row: self.row,
                col: self.col - 1,
            },
            Direction::Right if self.col + 1 < cols => Coord {
                row: self.row,
                col: self.col + 1,
            },
            _ => return None,
        };
        Some(coord)
    }
}


#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
struct State {
    cost: u32,
    pos: Coord,
    dir: Direction,
    inertia: u32,
}


impl Ord for State {
    fn cmp(&self, other: &Self) -> Ordering {
        other.cost.cmp(&self.cost)
    }
}


impl PartialOrd for State {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl State {
    fn get_frontier(&self, grid: &Vec<Vec<u32>>) -> Vec<Self> {
        let max_rows = grid.len();
        let max_cols = grid[0].len();

        let mut frontier_nodes = Vec::new();

        for dir in advent_2023::DIRECTION_ARRAY {
            if self.dir == dir && self.inertia == 3 {
                //max inertia
                continue;
            }

            if self.dir.opposite() == dir {
                //can't go backwards
                continue;
            }

            if let Some(pos) = self.pos.forward(&dir, max_rows, max_cols) {
                let cost = self.cost + grid[pos.row][pos.col];
                let new_inertia = if self.dir == dir {
                    self.inertia + 1
                } else {
                    1
                };
                frontier_nodes.push(State {
                    pos,
                    cost,
                    dir,
                    inertia: new_inertia,
                });
            }
        }
        return frontier_nodes;
    }

    fn get_ultra_frontier(&self, grid: &Vec<Vec<u32>>) -> Vec<Self> {
        let max_rows = grid.len();
        let max_cols = grid[0].len();

        let mut frontier_nodes: Vec<State> = Vec::new();

        for dir in advent_2023::DIRECTION_ARRAY {
            if self.inertia < 4 && dir != self.dir {
                continue;
            }

            if self.dir == dir && self.inertia == 10 {
                //max inertia
                continue;
            }


            if self.dir.opposite() == dir {
                //can't go backwards
                continue;
            }

            if let Some(pos) = self.pos.forward(&dir, max_rows, max_cols) {
                let cost = self.cost + grid[pos.row][pos.col];
                let new_inertia = if self.dir == dir {
                    self.inertia + 1
                } else {
                    1
                };
                frontier_nodes.push(State {
                    pos,
                    cost,
                    dir,
                    inertia: new_inertia,
                });
            }
        }
        return frontier_nodes;
    }
}


fn do_search(grid: &Vec<Vec<u32>>, expand: fn(&State, &Vec<Vec<u32>>) -> Vec<State>) -> Option<String> {
    let mut queue = BinaryHeap::new();
    let mut visited = HashSet::new();


    let (goal, right, down) = get_starter_states(&grid);

    // two possible moves from start, right & down. Cost of cell we start in only matters if we back into it.
    queue.push(right);
    queue.push(down);

    while let Some(state) = queue.pop() {
        if state.pos == goal {
            return Some(state.cost.to_string());
        }
        for f_state in expand(&state, &grid) {
            if !visited.contains(&(f_state.pos, f_state.dir, f_state.inertia)) {
                visited.insert((f_state.pos, f_state.dir, f_state.inertia));
                queue.push(f_state)
            }
        }
    }
    None
}

fn get_starter_states(grid: &Vec<Vec<u32>>) -> (Coord, State, State) {
    let goal = Coord {
        row: grid.len() - 1,
        col: grid[0].len() - 1,
    };
    let right = State {
        cost: grid[0][1],
        dir: Direction::Right,
        pos: Coord { row: 0, col: 1 },
        inertia: 1,
    };

    let down = State {
        cost: grid[1][0],
        dir: Direction::Down,
        pos: Coord { row: 1, col: 0 },
        inertia: 1,
    };
    (goal, right, down)
}