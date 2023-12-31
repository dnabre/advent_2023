/*
    Advent of Code 2023: Day 08
        part1 answer:   23147
        part2 answer:   22289513667691

 */

use std::collections::{HashMap, HashSet};
use std::time::Instant;

use advent_2023::file_to_lines;

const ANSWER: (&str, &str) = ("23147", "22289513667691");

fn main() {
    let _filename_test = "data/day08/test_input_01.txt";
    let _filename_test2 = "data/day08/test_input_02.txt";
    let _filename_test3 = "data/day08/test_input_03.txt";

    let filename_part1 = "data/day08/part1_input.txt";
    let filename_part2 = "data/day08/part2_input.txt";

    let start1 = Instant::now();
    let answer1 = part1(filename_part1);
    let duration1 = start1.elapsed();

    let start2 = Instant::now();
    let answer2 = part2(filename_part2);
    let duration2 = start2.elapsed();

    println!("Advent of Code, Day 08");
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


fn part1(input_file: &str) -> String {
    let lines = file_to_lines(input_file);

    let mut graph: HashMap<&str, (&str, &str)> = HashMap::new();

    for i in 2..lines.len() {
        let ll = &lines[i];
        let (start, (left, right)) = parse_input(ll);
        graph.insert(start, (left, right));
    }

    let choices: Vec<char> = lines[0].chars().collect();
    let ch_list_length = choices.len();
    let mut step = 0;

    let mut current = "AAA";
    loop {
        let (l, r) = graph[&current];
        let ch = choices[step % ch_list_length];
        if ch == 'L' {
            current = l;
        } else {
            current = r;
        }
        step += 1;
        if current == "ZZZ" {
            break;
        }
    }

    return step.to_string();
}

fn part2(input_file: &str) -> String {
    let lines = file_to_lines(input_file);

    let mut graph: HashMap<&str, (&str, &str)> = HashMap::new();

    let mut start_points: Vec<&str> = Vec::new();
    let mut end_points: HashSet<&str> = HashSet::new();

    for i in 2..lines.len() {
        let ll = &lines[i];
        let (start, (left, right)) = parse_input(ll);
        if start.ends_with("A") {
            start_points.push(start);
        }
        if start.ends_with("Z") {
            end_points.insert(start);
        }
        graph.insert(start, (left, right));
    }

    let choices: Vec<char> = lines[0].chars().collect();
    let ch_list_length = choices.len();

    let mut end_steps: Vec<usize> = Vec::new();
    for i in 0..start_points.len() {
        let mut step = 0;
        let mut current = start_points[i];
        loop {
            let (l, r) = graph[&current];
            let ch = choices[step % ch_list_length];
            if ch == 'L' {
                current = l;
            } else {
                current = r;
            }
            step += 1;
            if current.ends_with("Z") {
                break;
            }
        }
        end_steps.push(step);
    }

    let answer = advent_2023::lcm(end_steps.as_slice());
    return answer.to_string();
}

fn parse_input(line: &String) -> (&str, (&str, &str)) {
    let (start, options) = line.split_once("=").unwrap();
    let (sleft, sright) = options.split_once(",").unwrap();
    let sleft = &(sleft.trim())[1..];
    let sright = &(sright.trim())[..3];
    return (start.trim(), (sleft, sright));
}



