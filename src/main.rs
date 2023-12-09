#![allow(unused_variables)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(dead_code)]
#![allow(unused_assignments)]
#![allow(unreachable_code)]

/*
    Advent of Code 2023: Day 07
        part1 answer:
        part2 answer:

 part1: 248455100  - too high

 */


use std::collections::{HashMap, HashSet};
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::io::ErrorKind::AddrNotAvailable;
use std::str::FromStr;
use std::time::Instant;



const ANSWER: (&str, &str) = ("245794640", "247899149");

fn main() {
    let _filename_test = "data/day07/test_input_01.txt";
    let _filename_test2 = "data/day07/test_input_02.txt";

    let filename_part1 = "data/day07/part1_input.txt";
    let filename_part2 = "data/day07/part2_input.txt";

    let start1 = Instant::now();
    let answer1 = part1(_filename_test);
    let duration1 = start1.elapsed();

    let start2 = Instant::now();
    let answer2 = part2(_filename_test2);
    let duration2 = start2.elapsed();

    // println!("Advent of Code, Day 07");
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
enum Score {
    HighCard(i32),
    OnePair(i32),
    TwoPair(i32,i32),      // first will be higher rank
    ThreeOfKind(i32),
    FullHouse(i32,i32),    // first is triple, second pair
    FourOfKind(i32),
    FiveOfKind(i32)
}


const CARD_VALUES: [char; 13] = ['2','3','4','5','6','7','8','9','T','J','Q','K','A'];


fn part1(input_file: &str) -> String {
    let lines = file_to_lines(input_file);
 //   let card_values = ['2','3','4','5','6','7','8','9','T','J','Q','K','A'];

    println!("{}", lines[0]);

    let mut char_to_index:HashMap<char,usize> = HashMap::with_capacity(13);
    let mut index:usize = 0;
    for c in '2'..='9' {
        char_to_index.insert(c,index);
        index+=1;
    }
    char_to_index.insert('T', index);  index+=1;
    char_to_index.insert('J', index); index+=1;
    char_to_index.insert('Q', index); index+=1;
    char_to_index.insert('K', index); index+=1;
    char_to_index.insert('A', index);


    println!("indices: {:?}", char_to_index);

    let mut scored_hands:Vec<(Score,i32)> = Vec::new();

    let mut hand: [i32; 13]=  [0; 13];
   // println!("{:?}", hand);
    for c in lines[0].chars() {
        hand=  [0; 13];



    }



    return String::new();
}



fn part2(input_file: &str) -> String {
    let lines = file_to_lines(input_file);

    return String::new();
}

