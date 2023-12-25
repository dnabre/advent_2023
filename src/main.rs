#![allow(unused_variables)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(dead_code)]
#![allow(unused_assignments)]
#![allow(unreachable_code)]



use std::collections::{BTreeSet, HashSet};
use std::f64::{MAX, MIN};
use std::fmt::{Display, Formatter};
use std::hash::Hash;
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

    let start2 = Instant::now();
    let answer2 = part2(filename_part2);
    let duration2 = start2.elapsed();

    println!("\t Part 2: {:14} time: {:?}", answer2, duration2);
    if ANSWER.1 != answer2 {
        println!("\t\t ERROR: Answer is WRONG. Got: {}, Expected {}", answer2, ANSWER.1);
    }
    println!("    ---------------------------------------------");
}

#[derive(Debug, Clone, Copy)]
struct Point {
    x: f64,
    y: f64,
    z: f64,
}
impl From<&str> for Point {
    fn from(input: &str) -> Self {
        // We just split by comma, but we also trim because of extra
        // whitespace in the input.
        let mut split = input.split(", ");
        let x = split.next().unwrap().trim().parse().unwrap();
        let y = split.next().unwrap().trim().parse().unwrap();
        let z = split.next().unwrap().trim().parse().unwrap();
        Self { x, y, z }
    }
}


#[derive(Debug,  Clone, Copy)]
struct HailStone {
    position: Point,
    velocity: Point,
}

impl From<&str> for HailStone {
    fn from(input: &str) -> Self {
        // We can just split by the @ symbol.
        let mut split = input.split(" @ ");
        let position = split.next().unwrap().into();
        let velocity = split.next().unwrap().into();
        Self { position, velocity }
    }
}

// Our intersect_xy will return this. It's possible they are the same
// line and as such they intersect at all points.
enum Intersection {
    Point(Point),
    All,
}


impl HailStone {
    fn intersection_xy(&self, other: &HailStone) -> Option<Intersection> {
        // This is all based off of solving y = mx + b for two
        // lines. We first need to find the lines and then solve for
        // where they intersect.

        // Calculate the slope and intercept for each vector.
        let slope_self = self.velocity.y / self.velocity.x;
        let slope_other = other.velocity.y / other.velocity.x;
        let intercept_self = self.position.y - slope_self * self.position.x;
        let intercept_other = other.position.y - slope_other * other.position.x;

        // If they are the same line, then they always
        // intersect. Otherwise, if the slopes are the same, then they
        // never intersect because they are parallel.
        if slope_self == slope_other && intercept_self == intercept_other {
            return Some(Intersection::All);
        } else if slope_self == slope_other {
            return None;
        }

        // Solve for x and y.
        let x = (intercept_other - intercept_self) / (slope_self - slope_other);
        let y = slope_self * x + intercept_self;
        Some(Intersection::Point(Point { x, y, z: 0.0 }))
    }

    fn in_past_xy(&self, point: &Point) -> bool {
        // We want to check if the point of intersection would be in
        // the past. The simplest way to do this is to figure out the
        // number of steps to get to the point and make sure it's not
        // negative.
        let x = point.x - self.position.x;
        let y = point.y - self.position.y;

        let x = x / self.velocity.x;
        let y = y / self.velocity.y;

        x < 0.0 && y < 0.0
    }
}







#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
struct Vector3 {
    x: i128,
    y: i128,
    z: i128,
}

impl Vector3 {
    fn array(&self) -> [i128;3] {
        return [self.x, self.y, self.z]
    }
}

impl Vector3 {
    fn times_scalr(&self, v: i128) -> Vector3 {
        Vector3 {
            x: self.x * v,
            y: self.y * v,
            z: self.z * v,
        }
    }
    fn add_v(&self, other:Vector3) -> Vector3 {
        Vector3 {
            x: self.x + other.x,
            y: self.y + other.y,
            z: self.z + other.z,
        }
    }
}

impl Display for Vector3 {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f,"[{}, {}, {} ]", self.x, self.y, self.z)
    }
}







fn parse_line(input: &String) -> (Vector3, Vector3) {
    let mut position:Vector3;
    let mut velocity:Vector3;

    let (p,v) = input.split_once("@").unwrap();
    let pp:Vec<i128> = advent_2023::parse_number_list_comma(p);
    let vv:Vec<i128> = advent_2023::parse_number_list_comma(v);

    let position = Vector3{ x: pp[0], y: pp[1], z: pp[2], };
    let velocity = Vector3{ x: vv[0], y: vv[1], z: vv[2] };

    return (position,velocity);
}

const PART1_XY_BOUNDS:(i128,i128) = (200000000000000, 400000000000000);


fn part1(input_file: &str) -> String {
    let lines = advent_2023::file_to_lines(input_file);

    let mut pos:Vec<Vector3> = Vec::new();
    let mut vel:Vec<Vector3> = Vec::new();

    let mut hail_stones = Vec::new();

    let number = lines.len();
    for i in 0..number{
        let l = &lines[i];
        let (mut p,mut v) = parse_line(l);
        p.z = 0; v.z = 0;
        hail_stones.push((p,v));
    }

    let pair_up = advent_2023::list_to_pairs(hail_stones);

    let intersects = pair_up.iter()
        .filter(|(a,b)| xy_cross(*a,*b,PART1_XY_BOUNDS)).count();

    let answer = intersects;
    return answer.to_string();
}

fn xy_cross((pos1,vel1): (Vector3, Vector3), (pos2,vel2): (Vector3, Vector3), area: (i128, i128)) -> bool {
    let [px1, py1, _] = pos1.array();
    let [vx1, vy1, _] = vel1.array();
    let [px2, py2, _] = pos2.array();
    let [vx2, vy2, _] = vel2.array();
    let vv = vy2 * vx1 - vy1 * vx2;

    let x = (py1 - py2) * vx1 * vx2 - vy1 * vx2 * px1 + vy2 * vx1 * px2;
    let x = x as f64 / vv as f64;
    let future = if vx1 > 0 {
        x >= px1 as f64
    } else {
        x <= px1 as f64
    };

    if !future {
        return false;
    }
    let future = if vx2 > 0 {
        x >= px2 as f64
    } else {
        x <= px2 as f64
    };
    if !future {
        return false;
    }
    let y = (px1 - px2) * vy1 * vy2 - vx1 * vy2 * py1 + vx2 * vy1 * py2;
    let y = y as f64 / -vv as f64;
    let future = if vy1 > 0 {
        y >= py1 as f64
    } else {
        y <= py1 as f64
    };
    if !future {
        return false;
    }
    let future = if vy2 > 0 {
        y >= py2 as f64
    } else {
        y <= py2 as f64
    };
    if !future {
        return false;
    }
    let area = (area.0 as f64, area.1 as f64);
    area.0 <= x && x <= area.1 && area.0 <= y && y <= area.1
}




fn intersects(
    (pos1,vel1):(Vector3,Vector3), (pos2,vel2):(Vector3,Vector3)) ->Option <(f64,f64)>
{


return None;
}



fn part2(input_file: &str) -> String {
    let lines = advent_2023::file_to_lines(input_file);

    //unknowns initial pos and speed

    let lines = advent_2023::file_to_lines(input_file);

    let mut pos:Vec<Vector3> = Vec::new();
    let mut vel:Vec<Vector3> = Vec::new();

    let mut hail_stones = Vec::new();

    let number = lines.len();
    for i in 0..number{
        let l = &lines[i];
        let (mut p,mut v) = parse_line(l);
        hail_stones.push((p,v));
    }










    let answer = 0;
    return answer.to_string();
}



