#![allow(unused_variables)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(dead_code)]
#![allow(unused_assignments)]
#![allow(unreachable_code)]

use std::collections::HashMap;
use std::sync::atomic::AtomicU64;
use std::sync::atomic::Ordering::SeqCst;
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
    Low,
}

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
struct PulseInstance {
    strength: Pulse,
    source: usize,
    dest: usize,
}

static HIGH_PULSE_COUNT: AtomicU64 = AtomicU64::new(0);
static LOW_PULSE_COUNT: AtomicU64 = AtomicU64::new(0);

impl PulseInstance {
    fn new(strength: Pulse, source: usize, dest: usize) -> Self {
        match strength {
            Pulse::High => {
                HIGH_PULSE_COUNT.fetch_add(1, SeqCst);
            }
            Pulse::Low => {
                LOW_PULSE_COUNT.fetch_add(1, SeqCst);
            }
        }
        Self { strength, source, dest }
    }
}


#[derive(Debug, PartialEq, Eq, Hash, Clone)]
enum Module {
    FlipFlop {
        on: bool,
        output: Vec<usize>,
    },
    Conjunction {
        state: Vec<Pulse>,
        inputs: Vec<usize>,
        output: Vec<usize>,
    },
    Broadcaster {
        output: Vec<usize>,
    },
}

const FLIP_FLOP_PREFIX: u8 = 37_u8;
const CONJ_PREFIX: u8 = 38u8;
const BROADCAST_PREFIX: u8 = 98_u8;

fn part1(input_file: &str) -> String {
    let lines = advent_2023::file_to_lines(input_file);

    let mut modules: Vec<Module> = Vec::new();
    let mut name_lookup: HashMap<&str, usize> = HashMap::new();
    let mut name_by_id: HashMap<usize, &str> = HashMap::new();
    let mut module_names: Vec<&str> = Vec::new();
    let mut output_modules: Vec<(&str, usize)> = Vec::new();

    let mut m_broadcast_id: Option<usize> = None;

    let mut assign_id = 0;
    for l in &lines {
        let (src, dest) = l.split_once("->").unwrap();
        let mut src = trim_module_prefix(src.trim());
        module_names.push(src);
        name_by_id.insert(assign_id, src);
        name_lookup.insert(src, assign_id);
        assign_id += 1;
    }

    let mut conj_list: Vec<usize> = Vec::new();
    let mut id: usize = 0;
    for l in &lines {
        let (src, dest) = l.split_once("->").unwrap();
        let src = src.trim();
        let dest_list: Vec<&str> = dest.split(",").map(|s| s.trim()).collect();

        let src_id = name_lookup[trim_module_prefix(src)];

        let mut dest_ids: Vec<usize> = Vec::new();
        for i in 0..dest_list.len() {
            let key = dest_list[i];

            if !name_lookup.contains_key(&key) {
                let n_dest_id = assign_id;
                assign_id += 1;

                name_lookup.insert(key, n_dest_id);
                name_by_id.insert(n_dest_id, key);
                module_names.push(key);

                output_modules.push((key, n_dest_id));
            }
            let dest_id = name_lookup[dest_list[i]];
            dest_ids.push(dest_id);
        }
        let f = src.as_bytes()[0];
        let m = match f {
            FLIP_FLOP_PREFIX => {
                Module::FlipFlop { on: false, output: dest_ids }
            }
            CONJ_PREFIX => {
                conj_list.push(src_id);
                Module::Conjunction {
                    state: vec![],
                    inputs: vec![],
                    output: dest_ids,
                }
            }
            BROADCAST_PREFIX => {
                let t = Module::Broadcaster { output: dest_ids };
                m_broadcast_id = Some(src_id);
                t
            }
            _ => {
                panic!("No Untyped Modules should be sources: {}", l);
            }
        };
        modules.push(m);
    }
    let mut broadcast_id;
    if let Some(b) = m_broadcast_id {
        broadcast_id = b;
    } else {
        panic!("didn't find broadcast id");
    }
    let broadcast_id = broadcast_id;

    println!("    ---------------------------------------------");
    println!("broadcast: {}", broadcast_id);
    println!("output sinks: {:?}", output_modules);
    println!("    ---------------------------------------------");


    for i in 0..modules.len() {
        let mut m = &modules[i];
        if let Module::Conjunction { state: ref states, inputs: ref ins, output: ref outs } = m {
            assert_eq!(true, ins.is_empty());
            assert_eq!(true, states.is_empty());
            let mut new_states = Vec::new();
            let mut new_inputs = Vec::new();
            for j in 0..modules.len() {
                if i == j { continue; }
                match &modules[j] {
                    Module::FlipFlop { on, output } => {
                        if output.contains(&i) {
                            new_inputs.push(j);
                            new_states.push(Pulse::Low);
                        }
                    }
                    Module::Conjunction { state, inputs, output } => {
                        if output.contains(&i) {
                            new_inputs.push(j);
                            new_states.push(Pulse::Low);
                        }
                    }
                    Module::Broadcaster { output } => {
                        if output.contains(&i) {
                            new_inputs.push(j);
                            new_states.push(Pulse::Low);
                        }
                    }
                }
            }
            let new_conj = Module::Conjunction {
                state: new_states,
                inputs: new_inputs,
                output: outs.clone(),
            };
            modules[i] = new_conj;
        }
    }






let answer = 0; return answer.to_string(); }

fn part2(input_file: &str) -> String {
    let lines = advent_2023::file_to_lines(input_file);


    let answer = 0;
    return answer.to_string();
}

fn trim_module_prefix(i_src: &str) -> &str {
    let mut src = i_src;
    if i_src.starts_with("%") {
        src = i_src.strip_prefix("%").unwrap();
    }
    if i_src.starts_with("&") {
        src = i_src.strip_prefix("&").unwrap();
    }
    src
}