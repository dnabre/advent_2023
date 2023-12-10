#![allow(unused_variables)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(dead_code)]
#![allow(unused_assignments)]
#![allow(unreachable_code)]

/*
    Advent of Code 2023: Day 08
        part1 answer:   23147
        part2 answer:

 */

use std::cmp::Ordering;
use std::collections::{HashMap};
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::str::FromStr;
use std::time::Instant;

const ANSWER: (&str, &str) = ("23147", "247899149");

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
    let answer2 = part2(_filename_test2);
    let duration2 = start2.elapsed();

  //  println!("Advent of Code, Day 08");
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

//noinspection DuplicatedCode
fn file_to_lines(input_file: &str) -> Vec<String> {
    let file = File::open(input_file).expect(&*format!("error opening file {}", input_file));
    let bfile = BufReader::new(file);
    let lines: Vec<String> = bfile.lines().filter_map(|x| x.ok()).collect();
    return lines;
}

//noinspection DuplicatedCode
fn parse_number_list_whitespace<T: FromStr>(number_string: &str) -> Vec<T> {
    let oo = number_string.split_whitespace().map(|s| s.trim().parse());
    let un_oo: Vec<T> = oo.map(|r| match r {
        Ok(n) => { n }
        Err(_) => { panic!("Error parsing") }
    }).collect();
    return un_oo;
}


fn part1(input_file: &str) -> String {
    let lines = file_to_lines(input_file);

    let mut graph:HashMap<&str,(&str,&str)> = HashMap::new();

    for i in 2..lines.len() {
        let ll = &lines[i];
        let (start, (left,right)) = parse_input(ll);
        graph.insert(start, (left,right));
    }

    let choices:Vec<char> = lines[0].chars().collect();
    let ch_list_length = choices.len();
    let mut step =0;

    let mut current = "AAA";
    loop {
        let (l,r) = graph[&current];
        let ch = choices[step%ch_list_length];
        if ch=='L' {
            current = l;
        } else {
            current = r;
        }
        step += 1;
        if current=="ZZZ" {
            break;
        }
    }



    return step.to_string();
}

fn parse_input(line:&String) -> (&str,(&str,&str)) {
    let (start, options) = line.split_once("=").unwrap();
    let (sleft, sright) = options.split_once(",").unwrap();
    let sleft = &(sleft.trim())[1..];
    let sright = &(sright.trim())[..3];
    return (start.trim(), (sleft,sright));
}



fn part2(input_file: &str) -> String {
    let lines = file_to_lines(input_file);

    return String::new();
}
