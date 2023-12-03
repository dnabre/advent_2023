#![allow(unused_variables)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(dead_code)]
#![allow(unused_assignments)]


/*
    Advent of Code 2023: Day 03
        part1 answer:
        part2 answer:

 */

use std::collections::HashMap;
use std::fs::File;
use std::io::{BufRead, BufReader};

const ANSWER: (&str, &str) = ("2369", "66363");

fn main() {
    let _filename_test = "data/day03/test_input_01.txt";
    let _filename_test2 = "data/day03/test_input_02.txt";
    let filename_part1 = "data/day03/part1_input.txt";
    let filename_part2 = "data/day03/part2_input.txt";


    println!("Advent of Code, Day ");
    println!("    ---------------------------------------------");


    let answer1: String = part1(_filename_test);

    println!("\t Part 1: {}", answer1);
    if ANSWER.0 != answer1 {
        println!("\t\t ERROR: Answer is WRONG. Got: {}, Expected {}", answer1, ANSWER.0);
    }

    // let answer2: String = part2(filename_part2);
    // println!("\t Part 2: {}", answer2);
    // if ANSWER.1 != answer2 {
    //     println!("\t\t ERROR: Answer is WRONG. Got: {}, Expected {}", answer2, ANSWER.1);
    // }

    println!("    ---------------------------------------------");
}


fn file_to_lines(input_file: &str) -> Vec<String> {
    let file = File::open(input_file).expect(&*format!("error opening file {}", input_file));
    let bfile = BufReader::new(file);
    let lines: Vec<String> = bfile.lines().filter_map(|x| x.ok()).collect();
    return lines;
}


fn part1(input_file: &str) -> String {
    println!();
    let lines = file_to_lines(input_file);


    let rows= lines.len() as i32 + 1;
    let cols = lines[0].len() as i32 +1;
    println!("rows: {rows}, cols: {cols}");

    let mut grid:HashMap<(i32,i32), char> = HashMap::new();

    for r in 0..=rows {
        for c in 0..=cols {
            grid.insert((r as i32,c as i32), '.');
        }
    }
    let (rows, cols) = (rows+1, cols+1);

    let mut r =1;
    let mut c = 1;
    let mut good_points:HashMap<(i32,i32),bool> = HashMap::new();

    for l in lines {
        c =1;
        for ch in l.chars() {
            grid.insert((r,c), ch);
            if ch.is_digit(0) || ch == '.' {
                good_points.insert((r,c), false);
            } else {
                good_points.insert((r,c), true);
            }
            c += 1;
        }
        r +=1;
    }


        // expand good points to diag neighbors
    let mut in_number = false;
    let mut sb = String::new();
    let mut all_nums:Vec<(i32, (i32,i32), usize)> = Vec::new();

    for r in 0..rows {
        let mut in_number = false;
        let mut sb = String::new();
        for c in 0..cols {
            let ch = grid.get(&(r,c)).unwrap();
            if ch.is_digit(0) || *ch == '.' {
                continue;
            } else {
                let neighs: Vec<(i32, i32)> = get_neighbor_points(r, c, true);
                for p in neighs {
                    good_points.insert(p, true);
                }
            }
        }
    }





            println!("flush");
    return String::new();
}

fn get_neighbor_points(r: i32, c: i32, diag: bool) -> Vec<(i32, i32)> {
    static CARD_DELTA: [(i32, i32); 4] =[(-1, 0), (1, 0), (0, -1), (0, 1)];

    static DIAG_DELTA: [(i32, i32); 4] = [(-1, -1), (-1, 1), (1, -1), (1, 1)];

    let mut neighs:Vec<(i32,i32)> = Vec::new();
    for i in 0..CARD_DELTA.len() {

        let (dr,dc) = CARD_DELTA[i];
        neighs.push((r+dr, c+dc));
    }
    if diag {
        for i in 0..DIAG_DELTA.len() {
            let (dr,dc) = DIAG_DELTA[i];
            neighs.push((r+dr, c+dc));
        }

    }
    return neighs;
}



fn print_grid(grid: HashMap<(i32, i32), char>, n_rows:i32, n_cols:i32) {
    for r in 0..n_rows {
        for c in 0..n_cols {
          let ch =   grid.get(&(r,c));
            match ch {
                None => {
                    println!("error retrieving grid point at ({r},{c})");
                    return;
                }
                Some(ch) => {print!("{ch}");}
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

