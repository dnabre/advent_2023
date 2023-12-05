#![allow(unused_variables)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(dead_code)]
#![allow(unused_assignments)]

/*
    Advent of Code 2023: Day 05
        part1 answer:
        part2 answer:
 */

use std::collections::{HashSet, VecDeque};
use std::fmt::{Debug, Display, Formatter};
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::str::FromStr;
use std::time::Instant;

const ANSWER: (&str, &str) = ("18653", "5921508");

fn main() {
    let _filename_test = "data/day05/test_input_01.txt";
    let _filename_test2 = "data/day05/test_input_02.txt";

    let filename_part1 = "data/day05/part1_input.txt";
    let filename_part2 = "data/day05/part2_input.txt";

    let start1 = Instant::now();
    let answer1 = part1(_filename_test);
    let duration1 = start1.elapsed();

    let start2 = Instant::now();
    let answer2 = part2(_filename_test2);
    let duration2 = start2.elapsed();

    //println!("Advent of Code, Day 04");
    println!("    ---------------------------------------------");

    println!("\t Part 1: {:14} time: {:?}", answer1, duration1);
    // if ANSWER.0 != answer1 {
    //     println!("\t\t ERROR: Answer is WRONG. Got: {}, Expected {}", answer1, ANSWER.0);
    // }
    //
    // println!("\t Part 2: {:14} time: {:?}", answer2, duration2);
    // if ANSWER.1 != answer2 {
    //     println!("\t\t ERROR: Answer is WRONG. Got: {}, Expected {}", answer2, ANSWER.1);
    // }
    // println!("    ---------------------------------------------");
}

//noinspection DuplicatedCode
fn file_to_lines(input_file: &str) -> Vec<String> {
    let file = File::open(input_file).expect(&*format!("error opening file {}", input_file));
    let bfile = BufReader::new(file);
    let lines: Vec<String> = bfile.lines().filter_map(|x| x.ok()).collect();
    return lines;
}

//noinspection DuplicatedCode
fn parse_number_list_whitespace<T:FromStr>(number_string: &str) -> Vec<T>
{
    let oo = number_string.split_whitespace().map(|s| s.trim().parse());
    let un_oo:Vec<T> = oo.map( |r| match r {
        Ok(n) => {n},
        Err(_) => {panic!("Error parsing")}
    }).collect();
    return un_oo;
}

#[derive(Debug, Copy, Clone, PartialEq,Eq, Hash)]
struct Range {
    dest_start:i64,
    source_start:i64,
    range_length:i64
}

impl Display for Range {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "[{},{}] -> [{}, {}]  (length = {})",
               self.source_start,
               self.source_start+self.range_length,
               self.dest_start,
               self.dest_start+self.range_length,
             self.range_length)
    }
}

impl Range {
    pub fn new(values:Vec<i64>) -> Self {
        Self { dest_start: values[0], source_start: values[1], range_length: values[2] }
    }
}


#[derive(Debug, Clone, PartialEq,Eq, Hash)]
struct Mapping {
    name:String,
    ranges:Vec<Range>
}

impl Mapping {
    fn new(name: String) -> Self {
        Self { name, ranges: Vec::new() }
    }
    fn add_range_v(&mut self, range_as_vec:Vec<i64> ) {
        self.add_range(
            Range{
                dest_start: range_as_vec[0],
                source_start: range_as_vec[1],
                range_length: range_as_vec[2],
            }
        )
    }

    fn add_range(&mut self, r: Range) {
        self.ranges.push(r);
    }
}

const SEED_TO_SOIL_MAP:&'static str = "seed-to-soil map:";
const SOIL_TO_FERT_MAP:&'static str = "soil-to-fertilizer map:";
const FERT_TO_WATER_MAP:&'static str = "fertilizer-to-water map:";
const WATER_TO_LIGHT_MAP:&'static str = "water-to-light map:";
const LIGHT_TO_TEMP_MAP:&'static str = "light-to-temperature map:";
const TEMP_TO_HUMID_MAP:&'static str = "temperature-to-humidity map:";
const HUMID_TO_LOC_MAP:&'static str = "humidity-to-location map:";

fn part1(input_file: &str) -> String {
    let lines = file_to_lines(input_file);
    let mut line_queue:VecDeque<String> = VecDeque::from_iter(lines);

    let seed_line = line_queue.pop_front().unwrap();
    let (_,seed_strings) = seed_line.split_once(":").unwrap();
    let seeds:Vec<i64> = parse_number_list_whitespace(seed_strings);
    println!("seeds: {:?}", seeds);

    line_queue.pop_front();
    assert_eq!(SEED_TO_SOIL_MAP, line_queue.pop_front().unwrap());
    while !line_queue.front().unwrap().is_empty() {
        let l = line_queue.pop_front().unwrap();
        println!("\t {}", l);
    }





    return String::new();
}


fn part2(input_file: &str) -> String {
    let lines = file_to_lines(input_file);

    return String::new();
}
