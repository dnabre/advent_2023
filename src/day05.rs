/*
    Advent of Code 2023: Day 05
        part1 answer:   535088217
        part2 answer:   51399228
 */

use std::collections::VecDeque;
use std::fmt::Debug;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::ops::Range;
use std::str::FromStr;
use std::time::Instant;

const ANSWER: (&str, &str) = ("535088217", "");

fn main() {
    let _filename_test = "data/day05/test_input_01.txt";
    let _filename_test2 = "data/day05/test_input_02.txt";

    let filename_part1 = "data/day05/part1_input.txt";
    let filename_part2 = "data/day05/part2_input.txt";

    let start1 = Instant::now();
    let answer1 = part1(filename_part1);
    let duration1 = start1.elapsed();

    let start2 = Instant::now();
    let answer2 = part2(filename_part2);
    let duration2 = start2.elapsed();

    println!("Advent of Code, Day 05");
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
fn parse_number_list_whitespace<T: FromStr>(number_string: &str) -> Vec<T> {
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
                let offset = value - self.source_range[i].start;
                return self.dest_range[i].start + offset;
            }
        };
        return value;
    }
    fn inverse(&self, value: i64) -> i64 {
        for i in 0..self.dest_range.len() {
            if self.dest_range[i].contains(&value) {
                let offset = value - self.dest_range[i].start;
                return self.source_range[i].start + offset;
            }
        }
        return value;
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

        for i in 0..map_vec.len() {
            let map = &map_vec[i];
            value = map.map(value);
        }
        // last mapping should take us to location
        min_loc = min_loc.min(value);
    }
    return min_loc.to_string();
}

fn part2(input_file: &str) -> String {
    let lines = file_to_lines(input_file);
    let mut line_queue: VecDeque<String> = VecDeque::from_iter(lines);

    let seed_line = line_queue.pop_front().unwrap();
    let (_, seed_strings) = seed_line.split_once(":").unwrap();
    let seeds: Vec<i64> = parse_number_list_whitespace(seed_strings);

    let seed_pair_chunks = seeds.chunks(2);

    let mut seed_ranges: Vec<Range<i64>> = Vec::new();

    for pair in seed_pair_chunks {
        let a = pair[0];
        let b = pair[1];
        let r = a..(a + b);
        seed_ranges.push(r);
    }

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

    let answer: i64 = 0;
    let loop_max: i64 = 1024 * 1024 * 1024;
    let mut l = 0;
    while l <= loop_max {
        l += 14821;


        let mut value = l;


        for i in (0..map_vec.len()).rev() {
            let map = &map_vec[i];
            let n_value = map.inverse(value);


            value = n_value;
        }

        for r in &seed_ranges {
            if r.contains(&value) {
                let answer = l;
                return answer.to_string();
            }
        }
    }
    return answer.to_string();
}
