#![allow(unused_variables)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(dead_code)]
#![allow(unused_assignments)]
#![allow(unreachable_code)]

/*
    Advent of Code 2023: Day 17
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
use advent_2023::{Compass, ForwardDirection};

const ANSWER: (&str, &str) = ("814", "974");

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

    // println!("\t Part 2: {:14} time: {:?}", answer2, duration2);
    // if ANSWER.1 != answer2 {
    //     println!("\t\t ERROR: Answer is WRONG. Got: {}, Expected {}", answer2, ANSWER.1);
    // }
    println!("    ---------------------------------------------");
}


enum TurningOptions {
    Straight,
    Left,
    Right
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
struct State {
    pos: (usize,usize),
    heat_lost: i32,
    direction:Compass,
    inertia: i32
}

impl State {

}



impl Ord for State {
    fn cmp(&self, other: &Self) -> Ordering {
        self.heat_lost.cmp(&other.heat_lost)
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
    for l in &lines {
        let mut num_lines: Vec<i32> = l.chars().map(|ch| (ch as i32) - ('0' as i32)).collect();

        grid.push(num_lines);
    }
    let max_rows = grid.len();
    let max_cols = grid[0].len();


    let mut queue:BinaryHeap<State> = BinaryHeap::new();

    let start_point = (0,0);
    let goal_point:(usize,usize)= ((grid[0].len()-1) ,( grid.len() -1) );
    println!("finding path from {:?} to {:?} minimizing heat lost", start_point, goal_point);
    advent_2023::print_grid(&grid);
    let mut visited:HashSet<State> = HashSet::new();

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
    //    println!("current state: {:?}   queue_size: {}, visited: {}", current_state, queue.len(), visited.len());
        visited.insert(current_state);


        if current_state.pos == goal_point {
            break;
       }
        // expand frontier from current_state
        let mut new_state = State{
            pos: (0, 0),
            heat_lost: 0,
            direction: Compass::North,
            inertia: 0,
        };

        // straight
        if current_state.inertia < 3 {
            let new_pos = advent_2023::Compass::progress(current_state.pos, current_state.direction, (max_rows,max_cols));
            if let Some((x,y)) = new_pos {
                let new_state = State {
                    pos: (x, y),
                    heat_lost: current_state.heat_lost + grid[y as usize][x as usize],
                    direction: current_state.direction,
                    inertia: current_state.inertia + 1,
                };
                if !visited.contains(&new_state) {
                queue.push(new_state);
                    }
            }
        }

        // left
        let new_direction = Compass::turn_to(current_state.direction, ForwardDirection::Left);
        let new_pos = Compass::progress(current_state.pos, new_direction, (max_rows,max_cols));
        if let Some((x,y)) = new_pos {
            let new_state = State {
                pos: (x, y),
                heat_lost: current_state.heat_lost + grid[y as usize][x as usize],
                direction: new_direction,
                inertia: 0,
            };
            if !visited.contains(&new_state) {
                queue.push(new_state);
            }
        }

        // right
        let new_direction = Compass::turn_to(current_state.direction, ForwardDirection::Right);
        let new_pos = Compass::progress(current_state.pos, new_direction, (max_rows,max_cols));
        if let Some((x,y)) = new_pos {
            let new_state = State {
                pos: (x, y),
                heat_lost: current_state.heat_lost + grid[y as usize][x as usize],
                direction: new_direction,
                inertia: 0,
            };
            if !visited.contains(&new_state) {
                queue.push(new_state);
            }
        }








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
