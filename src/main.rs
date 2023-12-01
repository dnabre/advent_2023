/*
    Advent of Code 2023: Day 01
        part1 answer:   54630
        part2 answer:   54770

 */

use std::fs::File;
use std::io::{BufRead, BufReader};

const ANSWER: (&str, &str) = ("54630", "54770");

fn main() {
    let _filename_test  = "data/day01/test_input_01.txt";
    let _filename_test2 = "data/day01/test_input_02.txt";
    let _filename_test3 = "data/day01/test_input_03.txt";

    let filename_part1 = "data/day01/part1_input.txt";
    let filename_part2 = "data/day01/part2_input.txt";

    let answer1: String = part1(filename_part1);
    let answer2: String = part2(filename_part2);

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
    let mut sum = 0;

    for l in lines.iter() {
        let line_answer = solve_line(l);
        sum += line_answer;
    }
    return sum.to_string();
}


fn part2(input_file: &str) -> String {
    let lines = file_to_lines(input_file);
    let mut sum = 0;

    for line_number in 0..lines.len() {
        let l = &lines[line_number];
        let line_value = solve_line2(&l);
        sum += line_value;
    }

    return sum.to_string();
}


fn solve_line(input_line: &String) -> i32 {
    let letters: Vec<char> = input_line.chars().collect();
    let mut first: Option<i32> = None;
    let mut back: Option<i32> = None;

    for c in letters.iter() {
        if c.is_digit(10) {
            let c_v = (*c as i32) - ('0' as i32);
            if first.is_none() {
                first = Some(c_v);
            }
            back = Some(c_v);
        }
    }
    return (10 * first.unwrap()) + back.unwrap();
}

fn solve_line2(input: &String) -> usize {
    let words = vec![
        "1", "2", "3", "4", "5", "6", "7", "8", "9",
        "one", "two", "three", "four", "five", "six", "seven", "eight", "nine"];

    let mut forward_values: Vec<_> = Vec::new();
    let mut backward_values: Vec<_> = Vec::new();

    for i in 0..words.len() {
        let p = (i, words[i]);
        match input.find(p.1).map(|pos| (pos, p.0)) {
            None => {}
            Some(ff) => {
                forward_values.push(ff);
            }
        }
        match input.rfind(p.1).map(|pos| (pos, p.0)) {
            None => {}
            Some(bb) => {
                backward_values.push(bb);
            }
        }
    }

    let f = forward_values.iter().min().unwrap().1 as i32;
    let b = backward_values.iter().max().unwrap().1 as i32;

    // first for zero-based index
    let f = f % 9 + 1;
    let b = b % 9 + 1;
    return ((f * 10) + b) as usize;
}