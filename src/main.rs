#![allow(unused_variables)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(dead_code)]
#![allow(unused_assignments)]
#![allow(unreachable_code)]

/*
    Advent of Code 2023: Day 07
        part1 answer:
        part2 answer:
 */


use std::collections::HashSet;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::str::FromStr;
use std::time::Instant;

use poker::{Evaluator, cards, Card, Suit, Rank};

const ANSWER: (&str, &str) = ("1108800", "36919753");

fn main() {
    let _filename_test = "data/day07/test_input_01.txt";
    let _filename_test2 = "data/day07/test_input_02.txt";

    let filename_part1 = "data/day07/part1_input.txt";
    let filename_part2 = "data/day07/part2_input.txt";

    let start1 = Instant::now();
    let answer1 = part1(_filename_test);
    let duration1 = start1.elapsed();

    let start2 = Instant::now();
    let answer2 = part2(_filename_test2);
    let duration2 = start2.elapsed();

    // println!("Advent of Code, Day 07");
    println!("    ---------------------------------------------");

    println!("\t Part 1: {:14} time: {:?}", answer1, duration1);
    if ANSWER.0 != answer1 {
        println!("\t\t ERROR: Answer is WRONG. Got: {}, Expected {}", answer1, ANSWER.0);
    }

    // println!("\t Part 2: {:14} time: {:?}", answer2, duration2);
    // if ANSWER.1 != answer2 {
    //     println!("\t\t ERROR: Answer is WRONG. Got: {}, Expected {}", answer2, ANSWER.1);
    // }
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
    let mut hands = Vec::new();

    for l in &lines {
        let (h,n) = l.split_once(" ").unwrap();
        let nn:i32 = n.parse().unwrap();
        hands.push((h,nn));
    }
    let eval = Evaluator::new();

        let suits:[char;4] =['c','h','s','d',];
    let mut e_hands = Vec::new();

    for i in 0..hands.len() {
        let mut cards: HashSet<Card> = HashSet::new();
        let (h,n) = hands[i];
        for c in h.chars() {
            let mut suit_index = 0;
            let mut cd = Card::try_from_chars(c, suits[suit_index]).unwrap();
            while cards.contains(&cd) {
                suit_index += 1;
                cd = Card::try_from_chars(c, suits[suit_index]).unwrap();
            }
            cards.insert(cd);
        }
        let v_card: Vec<Card> = cards.iter().map(|c| *c).collect();
        let e = eval.evaluate(v_card).unwrap();
        e_hands.push((h,n,e));

        println!("{:?} => {}", cards, e);
    }

    e_hands.sort_by_key(|(_,_,e)| e);
    return String::new();
}



fn part2(input_file: &str) -> String {
    let lines = file_to_lines(input_file);

    return String::new();
}

