// #![allow(unused_variables)]
// #![allow(unused_imports)]
// #![allow(unused_mut)]
// #![allow(dead_code)]
// #![allow(unused_assignments)]


/*
    Advent of Code 2023: Day 03
        part1 answer:   527369
        part2 answer:

 */

use std::collections::HashMap;
use std::fs::File;
use std::io::{BufRead, BufReader};

const ANSWER: (&str, &str) = ("527369", "66363");

fn main() {
    let _filename_test = "data/day03/test_input_01.txt";
    let _filename_test2 = "data/day03/test_input_02.txt";
    let filename_part1 = "data/day03/part1_input.txt";
    let filename_part2 = "data/day03/part2_input.txt";


    println!("Advent of Code, Day ");
    println!("    ---------------------------------------------");


    let answer1: String = part1(filename_part1);

    println!("\t Part 1: {}", answer1);
    if ANSWER.0 != answer1 {
        println!("\t\t ERROR: Answer is WRONG. Got: {}, Expected {}", answer1, ANSWER.0);
    }

    let answer2: String = part2(filename_part2);
    println!("\t Part 2: {}", answer2);
    if ANSWER.1 != answer2 {
        println!("\t\t ERROR: Answer is WRONG. Got: {}, Expected {}", answer2, ANSWER.1);
    }

    println!("    ---------------------------------------------");
}


fn file_to_lines(input_file: &str) -> Vec<String> {
    let file = File::open(input_file).expect(&*format!("error opening file {}", input_file));
    let bfile = BufReader::new(file);
    let lines: Vec<String> = bfile.lines().filter_map(|x| x.ok()).collect();
    return lines;
}

fn get_neighbor_points(r: i32, c: i32, diag: bool) -> Vec<(i32, i32)> {
    static CARD_DELTA: [(i32, i32); 4] = [(-1, 0), (1, 0), (0, -1), (0, 1)];
    static DIAG_DELTA: [(i32, i32); 4] = [(-1, -1), (-1, 1), (1, -1), (1, 1)];
    let mut neighs: Vec<(i32, i32)> = Vec::new();
    for i in 0..CARD_DELTA.len() {
        let (dr, dc) = CARD_DELTA[i];
        neighs.push((r + dr, c + dc));
    }
    if diag {
        for i in 0..DIAG_DELTA.len() {
            let (dr, dc) = DIAG_DELTA[i];
            neighs.push((r + dr, c + dc));
        }
    }
    return neighs;
}


fn part1(input_file: &str) -> String {
    println!();
    let lines = file_to_lines(input_file);


    let rows = lines.len() as i32 + 1;
    let cols = lines[0].len() as i32 + 1;
    println!("rows: {rows}, cols: {cols}");

    let mut grid: HashMap<(i32, i32), char> = HashMap::new();

    for r in 0..=rows {
        for c in 0..=cols {
            grid.insert((r as i32, c as i32), '.');
        }
    }
    let (rows, cols) = (rows + 1, cols + 1);

    let mut r = 1;
    let mut c;
    let mut good_points: HashMap<(i32, i32), bool> = HashMap::new();


    for l in lines {
        c = 1;
        for ch in l.chars() {
            grid.insert((r, c), ch);
            if ch.is_digit(0) || ch == '.' {
                good_points.insert((r, c), false);
            } else {
                good_points.insert((r, c), true);
            }
            c += 1;
        }
        r += 1;
    }

    //  print_grid(grid.clone(), rows, cols);
    let mut found_nums = Vec::new();

    for  r in 0..rows {
        let mut c = 0;
        while c < cols {
            let ch = grid.get(&(r, c)).unwrap();
            if ch.is_ascii_digit() {
                let num_start = (r, c);
                let mut sb = String::new();
                sb.push(*ch);
                while c < cols {
                    c += 1;
                    let ch = grid.get(&(r, c)).unwrap();

                    if ch.is_ascii_digit() {
                        sb.push(*ch)
                    } else {
                        // end of number
                        let num: i32 = sb.parse().unwrap();
                        //               println!("Number Found: {num} at {:?} length {}",num_start, sb.len());
                        found_nums.push((num, num_start, sb.len() as i32));
                        break;
                    }
                }
            } else {
                c += 1;
            }
        }
    }
    println!("found {} numbers", found_nums.len());

    let mut is_good;
    let mut good_numbers: Vec<i32> = Vec::new();

    for i in 0..found_nums.len() {
        let (tnum, num_start, length) = found_nums[i];

        //for (num, num_start@(r,c), length) in &found_nums {
   //     println!("Checking {tnum} @ {:?} {}", num_start, length);
        is_good = false;
        let (r, c) = num_start;
        'num_check: for cl in c..(c + length) {
    //        println!("getting neighbors of point: {:?}  (value: {:?}", (r, cl),grid.get(&(r, cl)));
            let neighs = get_neighbor_points(r, cl, true);
            for neigh in neighs {
                let o_ch = grid.get(&neigh);
                match o_ch {
                    None => {}
                    Some(ch) => {
                        if ch.is_ascii_digit() || *ch == '.' {
                            //                    println!("not symbol: {ch} @ {:?}", neigh);
                            //neighbor isn't symbol
                            continue;
                        } else {
                            //neighbor is symbol
              //              println!("\t is good because {:?} is {}", neigh, ch);
                            good_numbers.push(tnum);
                            is_good = true;

                            break 'num_check;
                        }
                    }
                }
            }
        }
    }

    let sum:i32 = good_numbers.iter().sum();
    println!("flush");
    return sum.to_string();
}


fn print_grid(grid: HashMap<(i32, i32), char>, n_rows: i32, n_cols: i32) {
    for r in 0..n_rows {
        for c in 0..n_cols {
            let ch = grid.get(&(r, c));
            match ch {
                None => {
                    println!("error retrieving grid point at ({r},{c})");
                    return;
                }
                Some(ch) => { print!("{ch}"); }
            }
        }
        println!();
    }
    println!();
}


fn part2(input_file: &str) -> String {
    let lines = file_to_lines(input_file);

    return String::new();
}

