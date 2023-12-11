#![allow(unused_variables)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(dead_code)]
#![allow(unused_assignments)]
#![allow(unreachable_code)]

/*
    Advent of Code 2023: Day 09
        part1 answer:   2105961943
        part2 answer:   1019

 */


use std::time::Instant;

use advent_2023::file_to_lines;

const ANSWER: (&str, &str) = ("2105961943", "22289513667691");

fn main() {
    let _filename_test = "data/day09/test_input_01.txt";
    let _filename_test2 = "data/day09/test_input_02.txt";

    let filename_part1 = "data/day09/part1_input.txt";
    let filename_part2 = "data/day09/part2_input.txt";

    let start1 = Instant::now();
    let answer1 = part1(filename_part1);
    let duration1 = start1.elapsed();

    let start2 = Instant::now();
    let answer2 = part2(filename_part2);
    let duration2 = start2.elapsed();

    println!("Advent of Code, Day 09");
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
    let mut history:Vec<i32> = Vec::new();
    for l in &lines
    {
        let mut row_list: Vec<Vec<i32>> = Vec::new();
        let nums: Vec<i32> = advent_2023::parse_number_list_whitespace(l);

        row_list.push(nums);
        let mut all_zeros = false;
        while !all_zeros {
            let current = row_list.last().unwrap();
            let diffs = get_diffs(current);

          all_zeros = is_all_zero(&diffs);
            row_list.push(diffs);
        }

        let mut last_right = 0;
        for i in (0..row_list.len()).rev() {
            let mut row = &mut row_list[i];
            if is_all_zero(&row) {
                last_right = 0;
            } else {
                let end = row.last().unwrap();
                last_right = end + last_right;
            }

            row.push(last_right);
        }
        history.push(*row_list[0].last().unwrap());
    }
    let answer:i32 = history.iter().sum();
    return answer.to_string();
}

fn is_all_zero(series: &Vec<i32>) -> bool {
    for n in series {
        if *n != 0 {
            return false;
        }
    }
    return true;
}

fn get_diffs(series: &Vec<i32>) -> Vec<i32>
{
    let mut diffs: Vec<i32> = Vec::new();
    if series.len() < 2 {
        return series.clone();
    }
    let mut left = series[0];
    for i in 1..series.len() {
        let d = series[i] - left;
        diffs.push(d);
        left = series[i];
    }


    return diffs;
}

fn part2(input_file: &str) -> String {
    let lines = file_to_lines(input_file);
    let mut history:Vec<i32> = Vec::new();
    for l in &lines
 //   let l = &lines[0];
    {
        let mut row_list: Vec<Vec<i32>> = Vec::new();
        let nums: Vec<i32> = advent_2023::parse_number_list_whitespace(l);

        row_list.push(nums);
        let mut all_zeros = false;
        while !all_zeros {
            let current = row_list.last().unwrap();
            let diffs = get_diffs(current);

            all_zeros = is_all_zero(&diffs);
            row_list.push(diffs);
        }

        let mut last_left = 0;
        for i in (0..row_list.len()).rev() {
            let mut row = &mut row_list[i];
            if is_all_zero(&row) {
                last_left = 0;
            } else {
                let end = row.first().unwrap();
                last_left = end - last_left;
            }

            row.insert(0, last_left);
        }
        history.push(*row_list[0].first().unwrap());
    }
    let answer:i32 = history.iter().sum();
    println!("history: {:?} \t sum: {}", history, answer);


    return answer.to_string();
}

