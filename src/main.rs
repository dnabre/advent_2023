#![allow(unused_variables)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(dead_code)]
#![allow(unused_assignments)]

/*
    Advent of Code 2023: Day 04
        part1 answer:   18653
        part2 answer:   5921508

 */

use std::collections::{HashMap, HashSet, VecDeque};
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::num::ParseIntError;
use std::time::Instant;

const ANSWER: (&str, &str) = ("18653", "5921508");

fn main() {
    let _filename_test = "data/day04/test_input_01.txt";
    let _filename_test2 = "data/day04/test_input_02.txt";

    let filename_part1 = "data/day04/part1_input.txt";
    let filename_part2 = "data/day04/part2_input.txt";

    let start1 = Instant::now();
    let answer1 = part1(filename_part1);
    let duration1 = start1.elapsed();

    let start2 = Instant::now();
    let answer2 = part2(filename_part2);
    let duration2 = start2.elapsed();

    println!("Advent of Code, Day 04");
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


fn part1(input_file: &str) -> String {
    let lines = file_to_lines(input_file);



    let mut total_score = 0;
    for l in &lines {
        let (_, wins, numbers) = parse_line(l);
        let score = score_card(&wins,&numbers);
        total_score += score;
    }

    return total_score.to_string();
}


fn part2(input_file: &str) -> String {
    let lines = file_to_lines(input_file);
    let number_cards = lines.len();
    let mut cards:Vec<(HashSet<usize>, Vec<usize>)> = Vec::new();
    let blank_place_holder = (HashSet::with_capacity(0), Vec::with_capacity(0));
    cards.push(blank_place_holder);

    let mut base_scores: Vec<usize> = Vec::with_capacity(lines.len() + 1);
    base_scores.push(0);

    for input_line in &lines
    {
        let (card_number,winners,ours) = parse_line(input_line.as_str());
        let card_score = score_card(&winners,&ours);
        base_scores.push(card_score);
        cards.push((winners,ours));
    }



    let mut card_copies:Vec<usize> = vec![0;number_cards +1];
    card_copies[0] = 0;

    let mut winlist_by_card: Vec<Vec<usize>> = Vec::with_capacity(number_cards+1);
    for i in 0..cards.len() {
        let (w,o) = &cards[i];
        let win_list = score_wins_card(w,o,i);
        winlist_by_card.push(win_list);
    }




    for current_card in 1..cards.len() {
        let mut work_queue:VecDeque<usize> = VecDeque::new();
        work_queue.push_front(current_card);
        while !work_queue.is_empty() {
            let de_card = work_queue.pop_front().unwrap();
            card_copies[de_card] += 1;
            for nu_de in &winlist_by_card[de_card] {
                work_queue.push_front(*nu_de);
            }
        }
    }
    let mut sum:usize=0;
    for i in 0..card_copies.len() {
        let copies = card_copies[i];
        sum += copies;
    }




    return sum.to_string();
}

fn parse_line(input_line:&str) -> (usize,HashSet<usize>,Vec<usize>) {
    let r = input_line.split_once(":");
    let (raw_card,numbers) = match r {
        None => {
            println!("unable to split line on \":\" : ");
            println!("|{}|", input_line);
            panic!();
        }
        Some(r) => {r}
    };


    let s_card = &raw_card[5..].trim();

    let r_card_number = s_card.parse::<usize>();
    let card_number = match r_card_number {
        Ok(x) => {x}
        Err(e) => {panic!("parse error on {s_card}: {e}")}
    };

    let (s_winning,s_ours) = numbers.split_once("|").unwrap();

    let n_wins: HashSet<usize> =
        HashSet::from_iter(parse_number_list_whitespace(s_winning).iter().map(|s| *s as usize));
    let n_ours =
        parse_number_list_whitespace(s_ours).iter().map(|s| *s as usize).collect();
    return (card_number,n_wins,n_ours);
}


fn score_card(n_wins:&HashSet<usize>, n_ours:&Vec<usize>) -> usize {
    let mut wins = 0;
    for n in n_ours {
        if n_wins.contains(&n) {
            wins += 1;
        }
    }
    static BASE: usize = 2;
    if wins == 0 {
        return 0;
    } else
    {
        return BASE.pow(wins - 1);
    }
}


fn score_wins_card(n_wins:&HashSet<usize>, n_ours:&Vec<usize>, i: usize) -> Vec<usize> {
    let mut wins:Vec<usize> = Vec::new();
    let mut c_card_number = i;
    for n in n_ours {
        if n_wins.contains(&n) {
            c_card_number += 1;
            wins.push(c_card_number);
        }
    }
    return wins;
}



fn parse_number_list_whitespace(number_string: &str) -> Vec<i32> {
    number_string.split_whitespace().map(|s| s.trim().parse().unwrap()).collect()
}

