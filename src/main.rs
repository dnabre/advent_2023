#![allow(unused_variables)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(dead_code)]
#![allow(unused_assignments)]
#![allow(unreachable_code)]

/*
    Advent of Code 2023: Day 05
        part1 answer:   535088217
        part2 answer:
 */

use std::collections::{HashMap, HashSet, VecDeque};
use std::fmt::{Debug, Display, Formatter};
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::ops::Range;
use std::path::Component::ParentDir;
use std::str::FromStr;
use std::time::Instant;

const ANSWER: (&str, &str) = ("535088217", "51399228");

fn main() {
    let _filename_test = "data/day05/test_input_01.txt";
    let _filename_test2 = "data/day05/test_input_02.txt";

    let filename_part1 = "data/day05/part1_input.txt";
    let filename_part2 = "data/day05/part2_input.txt";

    let start1 = Instant::now();
    let answer1 = part1(filename_part1);
    let duration1 = start1.elapsed();

    let start2 = Instant::now();
    let answer2 = part2(_filename_test2);
    let duration2 = start2.elapsed();

    //println!("Advent of Code, Day 04");
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

//noinspection DuplicatedCode
fn file_to_lines(input_file: &str) -> Vec<String> {
    let file = File::open(input_file).expect(&*format!("error opening file {}", input_file));
    let bfile = BufReader::new(file);
    let lines: Vec<String> = bfile.lines().filter_map(|x| x.ok()).collect();
    return lines;
}

//noinspection DuplicatedCode
fn parse_number_list_whitespace<T: FromStr>(number_string: &str) -> Vec<T>
{
    let oo = number_string.split_whitespace().map(|s| s.trim().parse());
    let un_oo: Vec<T> = oo.map(|r| match r {
        Ok(n) => { n }
        Err(_) => { panic!("Error parsing") }
    }).collect();
    return un_oo;
}


#[derive(Debug, Clone, PartialEq, Eq, Hash)]
struct Mapping {
    name: String,
    source_range: Vec<Range<i64>>,
    dest_range: Vec<Range<i64>>,
}


impl Mapping {
    fn new(name: String) -> Self {
        Self { name, source_range: Vec::new(), dest_range: Vec::new() }
    }
    fn add_range_v(&mut self, rv: Vec<i64>) {
        self.add_range(
            rv[1]..rv[1] + rv[2],
            rv[0]..rv[0] + rv[2],
        );
    }

    fn add_range(&mut self, source: Range<i64>, dest: Range<i64>) {
        self.source_range.push(source);
        self.dest_range.push(dest);
    }


    fn map(&self, value: i64) -> i64 {
        for i in 0..self.source_range.len() {
            if self.source_range[i].contains(&value) {
                //          println!("value: {value} is in range {:?} ", self.source_range[i]);
                let offset = value - self.source_range[i].start;
                return self.dest_range[i].start + offset;
            }
        }
        //     println!("value: not found in ranges, defaulting to : {value}");
        return value;
    }
    fn map_range(&self, r: Range<i64>) -> Vec<Range<i64>> {
        // convert range r into mapped ranges. resulting ranges may be disjoint, so return vec of them

        return Vec::new();
    }
}


const SEED_TO_SOIL_MAP: &'static str = "seed-to-soil map:";
const SOIL_TO_FERT_MAP: &'static str = "soil-to-fertilizer map:";
const FERT_TO_WATER_MAP: &'static str = "fertilizer-to-water map:";
const WATER_TO_LIGHT_MAP: &'static str = "water-to-light map:";
const LIGHT_TO_TEMP_MAP: &'static str = "light-to-temperature map:";
const TEMP_TO_HUMID_MAP: &'static str = "temperature-to-humidity map:";
const HUMID_TO_LOC_MAP: &'static str = "humidity-to-location map:";

const MAP_NAMES: [&'static str; 7] = [
    SEED_TO_SOIL_MAP,
    SOIL_TO_FERT_MAP,
    FERT_TO_WATER_MAP,
    WATER_TO_LIGHT_MAP,
    LIGHT_TO_TEMP_MAP,
    TEMP_TO_HUMID_MAP,
    HUMID_TO_LOC_MAP,
];


fn part1(input_file: &str) -> String {
    let lines = file_to_lines(input_file);
    let mut line_queue: VecDeque<String> = VecDeque::from_iter(lines);

    let seed_line = line_queue.pop_front().unwrap();
    let (_, seed_strings) = seed_line.split_once(":").unwrap();
    let seeds: Vec<i64> = parse_number_list_whitespace(seed_strings);

    let mut mappings: HashMap<&str, Mapping> = HashMap::new();
    let mut map_vec: Vec<Mapping> = Vec::with_capacity(7);


    line_queue.pop_front();

    for i in 0..MAP_NAMES.len() {
        let map_name = MAP_NAMES[i];
        assert_eq!(map_name, line_queue.pop_front().unwrap());
        let mut n_map = Mapping::new(map_name.to_string());
        while !line_queue.is_empty() && !line_queue.front().unwrap().is_empty() {
            let l = line_queue.pop_front().unwrap();
            let r: Vec<i64> = parse_number_list_whitespace(&l);
            assert_eq!(r.len(), 3);
            n_map.add_range_v(r);
        }

        map_vec.push(n_map);
        line_queue.pop_front();
    }

    let mut min_loc: i64 = i64::MAX;

    for seed in seeds {
        let mut value = seed;
        let mut n_value = seed;

        for i in 0..map_vec.len() {
            let map = &map_vec[i];
            n_value = map.map(value);
            value = n_value;
        }
        // last mapping should take us to location
        min_loc = min_loc.min(value);
    }
    return min_loc.to_string();
}


/* @todo make a clear and consistent rule about whether ranges are inclusive or not. most likely
     having both ends either inclusive/exclusive would best. may need to throw out part1 code,
     but it's worthless anyway.

     Note: part1 is can be converted to part2 with four range of length 1

    Create class Bucket with lowest,highest value in range.
    Possibly make new rule that is the identity, so that values will always make some rule
 */


fn part2(input_file: &str) -> String {
    let lines = file_to_lines(input_file);
    let mut line_queue: VecDeque<String> = VecDeque::from_iter(lines);

    let seed_line = line_queue.pop_front().unwrap();
    let (_, seed_strings) = seed_line.split_once(":").unwrap();
    let seeds: Vec<i64> = parse_number_list_whitespace(seed_strings);

    let seed_range_1 = seeds[0]..seeds[0] + seeds[1];
    let seed_range_2 = seeds[2]..seeds[2] + seeds[3];


    return String::new();

    let mut current_ranges = vec![seed_range_1, seed_range_2];

    let pair_1 = (seed_range_1.start, seed_range_1.end);
    let pair_2 = (seed_range_2.start, seed_range_2.end);

    println!("seeds: ");
    println!("\t {:?} with length: {} \t start: {} end: {}", pair_1, pair_1.1 - pair_1.0, seed_range_1.start, seed_range_1.end);
    println!("\t {:?} with length: {} \t start: {} end: {}", pair_2, pair_2.1 - pair_2.0, seed_range_2.start, seed_range_2.end);


    let mut mappings: HashMap<&str, Mapping> = HashMap::new();
    let mut map_vec: Vec<Mapping> = Vec::with_capacity(7);


    line_queue.pop_front();

    for i in 0..MAP_NAMES.len() {
        let map_name = MAP_NAMES[i];
        assert_eq!(map_name, line_queue.pop_front().unwrap());
        let mut n_map = Mapping::new(map_name.to_string());
        while !line_queue.is_empty() && !line_queue.front().unwrap().is_empty() {
            let l = line_queue.pop_front().unwrap();
            let r: Vec<i64> = parse_number_list_whitespace(&l);
            assert_eq!(r.len(), 3);
            n_map.add_range_v(r);
        }
        //  mappings.insert(map_name, n_map);
        map_vec.push(n_map);
        line_queue.pop_front();
    }

    let mut min_loc: i64 = i64::MAX;
    let mut bumpy_range_list: Vec<Vec<Range<i64>>>;
    for i in 0..map_vec.len() {
        bumpy_range_list = Vec::new();
        let map = &map_vec[i];
        //      println!("\t using map: {:?}", map);

        //let bumpy_range_list:Vec<Vec<Range<i64>>> = current_ranges.iter().map(|r| map.map_range(r));
        for r in current_ranges {
            let partial_range_list = map.map_range(r);
            bumpy_range_list.push(partial_range_list);
        }

        current_ranges = Vec::new();
        for range_list in bumpy_range_list {
            for r in range_list {
                current_ranges.push(r);
            }
        }
    }

    println!("location ranges: ");
    for r in current_ranges {
        println!("\t {:?}", r);
        min_loc.min(r.start);
    }


    return min_loc.to_string();
}
