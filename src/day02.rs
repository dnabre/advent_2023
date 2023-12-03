/*
    Advent of Code 2023: Day 02
        part1 answer:   2369
        part2 answer: 66363

 */

use std::fs::File;
use std::io::{BufRead, BufReader};

const ANSWER: (&str, &str) = ("2369", "66363");

fn main() {
    let _filename_test = "data/day02/test_input_01.txt";
    let _filename_test2 = "data/day02/test_input_02.txt";
    let filename_part1 = "data/day02/part1_input.txt";
    let filename_part2 = "data/day02/part2_input.txt";


    println!("Advent of Code, Day 02");
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


fn part1(input_file: &str) -> String {
    let lines = file_to_lines(input_file);

    let mut game_num = 0;

    // only 12 red cubes, 13 green cubes, and 14 blue cubes
    let limit = (12, 13, 14);


    let mut sum = 0;
    for l in &lines {
        let (mut m_r, mut m_g, mut m_b) = (0, 0, 0);
        game_num += 1;
        let (_, rr) = l.split_once(":").unwrap();

        let parts: Vec<&str> = rr.split(";").map(|s| s.trim()).collect();
        let mut possible = true;
        for p in parts {
            let (r, g, b) = parse_game(p);
            (m_r, m_g, m_b) = (m_r.max(r), m_g.max(g), m_b.max(b));
            possible = check_limit(limit, (m_r, m_g, m_b));
        }
        if possible {
            sum += game_num;
        }
    }

    return sum.to_string();
}

fn part2(input_file: &str) -> String {
    let lines = file_to_lines(input_file);

    let mut sum = 0;
    for l in &lines {
        let (mut m_r, mut m_g, mut m_b) = (0, 0, 0);
        let (_, rr) = l.split_once(":").unwrap();
        let parts: Vec<&str> = rr.split(";").map(|s| s.trim()).collect();

        for p in parts {
            let  (r, g, b) = parse_game(p);
            (m_r, m_g, m_b) = (m_r.max(r), m_g.max(g), m_b.max(b));
        }
        let power = cube_power((m_r, m_g, m_b));
        sum += power;
    }
    return sum.to_string();
}

fn check_limit(limit: (i32, i32, i32), check: (i32, i32, i32)) -> bool {
    return (limit.0 >= check.0) && (limit.1 >= check.1) && (limit.2 >= check.2);
}

fn cube_power((r, g, b): (i32, i32, i32)) -> i32 {
    return r * g * b;
}

fn parse_game(game: &str) -> (i32, i32, i32) {
    let pp: Vec<&str> = game.split(",").map(|s| s.trim()).collect();
    if pp.len() == 1 {
        return parse_hunk(pp[0]);
    } else {
        let (mut r, mut g, mut b) = (0, 0, 0);
        for p in pp {
            let (d_r, d_g, d_b) = parse_hunk(p);
            (r, g, b) = (r + d_r, g + d_g, b + d_b);
        }
        return (r, g, b);
    }
}

fn parse_hunk(solo: &str) -> (i32, i32, i32) {
    let  (n, c) = solo.split_once(" ").unwrap();
    let n = n.parse::<i32>().unwrap();
    return match c {
        "red" => (n, 0, 0),
        "green" => (0, n, 0),
        "blue" => (0, 0, n),
        x => {
            panic!("unmatched color: {x}")
        }
    };
}
