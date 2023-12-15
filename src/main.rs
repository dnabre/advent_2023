#![allow(unused_variables)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(dead_code)]
#![allow(unused_assignments)]
#![allow(unreachable_code)]

/*
    Advent of Code 2023: Day 15
        part1 answer:   521341
        part2 answer:

 */

use std::path::Component::ParentDir;
use std::time::Instant;

use advent_2023::file_to_lines;

const ANSWER: (&str, &str) = ("521341", "32312");

fn main() {
    let _filename_test1 = "data/day15/test_input_01.txt";
    let _filename_test2 = "data/day15/test_input_02.txt";

    let filename_part1 = "data/day15/part1_input.txt";
    let filename_part2 = "data/day15/part2_input.txt";

    let start1 = Instant::now();
    let answer1 = part1(filename_part1);
    let duration1 = start1.elapsed();

    let start2 = Instant::now();
    let answer2 = part2(_filename_test2);
    let duration2 = start2.elapsed();

  //  println!("Advent of Code, Day 15");
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


fn part1(input_file: &str) -> String {
    let l = advent_2023::file_to_single_line(input_file,None);
    let parts = l.split(",").collect::<Vec<_>>();

    let mut sum:u64 = 0;
    for p in &parts
    {
        let h = calculate_hash(p);
     //   println!("{} becomes {}", p, h);
        sum += h as u64;

    }



    return sum.to_string();
}

fn calculate_hash(s: &str) -> u8 {
    let mut result:u32 = 0;
    for ch in s.chars() {
        let ascii_value = ch as i8;
  //      println!("char: {ch} it's ASCII code is: {}", ascii_value);
        result += ascii_value as u32;
  //      println!("current value is increases to {}", result);
        result *= 17_u32;
  //      println!("current value is multiplied by 17 to become {}", result);
        result = result % 256;
  //      println!("current value is becomes  {}", result);
    }
    assert_eq!(true, result <256);
    return result as u8;
}

fn part2(input_file: &str) -> String {
    let lines = file_to_lines(input_file);

    let answer = 0;
    return answer.to_string();
}

