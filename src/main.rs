#![allow(unused_variables)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(dead_code)]
#![allow(unused_assignments)]
#![allow(unreachable_code)]



use std::fmt::{Display, Formatter};
use std::time::Instant;


/*
    Advent of Code 2023: Day 24
        part1 answer:   25810
        part2 answer:

*/
const ANSWER: (&str, &str) = ("25810", "74594");

fn main() {
    let _filename_test = "data/day24/test_input_01.txt";
    let _filename_test2 = "data/day24/test_input_02.txt";

    let filename_part1 = "data/day24/part1_input.txt";
    let filename_part2 = "data/day24/part2_input.txt";

    println!("Advent of Code, Day 24");
    println!("    ---------------------------------------------");
    let start1 = Instant::now();
    let answer1 = part1(filename_part1);
    let duration1 = start1.elapsed();

    println!("\t Part 1: {:14} time: {:?}", answer1, duration1);
    if ANSWER.0 != answer1 {
        println!("\t\t ERROR: Answer is WRONG. Got: {}, Expected {}", answer1, ANSWER.0);
    }
    //
    // let start2 = Instant::now();
    // let answer2 = part2(filename_part2);
    // let duration2 = start2.elapsed();
    //
    // println!("\t Part 2: {:14} time: {:?}", answer2, duration2);
    // if ANSWER.1 != answer2 {
    //     println!("\t\t ERROR: Answer is WRONG. Got: {}, Expected {}", answer2, ANSWER.1);
    // }
    println!("    ---------------------------------------------");
}

#[derive(Debug, Clone, Copy)]
struct BoundingBox {
    min_x: i64,
    max_x: i64,
    min_y: i64,
    max_y: i64,
}
impl BoundingBox {
    fn new(min_x: i64, max_x: i64, min_y: i64, max_y: i64) -> Self {
        Self {
            min_x,
            max_x,
            min_y,
            max_y,
        }
    }

    fn contains(&self, point: &Point) -> bool {
        let Point { x, y, .. } = *point;
        self.min_x <= x && x <= self.max_x && self.min_y <= y && y <= self.max_y
    }
}

#[derive(Debug, Clone, Copy, Hash)]
struct Point {
    x: i64,
    y: i64,
    z: i64
}


impl Point {
    fn new(x: i64, y: i64, z: i64) -> Self {
        return Self{ x, y, z}
    }
}
#[derive(Debug, Clone, Copy, Hash)]
struct HailStone {
    position: Point,
    velocity: Point,
}



impl HailStone {
    fn new(position: Point, velocity: Point) -> Self {
        Self { position, velocity }
    }


    fn intersection_xy(&self, other: &Self) -> Option<Point> {
        let (x0, y0) = (self.position.x, self.position.y);
        let (x1, y1) = (other.position.x, other.position.y);

        let (vx0, vy0) = (self.velocity.x, self.velocity.y);
        let (vx1, vy1) = (other.velocity.x, other.velocity.y);

        let c0 = x1 - x0;
        let c1 = y1 - y0;
        let (a0, b0) = (vx0, -vy1);
        let (a1, b1) = (vy0, -vy1);

        if let Some((s, t)) = solve_linear(
            (a0 as f64, b0 as f64, c0 as f64),
            (a1 as f64, b1 as f64, c1 as f64),
        ) {
            if s >= 0.0 && t >= 0.0 {
                let xs = x0 as f64 + vx0 as f64 * s;
                let ys = y0 as f64 + vy0 as f64 * s;
                let xt = x1 as f64 + vx1 as f64 * t;
                let yt = y1 as f64 + vy1 as f64 * t;

                let x = (xs + xt) / 2.0;
                let y = (ys + yt) / 2.0;

                Some(Point::new(x as i64, y as i64, 0))
            } else {
                None
            }
        } else {
            None
        }
    }
}

fn solve_linear((a1,b1,c1): (f64, f64, f64), (a2,b2,c2): (f64, f64, f64)) -> Option<(f64,f64)> {
    let det = a1 * b2 - a2 * b1;
    if det == 0.0 {
        None
    } else {
        let x = (b2 * c1 - b1 * c2) /det;
        let y = (a1 * c2 - a2 * c1) /det;
        Some((x,y))
    }
}


fn parse_line(input: &String) -> HailStone {
    let mut position:Point;
    let mut velocity:Point;

    let (p,v) = input.split_once("@").unwrap();
    let pp:Vec<i64> = advent_2023::parse_number_list_comma(p);
    let vv:Vec<i64> = advent_2023::parse_number_list_comma(v);

    let position = Point{ x: pp[0], y: pp[1], z: pp[2], };
    let velocity = Point{ x: vv[0], y: vv[1], z: vv[2] };

    return HailStone::new(position,velocity);
}



fn part1(input_file: &str) -> String {
    let lines = advent_2023::file_to_lines(input_file);
    let hail_stone:Vec<HailStone> = lines.iter().map(|l| parse_line(l)).collect();


    let bounding_box = BoundingBox::new(
        200000000000000,
        400000000000000,
        200000000000000,
        400000000000000,
    );

    let mut count =0;
    for i in 0..hail_stone.len() {
        for j in i+1 .. hail_stone.len() {
            let hail_stone1 = &hail_stone[i];
            let hail_stone2 = &hail_stone[j];
            if let Some(intersection) = hail_stone1.intersection_xy(hail_stone2) {
                if bounding_box.contains(&intersection) {
                    count += 1;
                }
            }

        }
    }

    let answer = count;
    return answer.to_string();
}






fn part2(input_file: &str) -> String {
    let lines = advent_2023::file_to_lines(input_file);











    let answer = 0;
    return answer.to_string();
}



