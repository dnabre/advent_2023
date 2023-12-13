#![allow(unused_variables)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(dead_code)]
#![allow(unused_assignments)]
#![allow(unreachable_code)]

/*
    Advent of Code 2023: Day 10
        part1 answer: 7173
        part2 answer:

 */


use std::collections::{HashMap, HashSet, VecDeque};
use std::fmt::{Display, Formatter};
use std::time::Instant;
use advent_2023::file_to_lines;


const ANSWER: (&str, &str) = ("7173", "291");


// Hardcode which pipe should be used for each problem const TEST_START: (Coord, char) = (Coord { x: 1, y: 1 }, 'F'); const PART1_START: (Coord, char) = (Coord { x: 25, y: 42 }, '7');


fn main() {
    let _filename_test1 = "data/day10/test_input_01.txt";
    let _filename_test2 = "data/day10/test_input_02.txt";
    let _filename_test3 = "data/day10/test_input_03.txt";
    let _filename_test4 = "data/day10/test_input_04.txt";
    let _filename_test5 = "data/day10/test_input_05.txt";

    let filename_part1 = "data/day10/part1_input.txt";
    let filename_part2 = "data/day10/part2_input.txt";

    let start1 = Instant::now();
    let answer1 = part1(filename_part1);
    let duration1 = start1.elapsed();

    let start2 = Instant::now();
    let answer2 = part2(_filename_test1);
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

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
struct Coord {
    x: i32,
    y: i32,
}

impl Coord {
    fn offset_from(&self, other: Coord) -> (i32, i32) {
        let dx = self.x - other.x;
        let dy = self.y - other.y;
        return (dx, dy);
    }
}

impl Display for Coord {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "[{},{}]", self.x, self.y)
    }
}

impl Coord {
    fn shape_from_grid(&self, grid: &Vec<Vec<char>>) -> char {
        if self.x < 0 || self.y < 0 {
            return '.';
        }

        let ch = grid[self.y as usize][self.x as usize];
        return ch;
    }

    fn add_offsetr(&self, d_x: i32, d_y: i32) -> Coord {
        let new_x = self.x + d_x;
        let new_y = self.y + d_y;
        let new_coord = Coord { x: new_x, y: new_y };
        return new_coord;
    }
    fn add_offset(&self, other: Coord) -> Coord {
        self.add_offsetr(other.x, other.y)
    }

    fn add_offsetpi(&self, (d_x, d_y): (i32, i32)) -> Coord {
        self.add_offsetr(d_x, d_y)
    }
}


#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
struct Pipe {
    left: Coord,
    right: Coord,
}

const PIPE_TYPES: [char; 6] = ['|', '-', 'L', 'J', '7', 'F'];

const FACINGS: [Facing; 4] = [Facing::Up, Facing::Down, Facing::Left, Facing::Right];


#[derive(Debug, Copy, Clone, PartialEq, PartialOrd, Eq, Hash)]
enum Facing {
    Up,
    Down,
    Left,
    Right,
}

impl Display for Facing {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}",
               match self {
                   Facing::Up => { "Up" }
                   Facing::Down => { "Down" }
                   Facing::Left => { "Left" }
                   Facing::Right => { "Right" }
               })
    }
}


impl Facing {
    fn oppose(&self) -> Facing {
        match self {
            Facing::Up => { Facing::Down }
            Facing::Down => { Facing::Up }
            Facing::Left => { Facing::Right }
            Facing::Right => { Facing::Left }
        }
    }
}


fn pipe_to_offset(pipe: char) -> Pipe {
    let (l, r) = match pipe {
        '|' => { ((0, -1), (0, 1)) } // North/South
        '-' => { ((-1, 0), (1, 0)) } // West/East
        'L' => { ((0, -1), (1, 0)) } // North/East
        'J' => { ((-1, 0), (0, -1)) } // North/West
        '7' => { ((-1, 0), (0, 1)) } // South/West
        'F' => { ((1, 0), (0, 1)) } //South/East
        _ => { panic!("unknown pipe") }
    };
    let (pl, pr) = (Coord { x: l.0, y: l.1 }, Coord { x: r.0, y: r.1 });
    return Pipe { left: pl, right: pr };
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


fn connects_to(p_type: char, loc: Coord) -> [Coord; 2] {
    //println!("{loc} of {p_type} has offsets: ");

    let o_pipe = pipe_to_offset(p_type);
    let (o1, o2) = (o_pipe.left, o_pipe.right);
    let t1 = loc.add_offset(o1);
    let t2 = loc.add_offset(o2);
    return [t1, t2];
}


#[derive(Debug, Clone, PartialEq, Eq, Hash, Copy)]
struct State {
    loc: Coord,
    last_loc: Coord,
    length: usize,
    pipe: char,
}

impl Display for State {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "State: '{}' loc: {}, last_loc {}, length: {}",
               self.pipe, self.loc, self.last_loc, self.length)
    }
}


fn get_start_point(grid: &Vec<Vec<char>>) -> Coord {
    let y_max = grid.len();
    let x_max = grid[0].len();

    let mut start_point: Coord = Coord { x: -1, y: -1 };
    for y in 0..y_max {
        for x in 0..x_max {
            let ch = grid[y][x];
            if ch == 'S' {
                start_point = Coord { x: x as i32, y: y as i32 };
                return start_point;
            }
        }
    }
    start_point
}

fn get_start_shape(grid: &Vec<Vec<char>>, start_loc: &Coord) -> char {
    let offsets = [(1, 0), (-1, 0), (0, -1), (0, 1)];

    let ch = start_loc.add_offsetpi(offsets[0]).shape_from_grid(&grid);
    let connects_east = ch == '-' || ch == 'J' || ch == '7';
    let ch = start_loc.add_offsetpi(offsets[1]).shape_from_grid(&grid);
    let connects_west = ch == '_' || ch == 'F' || ch == 'L';
    let ch = start_loc.add_offsetpi(offsets[2]).shape_from_grid(&grid);
    let connects_north = ch == '|' || ch == '7' || ch == 'F';
    let ch = start_loc.add_offsetpi(offsets[3]).shape_from_grid(&grid);
    let connects_south = ch == '|' || ch == 'L' || ch == 'J';

    let start_pipe = match (connects_east, connects_west, connects_north, connects_south) {
        (_, _, true, true) => { '|' }
        (true, true, _, _) => { '-' }
        (true, _, true, _) => { 'L' }
        (_, true, true, _) => { 'J' }
        (_, true, _, true) => { '7' }
        (true, _, _, true) => { 'F' }
        (_, _, _, _) => {
            panic!("unknown facing for bool pattern: {:?}", (connects_east, connects_west, connects_north, connects_south));
        }
    };
    start_pipe
}


fn check_from_to((loc, c_pipe): (Coord, char), (n_loc, n_pipe): (Coord, char)) -> bool {
    if c_pipe == '.' || n_pipe == '.' {
        // neither should be grass
        return false;
    }
    let other_facing = pipe_to_facing(n_pipe);

    let offset = n_loc.offset_from(loc);

    assert_ne!(offset, (0, 0));
    let f = facing_from_offset(offset);
    let f = f.oppose();
    if other_facing.0 == f || other_facing.1 == f {
        return true;
    } else {
        return false;
    }
}

fn facing_from_offset(offset: (i32, i32)) -> Facing {
    match offset {
        (0, 1) => { Facing::Down }
        (0, -1) => { Facing::Up }
        (1, 0) => { Facing::Right }
        (-1, 0) => { Facing::Left }
        _ => { panic!("unable to determine facing for offset {:?}", offset); }
    }
}

fn part1(input_file: &str) -> String {
    let lines = file_to_lines(input_file);

    let grid: Vec<Vec<char>> = lines.iter().map(|l| l.chars().collect()).collect();
    let start_point = get_start_point(&grid);

    let start_pipe_shape = get_start_shape(&grid, &start_point);
    let next_to_start = connects_to(start_pipe_shape, start_point);

    let mut visited_pos: HashSet<Coord> = HashSet::new();
    let mut queue: VecDeque<State> = VecDeque::new();
    let mut current;

    visited_pos.insert(start_point);

    current = State {
        loc: next_to_start[0],
        last_loc: start_point,
        length: 1,
        pipe: next_to_start[0].shape_from_grid(&grid),
    };
    queue.push_front(current);
    current = State {
        loc: next_to_start[1],
        last_loc: start_point,
        length: 1,
        pipe: next_to_start[1].shape_from_grid(&grid),
    };
    queue.push_front(current);


    while !queue.is_empty() {
        current = queue.pop_front().unwrap();
        visited_pos.insert(current.loc);
        let pipe_shape_lookup = current.loc.shape_from_grid(&grid);
        let pipe_shape = current.pipe;
        let adj = connects_to(pipe_shape, current.loc);
        for i in 0..2 {
            let n_loc = adj[i];
            if visited_pos.contains(&n_loc) {
                continue;
            } else {
                let ch = n_loc.shape_from_grid(&grid);
                if !check_from_to((current.loc, pipe_shape),
                                  (n_loc, ch)) {
                    continue;
                } else {
                    let new_state = State {
                        loc: n_loc,
                        last_loc: current.loc,
                        length: current.length + 1,
                        pipe: ch,
                    };
                    queue.push_back(new_state);
                }
            }
        }
    }
    println!("{:?}", current);


    let answer: usize = current.length;
    return answer.to_string();
}

fn part2(input_file: &str) -> String {
    let lines = file_to_lines(input_file);

    let grid: Vec<Vec<char>> = lines.iter().map(|l| l.chars().collect()).collect();
    let start_point = get_start_point(&grid);

    let start_pipe_shape = get_start_shape(&grid, &start_point);
    let next_to_start = connects_to(start_pipe_shape, start_point);

    let mut visited_pos: HashSet<Coord> = HashSet::new();
    let mut queue: VecDeque<State> = VecDeque::new();
    let mut current;

    visited_pos.insert(start_point);

    current = State {
        loc: next_to_start[0],
        last_loc: start_point,
        length: 1,
        pipe: next_to_start[0].shape_from_grid(&grid),
    };
    queue.push_front(current);
    current = State {
        loc: next_to_start[1],
        last_loc: start_point,
        length: 1,
        pipe: next_to_start[1].shape_from_grid(&grid),
    };
    queue.push_front(current);


    while !queue.is_empty() {
        current = queue.pop_front().unwrap();
        visited_pos.insert(current.loc);
        let pipe_shape_lookup = current.loc.shape_from_grid(&grid);
        let pipe_shape = current.pipe;
        let adj = connects_to(pipe_shape, current.loc);
        for i in 0..2 {
            let n_loc = adj[i];
            if visited_pos.contains(&n_loc) {
                continue;
            } else {
                let ch = n_loc.shape_from_grid(&grid);
                if !check_from_to((current.loc, pipe_shape),
                                  (n_loc, ch)) {
                    continue;
                } else {
                    let new_state = State {
                        loc: n_loc,
                        last_loc: current.loc,
                        length: current.length + 1,
                        pipe: ch,
                    };
                    queue.push_back(new_state);
                }
            }
        }
    }
    println!("{:?}", current);

    println!("visited positions: \n\t{:?}", visited_pos);
    println!("number of visited positions: {}", visited_pos.len());

    let answer: usize = current.length;
    return answer.to_string();
}
