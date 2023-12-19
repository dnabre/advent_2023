#![allow(unused_variables)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(dead_code)]
#![allow(unused_assignments)]
#![allow(unreachable_code)]

/*
    Advent of Code 2023: Day 16
        part1 answer:   7632
        part2 answer:   8023
 */

use std::collections::{HashSet, VecDeque};
use std::fmt::{Debug, Display, Formatter};
use std::time::Instant;
use advent_2023::get_neighbor_points;

const ANSWER: (&str, &str) = ("7632", "8023");

fn main() {
    let _filename_test = "data/day16/test_input_01.txt";
    let _filename_test2 = "data/day16/test_input_02.txt";

    let filename_part1 = "data/day16/part1_input.txt";
    let filename_part2 = "data/day16/part2_input.txt";

    let start1 = Instant::now();
    let answer1 = part1(filename_part1);
    let duration1 = start1.elapsed();

    let start2 = Instant::now();
    let answer2 = part2(filename_part2);
    let duration2 = start2.elapsed();

    //println!("Advent of Code, Day 16");
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


#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Copy, Hash)]
enum MirrorType {
    UpRight,
    DownLeft,
    Hort,
    Vert,
    Nothing,
}

impl MirrorType {
    fn from_char(ch: char) -> MirrorType {
        match ch {
            '/' => { MirrorType::UpRight }
            '\\' => { MirrorType::DownLeft }
            '-' => { MirrorType::Hort }
            '|' => { MirrorType::Vert }
            _ => MirrorType::Nothing
        }
    }
}

impl Display for MirrorType {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", match self {
            MirrorType::UpRight => { '/' }
            MirrorType::DownLeft => { '\\' }
            MirrorType::Hort => { '-' }
            MirrorType::Vert => { '|' }
            MirrorType::Nothing => { '.' }
        })
    }
}


#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Copy, Hash)]
struct Beam {
    pos: Coord,
    dir: Direction,
}

impl Beam {
    fn forward(mut self, max_rows: usize, max_cols: usize) -> Option<Self> {
        match self.dir {
            Direction::Up if self.pos.y > 0 => { self.pos.y -= 1 }
            Direction::Down if self.pos.y < max_rows - 1 => { self.pos.y += 1 }
            Direction::Left if self.pos.x > 0 => { self.pos.x -= 1 }
            Direction::Right if self.pos.x < max_cols - 1 => { self.pos.x += 1 }
            _ => return None
        }

        return Some(self);
    }
}


#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Copy, Hash)]
struct Coord {
    x: usize,
    y: usize,
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Copy, Hash)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

fn part1(input_file: &str) -> String {
    let lines = advent_2023::file_to_lines(input_file);

    let grid = advent_2023::parse_grid(&lines);

    let mut o_grid = convert_grid_using(&grid, |ch| MirrorType::from_char(ch) );

    let start = Beam {
        pos: Coord { x: 0, y: 0 },
        dir: Direction::Right,
    };
    let answer = get_number_energized_from_start(start, & o_grid );

    return answer.to_string();
}

fn convert_grid_using(grid: &Vec<Vec<char>>, convert: fn(char) -> MirrorType) -> Vec<Vec<MirrorType>> {
    let mut o_grid = Vec::with_capacity(grid.len());
    for row in grid {
        let mut grid_row:Vec<MirrorType> = Vec::with_capacity(row.len());
        for r in row {
            grid_row.push(convert(*r));
        }
        o_grid.push(grid_row);

    }
    return o_grid;
}

fn part2(input_file: &str) -> String {
    let lines = advent_2023::file_to_lines(input_file);

    let grid = advent_2023::parse_grid(&lines);

    let mut o_grid = convert_grid_using(&grid, |ch| MirrorType::from_char(ch) );

    let start = Beam {
        pos: Coord { x: 0, y: 0 },
        dir: Direction::Right,
    };

    let max_rows = o_grid.len();
    let max_cols = o_grid[0].len();
    println!("grid size: {max_rows} rows by {max_cols} columns");

    let top_row = (0..max_cols).map(|cx| Beam{ pos: Coord{x: cx, y: 0 }, dir:Direction::Down});

    let bot_row = (0..max_cols).map(|cx| Beam{ pos: Coord{x: cx, y: max_rows-1,}, dir:Direction::Up});

    let left_row = (0..max_rows).map(|cy| Beam{ pos: Coord{ x: 0, y: cy }, dir:Direction::Right});

    let right_row = (0..max_rows).map(|cy| Beam{ pos: Coord{ x: max_cols -1, y: cy }, dir:Direction::Left   });

    let starts:Vec<Beam> = top_row.chain(bot_row).chain(left_row).chain(right_row).collect();

    let energized_counts = starts.iter().map(|b| get_number_energized_from_start(*b, &o_grid)).max();



    let answer = energized_counts.unwrap();
    return answer.to_string();

}



fn get_number_energized_from_start(start: Beam, o_grid: & Vec<Vec<MirrorType>> ) -> usize {
    let num_rows: usize = o_grid.len();
    let num_cols= o_grid[0].len();


    let mut queue: VecDeque<Beam> = VecDeque::new();
    let mut energized: HashSet<Coord> = HashSet::new();
    let mut seen: HashSet<Beam> = HashSet::new();

    queue.push_back(start);

    while let Some(mut beam) = queue.pop_front() {
        if seen.contains(&beam) {
            continue;
        }
        energized.insert(beam.pos);
        seen.insert(beam);

        let pos = beam.pos;
        let tile = o_grid[pos.y][pos.x];
        let dirs = match (beam.dir, tile) {
            (d, MirrorType::Nothing) => {
                vec![d]
            }
            (_, MirrorType::Hort) => {
                vec![Direction::Left, Direction::Right]
            }
            (_, MirrorType::Vert) => {
                vec![Direction::Up, Direction::Down]
            }
            (Direction::Up, MirrorType::UpRight) | (Direction::Down, MirrorType::DownLeft) => {
                vec![Direction::Right]
            }
            (Direction::Down, MirrorType::UpRight) | (Direction::Up, MirrorType::DownLeft) => {
                vec![Direction::Left]
            }
            (Direction::Left, MirrorType::UpRight) | (Direction::Right, MirrorType::DownLeft) => {
                vec![Direction::Down]
            }
            (Direction::Right, MirrorType::UpRight) | (Direction::Left, MirrorType::DownLeft) => {
                vec![Direction::Up]
            }
        };
        for d in dirs {
            beam.dir = d;
            if let Some(beam) = beam.forward(num_rows, num_cols) {
                queue.push_back(beam.clone());
            }
        }
    }
    return energized.len()
}
