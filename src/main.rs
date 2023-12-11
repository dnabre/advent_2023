#![allow(unused_variables)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(dead_code)]
#![allow(unused_assignments)]
#![allow(unreachable_code)]

/*
    Advent of Code 2023: Day 11
        part1 answer:   9609130
        part2 answer:   702152204842

 */


use std::collections::HashSet;
use std::time::Instant;
use advent_2023::file_to_lines;


const ANSWER: (&str, &str) = ("9609130", "702152204842");

fn main() {
    let _filename_test = "data/day11/test_input_01.txt";
    let _filename_test2 = "data/day11/test_input_02.txt";

    let filename_part1 = "data/day11/part1_input.txt";
    let filename_part2 = "data/day11/part2_input.txt";

    let start1 = Instant::now();
    let answer1 = part1(filename_part1);
    let duration1 = start1.elapsed();

    let start2 = Instant::now();
    let answer2 = part2(filename_part2);
    let duration2 = start2.elapsed();

    println!("Advent of Code, Day 11");
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

    let mut galaxy_list: Vec<(usize, usize)> = Vec::new();

    let grid: Vec<Vec<char>> = lines.iter().map(|l| l.chars().collect()).collect();
    let mut empty_rows:HashSet<usize> = HashSet::new();
    let mut empty_columns:HashSet<usize> = HashSet::new();

    for i in 0..grid.len() {
        let row = &grid[i];
        if row.iter().any(|&c| c != '.') {
            continue;
        }
        empty_rows.insert(i);
    }
    for i in 0..grid[0].len() {
        if grid.iter().any(|r| r[i] != '.') {
            continue;
        }
        empty_columns.insert(i);
    }

    let mut y_offset:usize = 0;
    for y in 0..lines.len() {
        if empty_rows.contains(&y) {
            y_offset +=1 ;
        }
        let mut x_offset = 0;
        let row: Vec<char> = lines[y].chars().collect();
            for x in 0..row.len() {
                if empty_columns.contains(&x) {
                    x_offset += 1;
                }
                let ch = row[x];
                if ch == '#' {
                    galaxy_list.push((x + x_offset, y + y_offset));
                }
            }
    }


    let mut pair_list:Vec<((usize,usize),(usize,usize))> = Vec::new();
    for i in 0..galaxy_list.len() {
        for j in i+1..galaxy_list.len() {
            pair_list.push((galaxy_list[i], galaxy_list[j]));
        }
    }




    let mut answer: usize = 0;
    for (p1,p2) in pair_list {
        let d = advent_2023::get_distance_m1(p1, p2);
        answer += d;
    }


    return answer.to_string();
}

const GALAXY_EXPAND_FACTOR:usize = 1000000 -1;

fn part2(input_file: &str) -> String {
    let lines = file_to_lines(input_file);

    let mut galaxy_list: Vec<(usize, usize)> = Vec::new();

    let grid: Vec<Vec<char>> = lines.iter().map(|l| l.chars().collect()).collect();
    let mut empty_rows:HashSet<usize> = HashSet::new();
    let mut empty_columns:HashSet<usize> = HashSet::new();

    for i in 0..grid.len() {
        let row = &grid[i];
        if row.iter().any(|&c| c != '.') {
            continue;
        }
        empty_rows.insert(i);
    }
    for i in 0..grid[0].len() {
        if grid.iter().any(|r| r[i] != '.') {
            continue;
        }
        empty_columns.insert(i);
    }

    let mut y_offset:usize = 0;
    for y in 0..lines.len() {
        if empty_rows.contains(&y) {
            y_offset += GALAXY_EXPAND_FACTOR ;
        }
        let mut x_offset = 0;
        let row: Vec<char> = lines[y].chars().collect();
        for x in 0..row.len() {
            if empty_columns.contains(&x) {
                x_offset += GALAXY_EXPAND_FACTOR;
            }
            let ch = row[x];
            if ch == '#' {
                galaxy_list.push((x + x_offset, y + y_offset));
            }
        }
    }


    let mut pair_list:Vec<((usize,usize),(usize,usize))> = Vec::new();
    for i in 0..galaxy_list.len() {
        for j in i+1..galaxy_list.len() {
            pair_list.push((galaxy_list[i], galaxy_list[j]));
        }
    }



    let mut answer: usize = 0;
    for (p1,p2) in pair_list {
        let d = advent_2023::get_distance_m1(p1, p2);
        answer += d;
    }


    return answer.to_string();
}