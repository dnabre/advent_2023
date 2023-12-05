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
    let answer2 = part2(filename_part2);
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
    fn inverse(&self, value: i64) -> i64 {
        for i in 0..self.dest_range.len() {
            if self.dest_range[i].contains(&value) {
           //     println!("\tvalue {value} is in dest_range: {:?}", self.dest_range);
                let offset = value - self.dest_range[i].start;
            //    println!("\t offset: {offset} returning {} + {}", self.source_range[i].start , offset);

                return self.source_range[i].start + offset;
            }
        }
      //  println!("inverse of {value} is fall through");
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

    let seed_pair_chunks = seeds.chunks(2);

    let mut seed_ranges:Vec<Range<i64>> = Vec::new();

    for pair in seed_pair_chunks {
        let a = pair[0];
        let b = pair[1];
        let r = a..(a+b);
        seed_ranges.push(r);
    }
    //
    // let irange_a = (seeds[0], seeds[0] + seeds[1]);
    // let irange_b = (seeds[2], seeds[2] + seeds[3]);
    //
    // let range_a = irange_a.0..irange_a.1;
    // let range_b = irange_b.0..irange_b.1;
    //

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

    let mut answer:i64 = 0;
    let mut min_loc: i64 = i64::MAX;
    let loop_max:i64 = 1024*1024 * 1024;
    for l in 0..loop_max {


            let mut value = l;
            let mut n_value = l;

            for i in (0..map_vec.len()).rev() {
                let map = &map_vec[i];

                n_value = map.inverse(value);
            //    println!("{:30} maps {:15} to {:15} ",map.name, value, n_value );

                value = n_value;
            }

        for r in &seed_ranges
        {
            if r.contains(&value) {
                println!("seed {value} maps to lowest location {l}");
                return l.to_string();
            }
        }
        //println!("loc: {l} reverse maps to seed {}", value);
        // let saved_seed =value;
        // if range_a.contains(&value) { println!("seed {value} in range a");}
        // if range_b.contains(&value) { println!("seed {value} in range b");}

        // let mut value = value;
        // let mut n_value = value;
        //
        // for i in 0..map_vec.len() {
        //     let map = &map_vec[i];
        //     n_value = map.map(value);
        //     value = n_value;
        // }
        // println!("\t checking seed: {} maps to loc: {}", saved_seed, value);

    }




        return answer.to_string();
    }
