/*
    Advent of Code 2023: Day 06
        part1 answer:   1108800
        part2 answer:   36919753
 */



use std::fs::File;
use std::io::{BufRead, BufReader};
use std::str::FromStr;
use std::time::Instant;

const ANSWER: (&str, &str) = ("1108800", "36919753");

fn main() {
    let _filename_test = "data/day06/test_input_01.txt";
    let _filename_test2 = "data/day06/test_input_02.txt";

    let filename_part1 = "data/day06/part1_input.txt";
    let filename_part2 = "data/day06/part2_input.txt";

    let start1 = Instant::now();
    let answer1 = part1(filename_part1);
    let duration1 = start1.elapsed();

    let start2 = Instant::now();
    let answer2 = part2(filename_part2);
    let duration2 = start2.elapsed();

    println!("Advent of Code, Day 06");
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


fn part1(input_file: &str) -> String {
    let lines = file_to_lines(input_file);
    let (time_v, dist_v): (Vec<i64>, Vec<i64>) = parse_input(lines);
    let mut product: i64 = 1;
    for i in 0..time_v.len() {
        let (t, d) = (time_v[i], dist_v[i]);
        let n = find_time_to_win(t as f64, d as f64);
        let r = (n.1 - n.0) + 1;
        product = product * r;
        //      println!(" {i}:\t {:?} \t r: {r}", n);
    }
    return product.to_string();
}

fn part2(input_file: &str) -> String {
    let lines = file_to_lines(input_file);
    let (time_v, dist_v): (Vec<i64>, Vec<i64>) = parse_input2(lines);
    let mut product: i64 = 1;
    for i in 0..time_v.len() {
        let (t, d) = (time_v[i], dist_v[i]);
        let n = find_time_to_win(t as f64, d as f64);
        let r = (n.1 - n.0) + 1;
        product = product * r;
    }
    return product.to_string();
}

fn find_time_to_win(race_time: f64, best_distance: f64) -> (i64, i64) {
    let determ = ((race_time * race_time) - (4.0 * best_distance)).sqrt();
    let mut a = (-race_time + determ) / -2.0;
    let mut b = (-race_time - determ) / -2.0;
    if a.fract() == 0.0 {
        a += 1.0;
    }
    if b.fract() == 0.0 {
        b -= 1.0
    }
    return (a.ceil() as i64, b.floor() as i64);
}

fn parse_input(lines: Vec<String>) -> (Vec<i64>, Vec<i64>) {
    let (_, time_string) = lines[0].trim().split_once(":").unwrap();
    let (_, dist_string) = lines[1].trim().split_once(":").unwrap();
    let times = parse_number_list_whitespace(time_string.trim());
    let distances = parse_number_list_whitespace(dist_string.trim());
    return (times, distances);
}

fn parse_input2(lines: Vec<String>) -> (Vec<i64>, Vec<i64>) {
    let (_, time_string) = lines[0].trim().split_once(":").unwrap();
    let (_, dist_string) = lines[1].trim().split_once(":").unwrap();

    let times: Vec<i64> = parse_number_list_whitespace(time_string.trim());
    let mut sb = String::new();
    for n in times {
        sb.push_str(n.to_string().as_str());
    }
    let f_time: i64 = sb.parse().unwrap();

    let distances: Vec<i64> = parse_number_list_whitespace(dist_string.trim());
    let mut sb = String::new();
    for n in distances {
        sb.push_str(n.to_string().as_str());
    }
    let d_time: i64 = sb.parse().unwrap();

    return (vec!(f_time), vec!(d_time));
}
