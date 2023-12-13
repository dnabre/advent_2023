/*
    Advent of Code 2023: Day 12
        part1 answer:   7922
        part2 answer:   18093821750095

 */

use std::time::Instant;
use advent_2023::file_to_lines;


const ANSWER: (&str, &str) = ("7922", "18093821750095");

fn main() {
    let _filename_test = "data/day12/test_input_01.txt";
    let _filename_test2 = "data/day12/test_input_02.txt";


    let filename_part1 = "data/day12/part1_input.txt";
    let filename_part2 = "data/day12/part2_input.txt";

    let start1 = Instant::now();
    let answer1 = part1(filename_part1);
    let duration1 = start1.elapsed();

    let start2 = Instant::now();
    let answer2 = part2(filename_part2);
    let duration2 = start2.elapsed();

    println!("Advent of Code, Day 12");
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
    let mut sum = 0;

    for l in &lines {
        let (pattern, s_counts) = l.split_once(" ").unwrap();
        let counts: Vec<usize> = advent_2023::parse_number_list_comma(s_counts);
        let t = count_for_line(pattern, &counts);
        sum += t;

    }
    return sum.to_string();
}

fn part2(input_file: &str) -> String {
    let lines = file_to_lines(input_file);
    let mut sum = 0;

    for l in &lines {
        let (mut pattern, s_counts) = l.split_once(" ").unwrap();
        let mut counts: Vec<usize> = advent_2023::parse_number_list_comma(s_counts);

        let mut sb = String::new();
        let mut new_count: Vec<usize> = Vec::with_capacity(counts.len() * 5);
        for _ in 0..4 {
            sb.push_str(pattern);
            sb.push('?');
            new_count.extend(counts.iter());
        }
        sb.push_str(pattern);
        new_count.extend(counts.iter());
        (pattern, counts)  = (sb.as_str(), new_count);

        let t = count_for_line(pattern, &counts);
        sum += t;
    }
    return sum.to_string();
}


fn count_for_line(line: &str, counts: &[usize]) -> usize {
    let pattern: Vec<char> = line.chars().collect();
    let n = pattern.len();
    let m = counts.len();
    let mut dp = &mut vec![vec![0; n + 1]; m + 1];
    let mut next_dp = &mut vec![vec![0; n + 1]; m + 1];

    (dp[m][0], dp[m-1][counts[m-1]]) = (1,1);

    for i in (0..n).rev() {
        for c in 0..=m {
            let max_count;
            if c == m {
                max_count = 0;
            } else {
                max_count = counts[c];
            }
            let ch = pattern[i];
            for count in 0..=max_count {
                next_dp[c][count] = 0;
                if ch == '#' || ch == '?' {
                    next_dp[c][count] += dp[c][count + 1];
                }
            }
              if ch == '.' || ch == '?' {
                next_dp[c][0] += dp[c][0];
                if c < m {
                    next_dp[c][max_count] += dp[c + 1][0];
                }
            }
        }
        (dp, next_dp) = (next_dp, dp);
    }
    return dp[0][0];
}