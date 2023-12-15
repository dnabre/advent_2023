#![allow(unused_variables)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(dead_code)]
#![allow(unused_assignments)]
#![allow(unreachable_code)]

/*
    Advent of Code 2023: Day 13
        part1 answer:   33122
        part2 answer:   32312

 */


use std::fmt::Display;
use std::time::Instant;
use advent_2023::file_to_lines;


const ANSWER: (&str, &str) = ("33122", "32312");

fn main() {
    let _filename_test1 = "data/day13/test_input_01.txt";
    let _filename_test2 = "data/day13/test_input_02.txt";

    let filename_part1 = "data/day13/part1_input.txt";
    let filename_part2 = "data/day13/part2_input.txt";

    let start1 = Instant::now();
    let answer1 = part1(filename_part1);
    let duration1 = start1.elapsed();

    let start2 = Instant::now();
    let answer2 = part2(filename_part2);
    let duration2 = start2.elapsed();

    // println!("Advent of Code, Day 13");
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

    let grouped_patterns = advent_2023::group_newline_separated_lines(&lines);

    let mut sum = 0;
    for i in 0..grouped_patterns.len() {
        let (rows, cols) = bit_pack_both_orders(grouped_patterns[i].as_str());

        let h_idx = get_reflection_index(&rows);
        let v_idx = get_reflection_index(&cols);

        if let Some(q) = h_idx {
            sum += 100 * q;
        } else if let Some(q) = v_idx {
            sum += q;
        } else {
            panic!("reflection index not found!")
        }
    }

    return sum.to_string();
}


fn part2(input_file: &str) -> String {
    let lines = file_to_lines(input_file);

    let grouped_patterns = advent_2023::group_newline_separated_lines(&lines);

    let mut sum = 0;

    for i in 0..grouped_patterns.len() {
        let (rows, cols) = bit_pack_both_orders(grouped_patterns[i].as_str());
        //


        let h_idx = get_smudged_reflection_index(&rows);
        let v_idx = get_smudged_reflection_index(&cols);

        // if h_idx != h_idx2 {
        //     println!("h_idx2 is wrong: {:?} should be {:?}", h_idx2, h_idx)
        // }
        // if v_idx != v_idx2 {
        //     println!("v_idx2 is wrong: {:?} should be {:?}", v_idx2, v_idx)
        // }


        if let Some(q) = h_idx {
            sum += 100 * q;
        } else if let Some(q) = v_idx {
            sum += q;
        } else {
            panic!("reflection index not found!")
        }
    }


    return sum.to_string();
}


fn bit_pack_both_orders(p: &str) -> (Vec<u32>, Vec<u32>) {
    let mut cols: Vec<u32> = vec![0; p.split_once("\n").unwrap().0.len()];
    let rows: Vec<u32> = p.lines().enumerate().map(|(j, s)| {
        let mut n: u32 = 0;
        s.chars().enumerate().for_each(|(i, ch)| match ch {
            '#' => {
                n |= 1 << i;
                cols[i] |= 1 << j;
            }
            '.' => {}
            x_ch => { panic!("unexpected character: {x_ch}") }
        });
        n
    }).collect();
    return (rows, cols);
}

fn get_reflection_index(seq: &[u32]) -> Option<usize> {
    // removed .collect::<Vec<_>>()
    for idx in 0..seq.len() - 1 {
        let paired_split_on_index = (0..=idx).into_iter().rev().zip(idx + 1..seq.len());
        let pair_values = paired_split_on_index.into_iter().map(|(a, b)| (seq[a], seq[b]));
        let all_match = pair_values.into_iter().all(|(x, y)| x == y);
        if all_match {
            return Some(idx + 1);
        }
    }
    return None;
}

fn get_smudged_reflection_index(seq: &[u32]) -> Option<usize> {
    // removed .collect::<Vec<_>>()
    for idx in 0..seq.len() - 1 {
        let paired_split_on_index = (0..=idx).into_iter().rev().zip(idx + 1..seq.len());


        let pair_values = paired_split_on_index.into_iter().map(|(a, b)| (seq[a], seq[b]));


        let xored_pairs = pair_values.into_iter().map(|(x, y)| (x ^ y).count_ones()).collect::<Vec<_>>();

        println!("{idx:3} xored_pairs: {:?}", xored_pairs);

        let sum_of_xored:u32= xored_pairs.iter().sum();
         if sum_of_xored == 1 {
             println!("\t{idx:3} sum of xored pairs is {}", sum_of_xored);

             return Some(idx+1);
        }


    }
    return None;
}

