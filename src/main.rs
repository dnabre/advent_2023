#![allow(unused_variables)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(dead_code)]
#![allow(unused_assignments)]
#![allow(unreachable_code)]

use std::collections::HashMap;
use std::time::Instant;


/*
    Advent of Code 2023: Day 20
        part1 answer:
        part2 answer:

*/

const ANSWER: (&str, &str) = ("3642", "608603023105276");

fn main() {
    let _filename_test = "data/day20/test_input_01.txt";
    let _filename_test2 = "data/day20/test_input_02.txt";

    let filename_part1 = "data/day20/part1_input.txt";
    let filename_part2 = "data/day20/part2_input.txt";

    // println!("Advent of Code, Day 20");
    println!("    ---------------------------------------------");
    let start1 = Instant::now();
    let answer1 = part1(_filename_test);
    let duration1 = start1.elapsed();

    println!("\t Part 1: {:14} time: {:?}", answer1, duration1);
    if ANSWER.0 != answer1 {
        println!("\t\t ERROR: Answer is WRONG. Got: {}, Expected {}", answer1, ANSWER.0);
    }
    //
    // let start2 = Instant::now();
    // let answer2 = part2(filename_part2);
    // let duration2 = start2.elapsed();
    //
    // println!("\t Part 2: {:14} time: {:?}", answer2, duration2);
    // if ANSWER.1 != answer2 {
    //     println!("\t\t ERROR: Answer is WRONG. Got: {}, Expected {}", answer2, ANSWER.1);
    // }
    println!("    ---------------------------------------------");
}

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
enum Pulse {
    High,
    Low
}

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
struct PulseInstance {
    strength: Pulse,
    source: usize,
    dest: usize
}
#[derive(Debug, PartialEq, Eq, Hash, Clone)]
enum Module {
    FlipFlop {
        on: bool,
        output: Vec<usize>,
    },
    Conjunction {
        state: Vec<bool>,
        inputs: Vec<usize>,
        output: Vec<usize>
    },
    Broadcaster {
        output: Vec<usize>,
    }
}

const FLIP_FLOP_PREFIX: u8 =37_u8;
const CONJ_PREFIX: u8= 38u8;
const BROADCAST_PREFIX:u8=98_u8;

fn part1(input_file: &str) -> String {
    let lines = advent_2023::file_to_lines(input_file);

    let mut modules:Vec<Module>  = Vec::new();
    let mut name_lookup:HashMap<&str, usize> = HashMap::new();
    let mut name_by_id:HashMap<usize,&str> = HashMap::new();

    let mut m_broadcast_id:Option<usize> = None;






    let mut id:usize = 0;
    for l in &lines {
        println!("input: {}", l);
        let (src, dest) = l.split_once("->").unwrap();
        println!("\t src: {}, dest: {}", src, dest);
        let src = src.trim();
        println!("\t src: {src}");
        let dest_list:Vec<&str> = dest.split(",").map(|s| s.trim()).collect();
        println!("\t dest: {} ", advent_2023::list_displayables_to_string(&dest_list));
        let src_id = add_if_new(&mut name_lookup,&mut id, src  );
        name_by_id.insert(src_id, src);
        let mut dest_ids:Vec<usize> = Vec::new();
        for i in 0..dest_list.len() {
            let dest_id = add_if_new(&mut name_lookup, &mut id, dest_list[i]);
            name_by_id.insert(dest_id,dest_list[i]);
            dest_ids.push(dest_id);
        }

        let f = src.as_bytes()[0];
        println!("\t src.as_bytes[0] : {}", f);

        let mut m = match f {
            FLIP_FLOP_PREFIX => {
                Module::FlipFlop {on: false, output: dest_ids}
            },
            CONJ_PREFIX => {
                Module::Conjunction {
                    state: vec![],
                    inputs: vec![],
                    output: dest_ids,
                }
            },
            BROADCAST_PREFIX => {
                let t = Module::Broadcaster { output: dest_ids };
                m_broadcast_id= Some(src_id);
                t
            },
            _ => {panic!("unable to parse module: {}", f)}
        };
        println!("module: {src_id:3}  :{:?}", m);
        modules.push(m);
    }

    println!(" ------------------------------ ");
    for i in 0..modules.len() {
        let name = name_by_id[&i];
        let id = name_lookup[&name];
        println!("{i:3} -> \t ({id:3},{name}) \t{:?}", modules[i]);
    }





    let answer = 0;
    return answer.to_string();
}

fn add_if_new<'a>(map: &mut HashMap<&'a str, usize>, id_num: &mut usize, value: &'a str) -> usize  {
    let current = map.get(&value);
    let and_prefix = value.strip_prefix("&");
    let pre_prefix = value.strip_prefix("%");
    let mut new_value = value;
    if let Some(s) = and_prefix {
        new_value = s;
    } else if let Some(s) = pre_prefix {
        new_value = s;
    }

    let value = new_value;


    match current {
        None => {
            let new_id = *id_num;
            *id_num += 1;
            map.insert(value, new_id) ;
   //         println!("assign id: {} to {}", new_id, value);
            return new_id;
        }
        Some(id) => {
            return *id;
        }
    }
}


fn part2(input_file: &str) -> String {
    let lines = advent_2023::file_to_lines(input_file);


    let answer = 0;
    return answer.to_string();
}
