#![allow(unused_variables)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(dead_code)]
#![allow(unused_assignments)]


/*
    Advent of Code 2023: Day 02
        part1 answer:
        part2 answer:

 */

use std::fs::File;
use std::io::{BufRead, BufReader};

const ANSWER: (&str, &str) = ("54630", "54770");

fn main() {
    let filename_test  = "data/day02/test_input_01.txt";
    let filename_test2 = "data/day02/test_input_02.txt";
    let filename_test3 = "data/day02/test_input_03.txt";

    let filename_part1 = "data/day02/part1_input.txt";
    let filename_part2 = "data/day02/part2_input.txt";

    let answer1: String = part1(filename_test);
    let answer2: String = part2(filename_test);

    println!("Advent of Code, Day 01");
    println!("    ---------------------------------------------");
    println!("\t Part 1: {}", answer1);
    if ANSWER.0 != answer1 {
        println!("\t\t ERROR: Answer is WRONG. Got: {}, Expected {}", answer1, ANSWER.0);
    }
    println!("\t Part 2: {}", answer2);
    if ANSWER.1 != answer2 {
        println!("\t\t ERROR: Answer is WRONG. Got: {}, Expected {}", answer2, ANSWER.1);
    }

    println!("    ---------------------------------------------");

}


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

