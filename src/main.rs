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

impl Node   {

}




fn part1(input_file: &str) -> String {
    let lines = advent_2023::file_to_lines(input_file);

    let mut id_lookup:HashMap<&str,usize> = HashMap::new();
    let mut name_list:Vec<&str> = Vec::new();
    let mut labeled:HashSet<&str> = HashSet::new();
    let mut node_list:Vec<Node> = Vec::new();


    let mut lefts:Vec<&str> = Vec::new();
    let mut part_id =0;
    for l in &lines {
        let (left,right) = l.split_once(":").unwrap();
        let  mut r_list:Vec<&str> = right.split_whitespace().collect();
        r_list.push(left);

        for name in r_list.iter() {
            if !labeled.contains(name) {
                id_lookup.insert(name, part_id);
                name_list.push(name);
                labeled.insert(name);
                let n = Node { id: part_id, adjacent: BTreeSet::new() };
                node_list.push(n);
                part_id += 1;
            }
        }
    }
    let node_count = node_list.len();
    println!("id_lookup: {:?}", id_lookup);

    for l in &lines {
        let (left,right) = l.split_once(":").unwrap();
        let  r_list:Vec<&str> = right.split_whitespace().collect();
        let left_id = id_lookup[left];
            for right in r_list {
                let right_id = id_lookup[right];
                node_list[left_id].add_edge_to(right_id);
                node_list[right_id].add_edge_to(left_id);
            }
    }

    let mut map_graph:HashMap<usize,Vec<usize>> = convert_adj_list_to_adj_map(&node_list);



    let answer = 0;
    return answer.to_string();
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



fn karger_algorithm(graph: &mut HashMap<usize, Vec<usize>>) -> usize {
    while graph.len() > 2 {
        let keys: Vec<_> = graph.keys().cloned().collect();
        let u = rand::thread_rng().gen_range(0..keys.len());
        let u_key = keys[u];
        let v = rand::thread_rng().gen_range(0..graph[&u_key].len());
        let w_key = graph[&u_key][v];

        let w_key_values = graph[&w_key].clone();
        graph.get_mut(&u_key).unwrap().extend(w_key_values.clone());
        println!("w_key_values: {:?}", w_key_values);
        for node in w_key_values.iter() {
            let m_node_list = graph.get_mut(node);
            if let Some(node_list) = m_node_list {
                node_list.retain(|x| *x != w_key);
                node_list.push(u_key);
            }
        }
        graph.remove(&w_key);
    }
    graph.values().next().unwrap().len()

}