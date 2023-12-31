/*
    Advent of Code 2023: Day 03
        part1 answer:   527369
        part2 answer:   73074886

 */

use std::collections::{HashMap, HashSet};
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::time::Instant;

const ANSWER: (&str, &str) = ("527369", "73074886");

fn main() {
    let _filename_test = "data/day03/test_input_01.txt";
    let _filename_test2 = "data/day03/test_input_02.txt";
    let _filename_test3 = "data/day03/test_input_03.txt";
    let filename_part1 = "data/day03/part1_input.txt";
    let filename_part2 = "data/day03/part2_input.txt";

    let start1 = Instant::now();
    let answer1 = part1(filename_part1);
    let duration1 = start1.elapsed();

    let start2 = Instant::now();
    let answer2 = part2(filename_part2);
    let duration2 = start2.elapsed();

    println!("Advent of Code, Day 03");
    println!("    ---------------------------------------------");
    println!("\t Part 1: {} \t\t time: {:?}", answer1, duration1);
    if ANSWER.0 != answer1 {
        println!("\t\t ERROR: Answer is WRONG. Got: {}, Expected {}", answer1, ANSWER.0);
    }
    println!("\t Part 2: {} \t\t time: {:?}", answer2, duration2);
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
    let lines = file_to_lines(input_file);

    let (grid, rows, cols) = parse_grid(lines);
    let mut found_nums = Vec::new();
    for r in 0..rows {
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
                        let num: i32 = sb.parse().unwrap();
                        found_nums.push((num, num_start, sb.len() as i32));
                        break;
                    }
                }
            } else {
                c += 1;
            }
        }
    }

    let mut good_numbers: Vec<i32> = Vec::new();
    for i in 0..found_nums.len() {
        let (num, num_start, length) = found_nums[i];
        let (r, c) = num_start;
        'num_check: for cl in c..(c + length) {
            let neighs = get_neighbor_points(r, cl, true);
            for neigh in neighs {
                let o_ch = grid.get(&neigh);
                match o_ch {
                    None => {}
                    Some(ch) => {
                        if ch.is_ascii_digit() || *ch == '.' {
                            continue;
                        } else {
                            good_numbers.push(num);
                            break 'num_check;
                        }
                    }
                }
            }
        }
    }
    let sum: i32 = good_numbers.iter().sum();
    return sum.to_string();
}


fn part2(input_file: &str) -> String {
    let lines = file_to_lines(input_file);

    let (grid, rows, cols) = parse_grid(lines);

    let mut number_by_location: HashMap<(i32, i32), i32> = HashMap::new();
    let mut found_nums = Vec::new();
    for r in 0..rows {
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
                        let num: i32 = sb.parse().unwrap();
                        found_nums.push((num, num_start, sb.len() as i32));
                        number_by_location.insert(num_start, num);
                        break;
                    }
                }
            } else {
                c += 1;
            }
        }
    }
    let mut gear_locations: Vec<(i32, i32)> = Vec::new();
    for r in 0..rows {
        for c in 0..cols {
            let o_ch = grid.get(&(r, c));
            if let Some(ch) = o_ch {
                if *ch == '*' {
                    gear_locations.push((r, c));
                }
            }
        }
    }

    let gear_locations = gear_locations;
    let mut gear_pair_list: Vec<HashSet<(i32, i32)>> = Vec::new();
    for g_loc in gear_locations {
        let mut gear_pair: HashSet<(i32, i32)> = HashSet::with_capacity(2);
        let (g_r, g_c) = g_loc;
        let neighbors = get_neighbor_points(g_r, g_c, true);
        'neigh_search: for neigh in neighbors {
            let o_ch = grid.get(&neigh);
            if let Some(ch) = o_ch {
                if ch.is_ascii_digit() {
                    let n = get_number_from_loc(neigh, &found_nums);
                    for i in n {
                        gear_pair.insert(i);
                    }
                    if gear_pair.len() >= 2 {
                        gear_pair_list.push(gear_pair);
                        break 'neigh_search;
                    }
                }
            }
        }
    }
    let mut total_ratio: i32 = 0;
    for g_set in gear_pair_list {
        let mut rat: i32 = 1;
        for loc in g_set {
            let val = number_by_location.get(&loc).unwrap();
            rat = rat * val;
        }
        total_ratio += rat;
    }
    return total_ratio.to_string();
}

fn get_number_from_loc(hit_loc: (i32, i32), found_numbers: &Vec<(i32, (i32, i32), i32)>) -> HashSet<(i32, i32)> {
    let (h_row, h_col) = hit_loc;
    let mut adj_vals: HashSet<(i32, i32)> = HashSet::new();
    for (_, start_loc, length) in found_numbers {
        let (s_row, s_col) = start_loc;
        if h_row != *s_row {
            continue;
        } else {
            if h_col >= *s_col && (*s_col + *length) > h_col {
                adj_vals.insert(*start_loc);
                if adj_vals.len() == 2 {
                    return adj_vals;
                }
            }
        }
    }
    return adj_vals;
}

fn parse_grid(lines: Vec<String>) -> (HashMap<(i32, i32), char>, i32, i32) {
    let rows = lines.len() as i32 + 1;
    let cols = lines[0].len() as i32 + 1;
    let mut grid: HashMap<(i32, i32), char> = HashMap::new();
    for r in 0..=rows {
        for c in 0..=cols {
            grid.insert((r, c), '.');
        }
    }
    let (rows, cols) = (rows + 1, cols + 1);
    let mut r = 1;
    for l in lines {
        let mut c = 1;
        for ch in l.chars() {
            grid.insert((r, c), ch);
            c += 1;
        }
        r += 1;
    }
    (grid, rows, cols)
}






