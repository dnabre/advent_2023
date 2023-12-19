#![allow(unused_variables)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(dead_code)]
#![allow(unused_assignments)]
#![allow(unreachable_code)]

/*
    Advent of Code 2023: Day 05
        part1 answer:
        part2 answer:
 */

use std::cmp::Ordering;
use std::collections::{BinaryHeap, HashMap, HashSet, VecDeque};
use std::fmt::{Debug, Display, Formatter};
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::ops::Range;
use std::path::Component::ParentDir;
use std::str::FromStr;
use std::time::Instant;
use advent_2023::Compass;

const ANSWER: (&str, &str) = ("535088217", "51399228");

fn main() {
    let _filename_test = "data/day17/test_input_01.txt";
    let _filename_test2 = "data/day17/test_input_02.txt";

    let filename_part1 = "data/day17/part1_input.txt";
    let filename_part2 = "data/day17/part2_input.txt";

    let start1 = Instant::now();
    let answer1 = part1(_filename_test);
    let duration1 = start1.elapsed();

    let start2 = Instant::now();
    let answer2 = part2(_filename_test2);
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

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
struct State {
    pos: (i32,i32),
    heat_lost: i32,
    direction:Compass,
    inertia: i32
}

impl Ord for State {
    fn cmp(&self, other: &Self) -> Ordering {
        self.heat_lost.cmp(&other.heat_lost).reverse()
    }
}

impl PartialOrd for State {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

fn part1(input_file: &str) -> String {
    let lines = advent_2023::file_to_lines(input_file);
    let mut grid: Vec<Vec<i32>> = Vec::new();
    let mut queue:BinaryHeap<State> = BinaryHeap::new();
    for l in &lines {
        let mut num_lines: Vec<i32> = l.chars().map(|ch| (ch as i32) - ('0' as i32)).collect();

        grid.push(num_lines);
    }

    let start_point = (0,0);
    let goal_point:(i32,i32)= ((grid[0].len()-1) as i32,( grid.len() -1) as i32);
    println!("finding path from {:?} to {:?} minimizing heat lost", start_point, goal_point);
    advent_2023::print_grid(&grid);

    let mut start_state = State {
        pos: (0, 0),
        heat_lost: 0,
        direction: Compass::East,
        inertia: 0,
    };

    queue.push(start_state);

    let mut current_state  = start_state;
    while !queue.is_empty()  {
        current_state = queue.pop().unwrap();
        if current_state.pos == goal_point {
            break;
       }
        // expand frontier from current_state




    }
    println!("minimum heat lost: {}", current_state.heat_lost);




    let answer = 0;
    return answer.to_string();
}


fn part2(input_file: &str) -> String {
    let lines = advent_2023::file_to_lines(input_file);
    let answer = 0;

    return answer.to_string();
}
