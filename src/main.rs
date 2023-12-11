#![allow(unused_variables)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(dead_code)]
#![allow(unused_assignments)]
#![allow(unreachable_code)]

/*
    Advent of Code 2023: Day 11
        part1 answer:   2105961943
        part2 answer:   1019

 */


use std::time::Instant;
use advent_2023::file_to_lines;


const ANSWER: (&str, &str) = ("2105961943", "1019");

fn main() {
    let _filename_test = "data/day11/test_input_01.txt";
    let _filename_test2 = "data/day11/test_input_02.txt";

    let filename_part1 = "data/day11/part1_input.txt";
    let filename_part2 = "data/day11/part2_input.txt";

    let start1 = Instant::now();
    let answer1 = part1(_filename_test);
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
/*
    inital: [(3, 0), (7, 1), (0, 2), (6, 4), (1, 5), (9, 6), (7, 8), (0, 9), (4, 9)]
    expand: [(4, 0), (9, 1), (0, 2), (8, 5), (1, 6), (12, 7), (9, 10), (0, 11), (5, 11)]

 */


fn part1(input_file: &str) -> String {
    let lines = file_to_lines(input_file);

    let mut galaxy_list: Vec<(usize, usize)> = Vec::new();

    let mut x: usize = 0;
    let mut y: usize = 0;


    for y in 0..lines.len() {
        let row: Vec<char> = lines[y].chars().collect();
        if is_all_foo(&row, '.') {
            galaxy_list = galaxy_list.iter().map(|(x,y)| (*x,*y+1)).collect();
        } else {
            for x in 0..row.len() {
                let ch = row[x];
                if ch == '#' {
                    galaxy_list.push((x, y));
                }
            }
        }
    }
    println!("galaxies found {}", galaxy_list.len());
    println!("galaxies: {:?}", galaxy_list);
    let mut empty_columns:Vec<usize> = Vec::new();
    for x in 0..lines.len() {
        let mut is_empty = true;
        for g@(g_x,g_y) in &galaxy_list {
            if *g_x == x {
                is_empty = false;
                break;
            }
        }
        if is_empty {
            empty_columns.push(x);

        }
    }

    for x in &empty_columns {
        for i in 0..galaxy_list.len() {
            let g@(g_x,g_y) = galaxy_list[i];
            if *x < g_x {
                galaxy_list[i] = (g_x+1, g_y);
            }
        }
    }
    println!("empty columns: {:?}", empty_columns);

    println!("galaxies: {:?}", galaxy_list);


    let answer: usize = 0;
    return answer.to_string();
}

fn part2(input_file: &str) -> String {
    let lines = file_to_lines(input_file);
    let answer: usize = 0;
    return answer.to_string();
}

fn is_all_foo<T: std::cmp::PartialEq>(series: &Vec<T>, element: T) -> bool {
    for n in series {
        if *n != element {
            return false;
        }
    }
    return true;
}