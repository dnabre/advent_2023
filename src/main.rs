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

use std::ops::Index;
use std::collections::{HashMap, HashSet, VecDeque};
use std::fmt::{Display, Formatter};
use std::rc::Rc;
use std::time::Instant;
use advent_2023::file_to_lines;


const ANSWER: (&str, &str) = ("7173", "291");


// Hardcode which pipe should be used for each problem
const TEST_START: (Coord, char) = (Coord { x: 1, y: 1 }, 'F');
const PART1_START: (Coord, char) = (Coord { x: 25, y: 42 }, '7');


fn main() {
    let _filename_test1 = "data/day10/test_input_01.txt";
    let _filename_test2 = "data/day10/test_input_02.txt";
    let _filename_test3 = "data/day10/test_input_03.txt";
    let _filename_test4 = "data/day10/test_input_04.txt";

    let filename_part1 = "data/day10/part1_input.txt";
    let filename_part2 = "data/day10/part2_input.txt";

    let start1 = Instant::now();
    let answer1 = part1(_filename_test1);
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

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
struct Coord {
    x: i32,
    y: i32,
}

impl Coord {
    fn offset_from(&self, other: Coord) -> (i32,i32) {
        let dx = self.x - other.x;
        let dy = self.y - other.y;
        return (dx,dy)
    }
}

impl Display for Coord {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "[{},{}]", self.x, self.y)
    }
}

impl Coord {
    fn shape_from_grid(&self, grid: &Vec<Vec<char>>) -> char {
        let ch = grid[self.y as usize][self.x as usize];
        return ch;
    }


    fn add_offset(&self, other: Coord) -> Coord {
        let new_x = self.x + other.x;
        let new_y = self.y + other.y;
        let new_coord =  Coord { x: new_x, y: new_y };
        return new_coord;
    }
}


#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
struct Pipe {
    left: Coord,
    right: Coord,
}

const PIPE_TYPES: [char; 6] = ['|', '-', 'L', 'J', '7', 'F'];

const FACINGS: [Facing; 4] = [Facing::Up, Facing::Down, Facing::Left, Facing::Right];


#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
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
                   Facing::Up => {"Up"}
                   Facing::Down => {"Down"}
                   Facing::Left => {"Left"}
                   Facing::Right => {"Right"}
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
    let (l, r) = match pipe {
        '|' => { ((0, -1), (0, 1)) } // North/South
        '-' => { ((-1, 0), (1, 0 )) } // West/East
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
    let d1 = t1.offset_from(loc);
    let d2 = t2.offset_from(loc);

    return [t1, t2];
}


#[derive(Debug, Clone, PartialEq, Eq, Hash, Copy)]
struct State {
    loc: Coord,
    last_loc: Coord,
    length: usize,
    pipe: char
}
impl Display for State {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f,"State: '{}' loc: {}, last_loc {}, length: {}",
               self.pipe, self.loc, self.last_loc, self.length)
    }
}

fn part1(input_file: &str) -> String {
    let lines = file_to_lines(input_file);
    let mut start_pipe_shape: char = ' ';
    let mut start_point: Coord;
    let (start_point, start_pipe_shape) = match input_file {
        "data/day10/part1_input.txt" => {
            PART1_START
        }
        _ => {
            TEST_START
        }
    };

    println!("start: {:?}", (start_point, start_pipe_shape));
    let grid: Vec<Vec<char>> = lines.iter().map(|l| l.chars().collect()).collect();

    let y_max = grid.len();
    let x_max = grid[0].len();

    let mut test_start_loc:Coord = Coord{x:-1,y:-1};

 'y_loop: for y in 0..y_max {
         for x in 0..x_max {
            let ch = grid[y][x];
            if ch == 'S' {
                test_start_loc = Coord{x: x as i32, y: y as i32};
                break 'y_loop;
            }
        }
    }

    println!("found start @{}, which is {}", test_start_loc,
             if test_start_loc == start_point { "correct"} else {"wrong"}
    );

    let start_adj_coords:Vec<Coord> =
        advent_2023::get_neighbor_points((test_start_loc.x, test_start_loc.y), false)
            .iter().map(|(cx,cy)| Coord{ x: *cy, y: *cx }).collect();
    println!("{:?}", start_adj_coords);

    // let ch = n_loc.shape_from_grid(&grid);
    // let good_connection = check_from_to((current.loc,pipe_shape), (n_loc,ch));

    let mut test_start_pipe:char = '.';
    'loop_over_pipe_shape: for p in PIPE_TYPES {
        for adj_c in &start_adj_coords {
            let other_shape = adj_c.shape_from_grid(&grid);
            if check_from_to((test_start_loc, p), (*adj_c, other_shape)) {
                test_start_pipe = p;
                println!("good pipe shape: {p}");
            }
        }
    }

        println!("found start pipe: {}@{} compared to {}@{}",
                 test_start_pipe, test_start_loc, start_pipe_shape,start_point);




    return String::new();


    let next_to_start = connects_to(start_pipe_shape, start_point);


    let mut visited_pos:HashSet<Coord> = HashSet::new();

    let mut queue: VecDeque<State> = VecDeque::new();

    let mut current;




    visited_pos.insert(start_point);
    current = State {
        loc: start_point,
        last_loc: start_point,
        length: 0,
        pipe: 'S',
    };





    current = State {
        loc: next_to_start[0],
        last_loc: start_point,
        length: 1,
        pipe:next_to_start[0].shape_from_grid(&grid),
    };
    queue.push_front(current);
    current = State {
        loc: next_to_start[1],
        last_loc: start_point,
        length: 1,
        pipe: next_to_start[1].shape_from_grid(&grid),
    };
    queue.push_front(current);

    let mut distance_to_start:usize = usize::MIN;
    while !queue.is_empty() {
        current = queue.pop_front().unwrap();

        visited_pos.insert(current.loc);
        if current.pipe == 'S' {
            println!("found start location");
            distance_to_start = current.length + 1;
            break;

        }

        let pipe_shape_lookup = current.loc.shape_from_grid(&grid);
        let pipe_shape = current.pipe;
        assert_eq!(pipe_shape,pipe_shape_lookup);

        let adj = connects_to(pipe_shape, current.loc);
        let adj_shapes = [adj[0].shape_from_grid(&grid), adj[1].shape_from_grid(&grid)];
        for i in 0..2 {
            let n_loc = adj[i];
            if visited_pos.contains(&n_loc) {
                continue;
            } else {
                let ch = n_loc.shape_from_grid(&grid);
                let good_connection = check_from_to((current.loc,pipe_shape), (n_loc,ch));
                if !good_connection {
                    continue;
                } else {
                    let new_state = State {
                        loc: n_loc,
                        last_loc: current.loc,
                        length: current.length + 1,
                        pipe: ch
                    };
                    queue.push_back(new_state);
                }
            }
        }
    }
    println!("{:?}", current);


    let mut answer: usize = current.length;
    return answer.to_string();
}

fn check_from_to((loc,c_pipe): (Coord, char), (n_loc,n_pipe): (Coord, char)) -> bool {
   println!("checking pipe {c_pipe}@{loc} connects to {n_pipe}@{n_loc}");

    if c_pipe == '.' || n_pipe == '.' {
        // neither should be grass
        println!("\t\t grass");
        return false;
    }
    let start_facing = pipe_to_facing(c_pipe);
    let other_facing = pipe_to_facing(n_pipe);

    let offset = n_loc.offset_from(loc);

    assert_ne!(offset, (0,0));
    let f = match offset {
        (0,1) => { Facing::Down }
        (0,-1) => { Facing::Up}
        (1,0) => {Facing::Right}
        (-1,0) => {Facing::Left}
        _ => {panic!("unable to determine facing for offset {:?}", offset);}
    };
    let f = f.oppose();
    if other_facing.0 == f || other_facing.1 == f {
        println!("\t\t good");
        return true;
    } else {
        println!("\t\t bad");
        return false;
    }
}

fn part2(input_file: &str) -> String {
    let lines = file_to_lines(input_file);

    let mut answer: usize = 0;


    return answer.to_string();
}