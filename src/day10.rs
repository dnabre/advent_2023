#![allow(unused_variables)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(dead_code)]
#![allow(unused_assignments)]
#![allow(unreachable_code)]

/*
    Advent of Code 2023: Day 10
        part1 answer:
        part2 answer:

 */

use std::ops::Index;
use std::collections::{HashMap, HashSet};
use std::rc::Rc;
use std::time::Instant;
use advent_2023::file_to_lines;


const ANSWER: (&str, &str) = ("7173", "291");


// Hardcode which pipe should be used for each problem
const TEST_START: ((i32, i32), char) = ((1, 1), 'F');
const PART1_START: ((i32, i32), char) = ((25, 42), '7');



fn main() {
    let _filename_test = "data/day10/test_input_01.txt";
    let _filename_test2 = "data/day10/test_input_02.txt";
    let _filename_test3 = "data/day10/test_input_03.txt";
    let _filename_test4 = "data/day10/test_input_04.txt";

    let filename_part1 = "data/day10/part1_input.txt";
    let filename_part2 = "data/day10/part2_input.txt";

    let start1 = Instant::now();
    let answer1 = part1(filename_part1);
    let duration1 = start1.elapsed();

    let start2 = Instant::now();
    let answer2 = part2(_filename_test2);
    let duration2 = start2.elapsed();

    //  println!("Advent of Code, Day 10");
    println!("    ---------------------------------------------");

    println!("\t Part 1: {:14} time: {:?}", answer1, duration1);
    if ANSWER.0 != answer1 {
        println!("\t\t ERROR: Answer is WRONG. Got: {}, Expected {}", answer1, ANSWER.0);
    }

//     println!("\t Part 2: {:14} time: {:?}", answer2, duration2);
//     if ANSWER.1 != answer2 {
//         println!("\t\t ERROR: Answer is WRONG. Got: {}, Expected {}", answer2, ANSWER.1);
//     }
    println!("    ---------------------------------------------");
}

type Coord = (i32, i32);
type Pipe = ((i32, i32), (i32, i32));

const PIPE_TYPES: [char; 6] = ['|', '-', 'L', 'J', '7', 'F'];

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
enum Facing {
    Up,
    Down,
    Left,
    Right,
}

const FACINGS: [Facing; 4] = [Facing::Up, Facing::Down, Facing::Left, Facing::Right];

impl Facing {
    fn oppose(&self) -> Facing {
        match self {
            Facing::Up => { Facing::Down }
            Facing::Down => { Facing::Up }
            Facing::Left => { Facing::Right }
            Facing::Right => { Facing::Left }
        }
    }

    fn pipe_from_facing(a: Facing, b: Facing) -> char {
        for p in PIPE_TYPES {
            let (p1, p2) = pipe_to_facing(p);
            if (p1 == a || p2 == a) && (p1 == b || p2 == b) {
                return p;
            }
        }
        panic!("no pipe for facing pair {:?}", (a, b));
    }
}


fn pipe_to_offset(pipe: char) -> Pipe {
    match pipe {
        '|' => { ((-1, 0), (1, 0)) } // North/South
        '-' => { ((0, -1), (0, 1)) } // West/East
        'L' => { ((-1, 0), (0, 1)) } // North/East
        'J' => { ((-1, 0), (0, -1)) } // North/West
        '7' => { ((1, 0), (0, -1)) } // South/West
        'F' => { ((1, 0), (0, 1)) } //South/East
        _ => { panic!("unknown pipe") }
    }
}

fn pipe_to_facing(pipe: char) -> (Facing, Facing) {
    match pipe {
        '|' => { (Facing::Up, Facing::Down) } // North/South
        '-' => { (Facing::Left, Facing::Right) } // West/East
        'L' => { (Facing::Up, Facing::Right) } // North/East
        'J' => { (Facing::Up, Facing::Left) } // North/West
        '7' => { (Facing::Down, Facing::Left) } // South/West
        'F' => { (Facing::Down, Facing::Right) }//South/East
        _ => { panic!("unknown pipe") }
    }
}

fn pipe_has(pipe: char, facing: Facing) -> bool {
    let (a, b) = pipe_to_facing(pipe);
    return facing == a || facing == b;
}

fn factial_to_my(loc: Coord, facing: Facing) -> Coord {
    let (x, y) = loc;

    match facing {
        Facing::Up => { (x, y - 1) }
        Facing::Down => { (x, y + 1) }
        Facing::Left => { (x - 1, y) }
        Facing::Right => { (x + 1, y) }
    }
}


fn connects_to(p_type: char, loc: Coord) -> [Coord; 2] {
    let (o1, o2) = pipe_to_offset(p_type);
    let c1 = (loc.0 + o1.0, loc.1 + o1.1);
    let c2 = (loc.0 + o2.0, loc.1 + o2.1);
    return [c1, c2];
}


#[derive(Debug, Clone, PartialEq, Eq, Hash)]
struct Node<'a> {
    id: Coord,
    pipe_type: (Facing, Facing),
    links: Vec<&'a Node<'a>>,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
enum GridType {
    Start,
    Dirt,
    Pipe,
}


fn part1(input_file: &str) -> String {
    let lines = file_to_lines(input_file);
    let grid: Vec<Vec<char>> = lines.iter().map(|l| l.chars().collect()).collect();

    let mut node_list: Vec<Node> = Vec::new();
    let mut node_map: HashMap<Coord, &Node> = HashMap::new();


    let mut start_point: Coord = (-1, -1);
    for y in 0..grid.len() {
        for x in 0..grid[0].len() {
            let ch = grid[y][x];
            let (x, y) = (x as i32, y as i32);
            if ch == 'S' {
                start_point = (x, y);
            }
        }
    }
    let start_point = start_point;
    println!("start point: {:?}", start_point);

    for y in (start_point.1-5)..(start_point.1+5) {
        for x in (start_point.0-5)..(start_point.0+5) {
            let ch = grid[y as usize][x as usize];
            print!("{ch}");

        }
        println!();
    }



    let mut answer: usize = 0;
    return answer.to_string();
}

fn part2(input_file: &str) -> String {
    let lines = file_to_lines(input_file);

    let mut answer: usize = 0;


    return answer.to_string();
}