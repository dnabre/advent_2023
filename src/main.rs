#![allow(unused_variables)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(dead_code)]
#![allow(unused_assignments)]
#![allow(unreachable_code)]

/*
    Advent of Code 2023: Day 15
        part1 answer:   521341
        part2 answer:   252782

 */

use std::collections::VecDeque;
use std::fmt::Debug;
use std::path::Component::ParentDir;
use std::str::FromStr;
use std::time::Instant;

use advent_2023::file_to_lines;

const ANSWER: (&str, &str) = ("521341", "252782");

fn main() {
    let _filename_test1 = "data/day15/test_input_01.txt";
    let _filename_test2 = "data/day15/test_input_02.txt";

    let filename_part1 = "data/day15/part1_input.txt";
    let filename_part2 = "data/day15/part2_input.txt";

    let start1 = Instant::now();
    let answer1 = part1(filename_part1);
    let duration1 = start1.elapsed();

    let start2 = Instant::now();
    let answer2 = part2(filename_part2);
    let duration2 = start2.elapsed();

  //  println!("Advent of Code, Day 15");
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
    let l = advent_2023::file_to_single_line(input_file,None);
    let parts = l.split(",").collect::<Vec<_>>();

    let mut sum:u64 = 0;
    for p in &parts
    {
        let h = calculate_hash(p);
     //   println!("{} becomes {}", p, h);
        sum += h as u64;

    }



    return sum.to_string();
}

fn calculate_hash(s: &str) -> u8 {
    let mut result:u32 = 0;
    for ch in s.chars() {
        let ascii_value = ch as i8;
  //      println!("char: {ch} it's ASCII code is: {}", ascii_value);
        result += ascii_value as u32;
  //      println!("current value is increases to {}", result);
        result *= 17_u32;
  //      println!("current value is multiplied by 17 to become {}", result);
        result = result % 256;
  //      println!("current value is becomes  {}", result);
    }
    assert_eq!(true, result <256);
    return result as u8;
}


fn str_to_char_vec(s:&str) -> Vec<char> {
    let mut r_vec:Vec<char> = Vec::with_capacity(s.len());
    let b_array = s.as_bytes();
    for i in 0..s.len() {
        let ch:char = b_array[i] as char;
        r_vec.push(ch);
    }
    return r_vec;
}

fn part2(input_file: &str) -> String {
    let l = advent_2023::file_to_single_line(input_file,None);
    let parts = l.split(",").collect::<Vec<_>>();
    //let mut table: [Vec<u32>; 256] = [Vec::new(); 256];
    let mut table: Vec<Vec<(String,u32)>>  = Vec::with_capacity(256);
    for i in 0..256 {
        table.push(Vec::new());
    }

    for p in &parts {

        let mut sb = String::new();
        let c_array = str_to_char_vec(p);
        let mut i = 0;
        while i < c_array.len() && c_array[i] != '=' && c_array[i] != '-' {
            sb.push(c_array[i]);
            i += 1;
        }
        let key = sb.to_string();
        let key2 = key.clone();
        let h = calculate_hash(key.as_str());
        let h_idx = h as usize;
        let op = c_array[i];
        let mut num:Option<u32> = None;
        if op== '=' {
            i += 1;

            let mut sb = String::new();
            for j in i..c_array.len() {
                sb.push(c_array[i]);
            }
            num = Some(sb.parse().unwrap());
        }
      //let entry = (key,num.unwrap());
        //table[h_idx].push(entry);
        match op {
            '='=> {
                let entry = (key,num.unwrap());
                // println!("\tSearching table {h_idx} for key: {key2}");
                // println!("\t\t {:?}", table[h_idx]);
                let mut replaced = false;
                for t_idx in 0..table[h_idx].len() {
                    let (s,v) = &table[h_idx][t_idx];
                    // println!("\t\t checking table @ {t_idx:3} {:?}", (s,v));
                    if *s == key2 {
                        let r=table[h_idx].remove(t_idx);
                        table[h_idx].insert(t_idx, entry);
                        replaced = true;
                        // println!("\t\t removal result: {:?}", r);
                        break;
                    }
                }
                if !replaced {
                    table[h_idx].push((key2,num.unwrap()));
                }


            },
            '-' => {
                // println!("\tSearching table {h_idx} for key: {key2}");
                // println!("\t\t {:?}", table[h_idx]);
                for t_idx in 0..table[h_idx].len() {
                    let (s,v) = &table[h_idx][t_idx];
                    // println!("\t\t checking table @ {t_idx:3} {:?}", (s,v));
                    if *s == key {
                      let r=table[h_idx].remove(t_idx);
                        // println!("\t\t removal result: {:?}", r);
                        break;
                    }
                }

            }
            x =>{ panic!("unknown op: {x}")}
        }
        // println!("After \"{}\"", p);
        // print_table(&table);
        //println!("\t key: {key2}, op: {op} , num:{:?}, hash:{}", num, h);
        // println!();
    }

    let mut answer:usize = 0;
    for b in 0..table.len() {
        let v = &table[b];
        if !v.is_empty() {
            for i in 0..v.len() {
                let e@(k,v) = &v[i];
                let f_value:usize = (b+1) * (i+1) * (*v as usize);
                answer += f_value;
            }
        }
    }



    return answer.to_string();
}

fn print_table(table: &Vec<Vec<(String, u32)>>) {
    for t_idx in 0..table.len() {
        if table[t_idx].is_empty() {
            continue;
        }
        println!("Box {t_idx:3}: {:?}", table[t_idx]);

    }
}

