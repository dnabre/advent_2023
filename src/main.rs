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
use std::collections::{BinaryHeap, HashSet};
use std::fmt::Debug;
use std::hash::Hash;
use std::time::Instant;

use advent_2023::Direction;

const ANSWER: (&str, &str) = ("814", "974");

fn main() {
    let _filename_test = "data/day18/test_input_01.txt";
    let _filename_test2 = "data/day18/test_input_02.txt";
    let _filename_test3 = "data/day18/test_input_03.txt";

    let filename_part1 = "data/day18/part1_input.txt";
    let filename_part2 = "data/day18/part2_input.txt";

    let start1 = Instant::now();
    let answer1 = part1(filename_part1);
    let duration1 = start1.elapsed();

    let start2 = Instant::now();
    let answer2 = part2(filename_part2);
    let duration2 = start2.elapsed();

    // println!("Advent of Code, Day 18");
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


fn part1(input_file: &str) -> String {
    let lines = advent_2023::file_to_lines(input_file);

    let answer = 0;
    return answer.to_string();

}
fn part2(input_file: &str) -> String {
    let lines = advent_2023::file_to_lines(input_file);

    let answer = 0;
    return answer.to_string();

}
