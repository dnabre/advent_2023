#![allow(unused_variables)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(dead_code)]
#![allow(unused_assignments)]


/*
    Advent of Code 2023: Day 03
        part1 answer:
        part2 answer:

 */

use std::fs::File;
use std::io::{BufRead, BufReader};

const ANSWER: (&str, &str) = ("2369", "66363");

fn main() {
    let _filename_test = "data/day03/test_input_01.txt";
    let _filename_test2 = "data/day03/test_input_02.txt";
    let filename_part1 = "data/day03/part1_input.txt";
    let filename_part2 = "data/day03/part2_input.txt";


    println!("Advent of Code, Day ");
    println!("    ---------------------------------------------");


    let answer1: String = part1(_filename_test);

    println!("\t Part 1: {}", answer1);
    if ANSWER.0 != answer1 {
        println!("\t\t ERROR: Answer is WRONG. Got: {}, Expected {}", answer1, ANSWER.0);
    }
    let answer2: String = part2(filename_part2);
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
