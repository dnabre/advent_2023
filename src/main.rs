#![allow(unused_variables)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(dead_code)]
#![allow(unused_assignments)]
#![allow(unreachable_code)]


use std::collections::{BTreeMap, HashMap};
use std::fmt::Display;
use std::hash::Hash;
use std::time::Instant;

/*
    Advent of Code 2023: Day 22
        part1 answer:
        part2 answer:

*/

const ANSWER: (&str, &str) = ("825896364", "243566897206981");


fn main() {
    let _filename_test = "data/day22/test_input_01.txt";
    let _filename_test2 = "data/day22/test_input_02.txt";

    let filename_part1 = "data/day22/part1_input.txt";
    let filename_part2 = "data/day22/part2_input.txt";

    // println!("Advent of Code, Day 22");
    println!("    ---------------------------------------------");
    let start1 = Instant::now();
    let answer1 = part1(_filename_test);
    let duration1 = start1.elapsed();

    println!("\t Part 1: {:14} time: {:?}", answer1, duration1);
    if ANSWER.0 != answer1 {
        println!("\t\t ERROR: Answer is WRONG. Got: {}, Expected {}", answer1, ANSWER.0);
    }

    let start2 = Instant::now();
    let answer2 = part2(filename_part2);
    let duration2 = start2.elapsed();

    // println!("\t Part 2: {:14} time: {:?}", answer2, duration2);
    // if ANSWER.1 != answer2 {
    //     println!("\t\t ERROR: Answer is WRONG. Got: {}, Expected {}", answer2, ANSWER.1);
    // }
    println!("    ---------------------------------------------");
}

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
struct Coord {
    x: i32,
    y: i32,
    z: i32,
}

#[derive(Debug, Clone, Copy)]
struct Brick {
    id: usize,
    coords1: Coord,
    coords2: Coord,
}

impl Brick {
    fn move_by(&self, delta: [i32; 3]) -> Brick {
        let [dx, dy, dz] = delta;

        Self {
            id: self.id,
            coords1: Coord { x: self.coords1.x + dx, y: self.coords1.y + dy, z: self.coords1.z + dz },
            coords2: Coord { x: self.coords2.x + dx, y: self.coords2.y + dy, z: self.coords2.z + dz },
        }
    }
}

impl Brick {
    fn new(id: usize, coords1: Coord, coords2: Coord) -> Self {
        Self { id, coords1, coords2 }
    }

    fn intersects(&self, other: &Self) -> bool {
        let (x1, y1, z1) = (self.coords1.x, self.coords1.y, self.coords1.z);
        let (x2, y2, z2) = (self.coords2.x, self.coords2.y, self.coords2.z);
        let highx = x1.max(x2);
        let lowx = x1.min(x2);
        let highy = y1.max(y2);
        let lowy = y1.min(y2);
        let highz = z1.max(z2);
        let lowz = z1.min(z2);
        let (x3, y3, z3) = (other.coords1.x, other.coords1.y, other.coords1.z);
        let (x4, y4, z4) = (other.coords2.x, other.coords2.y, other.coords2.z);
        let otherhighx = x3.max(x4);
        let otherlowx = x3.min(x4);
        let otherhighy = y3.max(y4);
        let otherlowy = y3.min(y4);
        let otherhighz = z3.max(z4);
        let otherlowz = z3.min(z4);
        let xintersects = (otherlowx >= lowx && otherlowx <= highx) || (otherhighx >= lowx && otherhighx <= highx) || (lowx >= otherlowx && lowx <= otherhighx) || (highx >= otherlowx && highx <= otherhighx);
        let yintersects = (otherlowy >= lowy && otherlowy <= highy) || (otherhighy >= lowy && otherhighy <= highy) || (lowy >= otherlowy && lowy <= otherhighy) || (highy >= otherlowy && highy <= otherhighy);
        let zintersects = (otherlowz >= lowz && otherlowz <= highz) || (otherhighz >= lowz && otherhighz <= highz) || (lowz >= otherlowz && lowz <= otherhighz) || (highz >= otherlowz && highz <= otherhighz);
        xintersects && yintersects && zintersects
    }
    fn get_height_above_ground(&self) -> i32 {
        let z1 = self.coords1.z;
        let z2 = self.coords2.z;
        return z1.min(z2);
    }
}




fn part1(input_file: &str) -> String {
    let lines = advent_2023::file_to_lines(input_file);

    let mut bricks = lines.iter().filter(|line| !line.is_empty())
        .map(|line| line.split_once("~").unwrap())
        .enumerate()
        .map(|(id, (coords1, coords2))| Brick::new(id, parse_coords(coords1), parse_coords(coords2)))
        .collect::<Vec<_>>();
    for i in 0..bricks.len() {
        println!("_ {i:3}= {:?}", bricks[i]);
    }

return String::new();

    let mut brick_list: Vec<Brick> = Vec::new();

    // for i in 0..lines.len() {
    //     let (p1, p2) = parse_coord_pair(&lines[i]);
    //     let b = Brick::new(i, p1, p2);
    //     brick_list.push(b);
    // }

    let brick_list = drop_bricks(brick_list);
    let mut supported_by: HashMap<usize, Vec<usize>> = HashMap::new();
    let mut supports: HashMap<usize, Vec<usize>> = HashMap::new();

    for (i,brick) in brick_list.iter().enumerate() {
        let down_brick = brick.move_by([0, 0, -1]);
        if brick.get_height_above_ground() == 1 {
            supported_by.entry(i).or_default().push(usize::MAX)
        }
    }
    let mut answer = 0u64;
    for i in 0..brick_list.len() {
        let b = brick_list[i];
        let v = &supported_by[&i];
        println!("brick[{i:3}]: {:?} -> {:?}", b, v);

        if v.len() == 1 { answer += 1; }
    }


    return answer.to_string();
}

fn drop_bricks(bricks: Vec<Brick>) -> Vec<Brick> {
    println!("dropping");
    let mut bricks = bricks;
    let mut change = true;
    while change {
        let mut new_bricks = Vec::with_capacity(bricks.len());
        change = false;
        for brick in &bricks {
            let new_brick = if brick.get_height_above_ground() > 1 {
                let potential_brick = brick.move_by([0, 0, -1]);
                let mut intersects = false;
                for other_brick in bricks.iter().filter(|b| b.id != brick.id) {
                    if potential_brick.intersects(other_brick) {
                        intersects = true;
                        break;
                    }
                }
                if intersects {
                    brick.clone()
                } else {
                    change = true;
                    potential_brick
                }
            } else {
                brick.clone()
            }
                ;
            new_bricks.push(new_brick);
        }
        bricks = new_bricks;
    }
    println!("down dropping");
    return bricks;
}

fn parse_coords(l: &str) -> Coord {
    let p1: Vec<&str> = l.split(",").collect();
    let cc1 = Coord {
        x: p1[0].parse().unwrap(),
        y: p1[1].parse().unwrap(),
        z: p1[2].parse().unwrap(),
    };
    return cc1;
}

fn part2(input_file: &str) -> String {
    let lines = advent_2023::file_to_lines(input_file);


    let answer = 0;
    return answer.to_string();
}