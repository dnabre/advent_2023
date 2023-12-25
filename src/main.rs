#![allow(unused_variables)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(dead_code)]
#![allow(unused_assignments)]
#![allow(unreachable_code)]

use std::collections::{BTreeSet, HashMap, HashSet};
use std::hash::Hash;
use std::time::Instant;
use rand::Rng;


/*
    Advent of Code 2023: Day 25
        part1 answer:   485
        part2 answer:   74594

*/
const ANSWER: (&str, &str) = ("485", "74594");

fn main() {
    let _filename_test = "data/day25/test_input_01.txt";
    let _filename_test2 = "data/day25/test_input_02.txt";

    let filename_part1 = "data/day25/part1_input.txt";
    let filename_part2 = "data/day25/part2_input.txt";

    println!("Advent of Code, Day 25");
    println!("    ---------------------------------------------");
    let start1 = Instant::now();
    let answer1 = part1(_filename_test);
    let duration1 = start1.elapsed();

    println!("\t Part 1: {:14} time: {:?}", answer1, duration1);
    if ANSWER.0 != answer1 {
        println!("\t\t ERROR: Answer is WRONG. Got: {}, Expected {}", answer1, ANSWER.0);
    }

    let start2 = Instant::now();
    let answer2 = part2(_filename_test2);
    let duration2 = start2.elapsed();

    println!("\t Part 2: {:14} time: {:?}", answer2, duration2);
    if ANSWER.1 != answer2 {
        println!("\t\t ERROR: Answer is WRONG. Got: {}, Expected {}", answer2, ANSWER.1);
    }
    println!("    ---------------------------------------------");
}

#[derive(Debug, PartialEq, Eq, Hash, Clone)]
struct Node {
    id: usize,
    adjacent: BTreeSet<usize>
}

impl Node {
    fn add_edge_to(&mut self, a_vertex  : usize) -> bool {
       return self.adjacent.insert(a_vertex);
    }
}

type Graph = (V,E);
type V = HashSet<String>;
type E = Vec<(String,String)>;




fn part1(input_file: &str) -> String {
    let lines = advent_2023::file_to_lines(input_file);

    let (vertices,edges) = parse_input(&lines);

    let mut rng = rand::thread_rng();

    loop {
        let mut vertices = vertices.clone();
        let mut edges = edges.clone();
        while vertices.len() > 2 {
            let i = rng.gen_range(0..edges.len());
            let (v1,v2) = edges[i].clone();

            edges.swap_remove(i);
            vertices.remove(&v1);
            vertices.remove(&v2);




        }
    }




    let answer = 0;
    return answer.to_string();
}


fn parse_input(lines:&Vec<String>)->Graph {
    let n = lines.len();
    let mut vertices:HashSet<String> = HashSet::with_capacity(n);
    let mut edges:Vec<(String,String)> = Vec::with_capacity(n*n);

    for i in 0..n {
        let line = &lines[i];
        let (left,right_side) = line.split_once(":").unwrap();
        vertices.insert(left.to_string());
        for right in right_side.split_whitespace() {
            vertices.insert(right.to_string());
            edges.push((left.to_string(),right.to_string()));
        }
    }
    return (vertices, edges);
}


fn convert_adj_list_to_adj_map(node_list: &Vec<Node>) -> HashMap<usize, Vec<usize>> {
    let mut graph:HashMap<usize, Vec<usize>> = HashMap::new();
    let num_nodes = node_list.len();
    for i in 0..num_nodes {
        let mut a_list:Vec<usize> = Vec::new();
        for u in &node_list[i].adjacent {
            a_list.push(*u);
        }
        graph.insert(i,a_list);
    }
    return graph;
}

fn part2(input_file: &str) -> String {
    let lines = advent_2023::file_to_lines(input_file);

    let answer = 0;
    return answer.to_string();
}
