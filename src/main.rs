#![allow(unused_variables)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(dead_code)]
#![allow(unused_assignments)]
#![allow(unreachable_code)]

/*
    Advent of Code 2023: Day 18
        part1 answer:   62365
        part2 answer:   159485361249806


 */


use std::collections::HashMap;
use std::fmt::{Display, Formatter};
use std::path::Component::ParentDir;

use std::time::Instant;

const ANSWER: (&str, &str) = ("62365", "159485361249806");

fn main() {
    let _filename_test = "data/day19/test_input_01.txt";
    let _filename_test2 = "data/day19/test_input_02.txt";

    let filename_part1 = "data/day19/part1_input.txt";
    let filename_part2 = "data/day19/part2_input.txt";

    let start1 = Instant::now();
    let answer1 = part1(_filename_test);
    let duration1 = start1.elapsed();

    let start2 = Instant::now();
    let answer2 = part2(_filename_test2);
    let duration2 = start2.elapsed();

    //  println!("Advent of Code, Day 19");
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

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
struct Part {
    x: i64,
    m: i64,
    a: i64,
    s: i64,
}
// px{a<2006:qkq,m>2090:A,rfg}
#[derive(Debug, PartialEq, Eq, Hash, Clone)]
struct Rule{
     start_queue: String,
    last_queue: String,
    compares:Vec<RulePart>
}
impl Display for Rule {
    fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
        write!(f, " {}: {{ {} : {} }}", self.start_queue, advent_2023::list_displayables_to_string(&self.compares), self.last_queue)
    }
}

#[derive(Debug, PartialEq, Eq, Hash, Clone)]
struct RulePart {
    xmas_letter:char,
    op: char,
    num: i32,
    queue_name: String
}
impl Display for RulePart {
    fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
        write!(f, "{} {} {} : {}", self.xmas_letter, self.op, self.num,self.queue_name )
    }
}

fn xmas_to_index(ch:char) -> usize {
    match ch {
        'x' => {0},
        'm' => {1},
        'a' => {2},
        's' => {3},
        x => {panic!("naught char isn't xmas: {}", x)}
    }
}


fn part1(input_file: &str) -> String {
    let lines = advent_2023::file_to_lines(input_file);


    let mut rule_list:Vec<Rule> = Vec::new();
    let mut index: usize = 0;
    println!("rules:");

    while lines[index] != "" {
        println!("{}", lines[index]);
        let new_rule:Rule = parse_line(&lines[index]);
        println!("{index:3}: {}", new_rule);
        rule_list.push(new_rule);
        index += 1;
    }
    index += 1;
    let mut parts_list: Vec<Part> = Vec::new();


    while index < lines.len() {
        let p = parse_xmas(&lines[index]);
        parts_list.push(p);
        index += 1;
    }
    let parts_list = parts_list;

    let mut rule_map: HashMap<&str, usize> =HashMap::new();
    for i in 0..rule_list.len() {
        rule_map.insert(rule_list[i].start_queue.as_str(), i);
    }

    let mut accept_list:Vec<usize> = Vec::new();
    let mut reject_list:Vec<usize> = Vec::new();
    for part_idx in 0..parts_list.len() {
        let mut current_queue = "in";
        while current_queue != "A" && current_queue != "R" {

        }
        if current_queue == "A" {
            accept_list.push(part_idx);
        } else {
            reject_list.push(part_idx);
        }
    }

    let mut answer:i64=0;
    for a_idx in 0..accept_list.len() {
        let part = parts_list[accept_list[a_idx]];
        let part_total = part.x + part.m + part.a + part.s;
        println!("part: {:?} gives {}", part, part_total);
        answer += part_total;
    }



    return answer.to_string();
}

fn parse_line(input_line: &String) -> Rule {
    let (s_queue, rest) = input_line.split_once("{").unwrap();
    //   println!("start_queue: {}", start_queue);
    let r_parts: Vec<&str> = rest.split(",").collect();
    //  println!("r_parts: {:?}", r_parts);
    //
    // let mut new_rule = Rule{
    //     start_queue: s_queue.to_string(),
    //     last_queue: "".into_string(),
    //     compares: vec![],
    // };
    let mut n_compares:Vec<RulePart> = Vec::new();

    for i in 0..r_parts.len() - 1 {
        let rp = r_parts[i];
        let letter = rp.as_bytes()[0] as char;
        let comparsion = rp.as_bytes()[1] as char;
        let num_c_queue  = &rp[2..];
        let (num,n_queue) = num_c_queue.split_once(":").unwrap();
        let z:i32 = num.parse().unwrap();
        let new_rule_part = RulePart {
            xmas_letter: letter,
            op: comparsion,
            num: z,
            queue_name: n_queue.to_string(),
        };
        //println!("\t rule_part: {:?}", new_rule_part);
        n_compares.push(new_rule_part);
    }
    let rp:&str =* r_parts.last().unwrap();
    let last_queue =rp[0..rp.len()-1].to_string();
    let new_rule = Rule{
        start_queue: s_queue.to_string(),
        last_queue: last_queue,
        compares: n_compares,
    };
    return new_rule;

}


fn parse_xmas(line: &String) -> Part {
    let parts: Vec<_> = line.split(",").map(|s| s.trim()).collect();
    let x = &parts[0][3..parts[0].len()];
    let m = &parts[1][2..];
    let a = &parts[2][2..];
    let s = &parts[3][2..parts[3].len() - 1];
    let p = Part {
        x: x.parse().unwrap(),
        m: m.parse().unwrap(),
        a: a.parse().unwrap(),
        s: s.parse().unwrap(),
    };
    p
}


fn part2(input_file: &str) -> String {
    let lines = advent_2023::file_to_lines(input_file);


    let answer = 0;
    return answer.to_string();
}