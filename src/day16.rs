// #![allow(unused_variables)]
// #![allow(unused_imports)]
// #![allow(unused_mut)]
// #![allow(dead_code)]
// #![allow(unused_assignments)]
// #![allow(unreachable_code)]

/*
    Advent of Code 2023: Day 16
        part1 answer:   7632
        part2 answer:   8023
 */

use std::collections::{HashSet, VecDeque};
use std::fmt::{Debug, Display, Formatter};
use std::time::Instant;
use advent_2023::Compass;

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

    println!("Advent of Code, Day 16");
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
    dir: Compass,
}

impl Beam {
    fn forward(mut self, max_rows: usize, max_cols: usize) -> Option<Self> {
        match self.dir {
            Compass::North if self.pos.y > 0 => { self.pos.y -= 1 }
            Compass::South if self.pos.y < max_rows - 1 => { self.pos.y += 1 }
            Compass::West if self.pos.x > 0 => { self.pos.x -= 1 }
            Compass::East if self.pos.x < max_cols - 1 => { self.pos.x += 1 }
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


fn part1(input_file: &str) -> String {
    let lines = advent_2023::file_to_lines(input_file);
    let grid = advent_2023::parse_grid(&lines);

    let o_grid = advent_2023::convert_grid_using(&grid, |ch| MirrorType::from_char(ch));

    let start = Beam {
        pos: Coord { x: 0, y: 0 },
        dir: Compass::East,
    };

    let answer = get_number_energized_from_start(start, &o_grid);

    return answer.to_string();
}

fn part2(input_file: &str) -> String {
    let lines = advent_2023::file_to_lines(input_file);

    let grid = advent_2023::parse_grid(&lines);

    let o_grid = advent_2023::convert_grid_using(&grid, |ch| MirrorType::from_char(ch));

    let max_rows = o_grid.len();
    let max_cols = o_grid[0].len();

    let starts =
        (0..max_cols).map(|cx| Beam { pos: Coord { x: cx, y: 0 }, dir: Compass::South })
            .chain((0..max_cols).map(|cx| Beam { pos: Coord { x: cx, y: max_rows - 1 }, dir: Compass::North }))
            .chain((0..max_rows).map(|cy| Beam { pos: Coord { x: 0, y: cy }, dir: Compass::East }))
            .chain((0..max_rows).map(|cy| Beam { pos: Coord { x: max_cols - 1, y: cy }, dir: Compass::West }));

    let energized_counts = starts.map(|b| get_number_energized_from_start(b, &o_grid)).max();


    let answer = energized_counts.unwrap();
    return answer.to_string();
}


fn get_number_energized_from_start(start: Beam, o_grid: &Vec<Vec<MirrorType>>) -> usize {
    let num_rows: usize = o_grid.len();
    let num_cols = o_grid[0].len();

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

        // Pair of directions, if only one direction it is repeated. Ugly but works.
        let (a,b) = match (beam.dir, tile) {
            (d, MirrorType::Nothing) => {
                (d,d)
            }
            (_, MirrorType::Hort) => {
                (Compass::West, Compass::East)
            }
            (_, MirrorType::Vert) => {
                (Compass::North, Compass::South)
            }
            (Compass::North, MirrorType::UpRight) | (Compass::South, MirrorType::DownLeft) => {
                (Compass::East, Compass::East)
            }
            (Compass::South, MirrorType::UpRight) | (Compass::North, MirrorType::DownLeft) => {
                (Compass::West, Compass::West)
            }
            (Compass::West, MirrorType::UpRight) | (Compass::East, MirrorType::DownLeft) => {
                (Compass::South,Compass::South)
            }
            (Compass::East, MirrorType::UpRight) | (Compass::West, MirrorType::DownLeft) => {
                (Compass::North, Compass::North)
            }
        };

            beam.dir = a;
            if let Some(beam) = beam.forward(num_rows, num_cols) {
                queue.push_back(beam.clone());
            }
            if a != b {
                beam.dir = b;
                if let Some(beam) = beam.forward(num_rows, num_cols) {
                    queue.push_back(beam.clone());
                }


        }
    }
    return energized.len();
}
