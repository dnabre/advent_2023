#![allow(unused_variables)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(dead_code)]
#![allow(unused_assignments)]

/*
    Advent of Code 2023: Day 04
        part1 answer:
        part2 answer:

 */

use std::collections::{HashMap, HashSet};
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::time::Instant;

const ANSWER: (&str, &str) = ("527369", "73074886");

fn main() {
    let _filename_test = "data/day04/test_input_01.txt";
    let _filename_test2 = "data/day04/test_input_02.txt";

    let filename_part1 = "data/day04/part1_input.txt";
    let filename_part2 = "data/day04/part2_input.txt";

    let start1 = Instant::now();
    let answer1 = part1(_filename_test);
    let duration1 = start1.elapsed();

    let start2 = Instant::now();
    let answer2 = part2(_filename_test2);
    let duration2 = start2.elapsed();

    println!("Advent of Code, Day 04");
    println!("    ---------------------------------------------");
    println!("\t Part 1: {} \t\t time: {:?}", answer1, duration1);
    if ANSWER.0 != answer1 {
        println!("\t\t ERROR: Answer is WRONG. Got: {}, Expected {}", answer1, ANSWER.0);
    }
    println!("\t Part 2: {} \t\t time: {:?}", answer2, duration2);
    if ANSWER.1 != answer2 {
        println!("\t\t ERROR: Answer is WRONG. Got: {}, Expected {}", answer2, ANSWER.1);
    }
    println!("    ---------------------------------------------");
}

//noinspection DuplicatedCode
fn file_to_lines(input_file: &str) -> Vec<String> {
    let file = File::open(input_file).expect(&*format!("error opening file {}", input_file));
    let bfile = BufReader::new(file);
    let lines: Vec<String> = bfile.lines().filter_map(|x| x.ok()).collect();
    return lines;
}


fn part1(input_file: &str) -> String {
    let lines = file_to_lines(input_file);


    return String::new();
}


fn part2(input_file: &str) -> String {
    let lines = file_to_lines(input_file);


    return String::new();
}


