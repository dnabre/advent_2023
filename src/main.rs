// #![allow(unused_variables)]
// #![allow(unused_imports)]
// #![allow(unused_mut)]
// #![allow(dead_code)]
// #![allow(unused_assignments)]
// #![allow(unreachable_code)]

/*
    Advent of Code 2023: Day 15
        part1 answer:   521341
        part2 answer:   252782

 */



use std::time::Instant;

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

    println!("Advent of Code, Day 15");
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
    let l = advent_2023::file_to_single_line(input_file, None);
    let parts = l.split(",").collect::<Vec<_>>();

    let mut sum: u64 = 0;
    for p in &parts {
        let h = calculate_hash(p);
        sum += h as u64;
    }

    return sum.to_string();
}

fn part2(input_file: &str) -> String {
    let l = advent_2023::file_to_single_line(input_file, None);
    let parts = l.split(",").collect::<Vec<_>>();
    let mut table: Vec<Vec<(String, u32)>> = Vec::with_capacity(256);
    for _ in 0..256 {
        table.push(Vec::new());
    }

    for p in &parts {
        let mut sb = String::new();
        let c_array = advent_2023::str_to_char_vec(p);
        let mut i = 0;
        while i < c_array.len() && c_array[i] != '=' && c_array[i] != '-' {
            sb.push(c_array[i]);
            i += 1;
        }
        let key = sb.to_string();
        let h = calculate_hash(key.as_str());
        let h_idx = h as usize;
        let op = c_array[i];
        let mut num: Option<u32> = None;
        if op == '=' {
            i += 1;

            let mut sb = String::new();
            for _ in i..c_array.len() {
                sb.push(c_array[i]);
            }
            num = Some(sb.parse().unwrap());
        }

        match op {
            '=' => {
                let entry = (key.clone(), num.unwrap());
                let mut replaced = false;
                for t_idx in 0..table[h_idx].len() {
                    let (s, _) = &table[h_idx][t_idx];
                    if *s == key {
                        table[h_idx].remove(t_idx);
                        table[h_idx].insert(t_idx, entry.clone());
                        replaced = true;
                        break;
                    }
                }
                if !replaced {
                    table[h_idx].push(entry.clone());
                }
            }
            '-' => {
                for t_idx in 0..table[h_idx].len() {
                    let (s, _) = &table[h_idx][t_idx];
                    if *s == key {
                        table[h_idx].remove(t_idx);
                        break;
                    }
                }
            }
            x => { panic!("unknown op: {}",x) }
        }
    }

    let mut answer: usize = 0;
    for b in 0..table.len() {
        let v = &table[b];
        if !v.is_empty() {
            for i in 0..v.len() {
                let (_, v) = &v[i];
                let f_value: usize = (b + 1) * (i + 1) * (*v as usize);
                answer += f_value;
            }
        }
    }


    return answer.to_string();
}

fn calculate_hash(s: &str) -> u8 {
    let mut result: u32 = 0;
    for ch in s.chars() {
        let ascii_value = ch as i8;
        result += ascii_value as u32;
         result *= 17_u32;
         result = result % 256;
      }
    return result as u8;
}
