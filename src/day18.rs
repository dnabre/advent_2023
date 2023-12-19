
/*
    Advent of Code 2023: Day 18
        part1 answer:   62365
        part2 answer:   159485361249806


 */



use std::time::Instant;

use advent_2023::Coord;

const ANSWER: (&str, &str) = ("62365", "159485361249806");

fn main() {
    let _filename_test = "data/day18/test_input_01.txt";
    let _filename_test2 = "data/day18/test_input_02.txt";

    let filename_part1 = "data/day18/part1_input.txt";
    let filename_part2 = "data/day18/part2_input.txt";

    let start1 = Instant::now();
    let answer1 = part1(filename_part1);
    let duration1 = start1.elapsed();

    let start2 = Instant::now();
    let answer2 = part2(filename_part2);
    let duration2 = start2.elapsed();

    println!("Advent of Code, Day 18");
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
    let lines = advent_2023::file_to_lines(input_file);

    let mut current: Coord = Coord { x: 0, y: 0 };
    let mut prev: Coord = Coord { x: 0, y: 0 };

    let mut count: i64 = 0;
    let mut s: i64 = 0;


    for l in &lines {
        let parts: Vec<&str> = l.split(" ").collect();
        let dir_letter = parts[0];
        let number = parts[1];
        let n: i64 = number.parse().unwrap();
        let (dx, dy) = match dir_letter {
            "U" => { (0, n) }
            "D" => { (0, -n) }
            "L" => { (-n, 0) }
            "R" => { (n, 0) }
            x => { panic!("unexpected direction code: {}", x); }
        };
        current = Coord { x: current.x + dx, y: current.y + dy };

        s += current.x * prev.y - current.y * prev.x;
        count += n;
        prev = current;
    }


    let answer = s.abs() / 2 + count / 2 + 1;
    return answer.to_string();
}

const DIG_INDEX: [&str; 4] = ["R", "D", "L", "U"];

fn part2(input_file: &str) -> String {
    let lines = advent_2023::file_to_lines(input_file);

    let mut current: Coord = Coord { x: 0, y: 0 };
    let mut prev: Coord = Coord { x: 0, y: 0 };

    let mut count: i64 = 0;
    let mut s: i64 = 0;


    for l in &lines {
        let parts: Vec<&str> = l.split(" ").collect();

        let mut hex = parts[2];
        hex = &hex[2..7];
        let dig_num = parts[2].chars().nth(7).unwrap();
        let index: usize = (dig_num as u8 - '0' as u8) as usize;

        let n = i64::from_str_radix(hex, 16).unwrap();

        let dir_letter = DIG_INDEX[index];


        let (dx, dy) = match dir_letter {
            "U" => { (0, n) }
            "D" => { (0, -n) }
            "L" => { (-n, 0) }
            "R" => { (n, 0) }
            x => { panic!("unexpected direction code: {}", x); }
        };
        current = Coord { x: current.x + dx, y: current.y + dy };

        s += current.x * prev.y - current.y * prev.x;
        count += n;
        prev = current;
    }

    let answer = s.abs() / 2 + count / 2 + 1;
    return answer.to_string();
}