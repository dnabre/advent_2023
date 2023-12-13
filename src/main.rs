#![allow(unused_variables)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(dead_code)]
#![allow(unused_assignments)]
#![allow(unreachable_code)]

/*
    Advent of Code 2023: Day 13
        part1 answer:
        part2 answer:

 */


use std::collections::{HashSet, VecDeque};
use std::fmt::{Display, Formatter};
use std::time::Instant;
use advent_2023::file_to_lines;


const ANSWER: (&str, &str) = ("7173", "291");

fn main() {
    let _filename_test1 = "data/day13/test_input_01.txt";
    let _filename_test2 = "data/day13/test_input_02.txt";

    let filename_part1 = "data/day13/part1_input.txt";
    let filename_part2 = "data/day13/part2_input.txt";

    let start1 = Instant::now();
    let answer1 = part1(filename_part1);
    let duration1 = start1.elapsed();

    let start2 = Instant::now();
    let answer2 = part2(filename_part2);
    let duration2 = start2.elapsed();

   // println!("Advent of Code, Day 13");
    println!("    ---------------------------------------------");

    println!("\t Part 1: {:14} time: {:?}", answer1, duration1);
    if ANSWER.0 != answer1 {
        println!("\t\t ERROR: Answer is WRONG. Got: {}, Expected {}", answer1, ANSWER.0);
    }
    //
    // println!("\t Part 2: {:14} time: {:?}", answer2, duration2);
    // if ANSWER.1 != answer2 {
    //     println!("\t\t ERROR: Answer is WRONG. Got: {}, Expected {}", answer2, ANSWER.1);
    // }
    println!("    ---------------------------------------------");
}

fn part1(input_file: &str) -> String {
    let lines = file_to_lines(input_file);


    let answer = 0;
    return answer.to_string();
}


fn part2(input_file: &str) -> String {
    let lines = file_to_lines(input_file);


    let answer = 0;
    return answer.to_string();
}
