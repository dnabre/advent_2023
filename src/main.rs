#![allow(unused_variables)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(dead_code)]
#![allow(unused_assignments)]
/*
    Advent of Code 2023: Day 01
        part1 answer: 54630
        part2 answer:

 */

// test_input_01 -> 142
//54788 is too high
//54762 is too low


// part two should be 54770

use std::collections::HashMap;
use std::fs;

fn main() {
    let filename_test = "data/day01/test_input_01.txt";
    let filename_part1 = "data/day01/part1_input.txt";
    let filename_part2 =  "data/day01/part2_input.txt";
    let filename_test2 = "data/day01/test_input_02.txt";
    let filename_test3 = "data/day01/test_input_03.txt";

    let answer1:String = part1(filename_part1);

   //  let answer2:String = part2(filename_test3);
    let answer2:String = part2(filename_part2);

    println!("Advent of Code, Day 01");
    println!("\t ---------------------------------------------");
    println!("\t part1: {}", answer1);
    println!("\t part2: {}", answer2);
    println!("\t ---------------------------------------------\n\t done")
}

fn part1(input_file: &str) -> String {
    let data_raw = fs::read_to_string(input_file).expect(&*format!("error opening file {}", input_file));
    let lines: Vec<&str> = data_raw.trim().split("\n").collect();
    let l_num = lines.len();

    println!("read {} lines", l_num);
    println!();

    let mut sum = 0;

    for l in lines.iter() {

        let line_answer = solve_line(*l);

   //     println!("{} => {}", l, line_answer);
        sum += line_answer;
    }
    return sum.to_string();
}


fn part2(input_file: &str) -> String {
    let data_raw = fs::read_to_string(input_file).expect(&*format!("error opening file {}", input_file));
    let lines: Vec<&str> = data_raw.trim().split("\n").map(|s| s.trim() ).collect();
    let l_num = lines.len();



    let mut sum = 0;

    let words: Vec<(&str, &str)> = vec![("one", "1"), ("two", "2"), ("three", "3"), ("four", "4"), ("five", "5"),
                                        ("six","6"), ("seven","7"), ("eight","8"), ("nine","9")
    ];

    let i_words:Vec<&str> = words.iter().map(|(a,b)| *a).collect();

  //  println!("i_words: {:?}", i_words);

    let mut new_lines:Vec<String> = Vec::new();
    let mut sum = 0;

    for line_number in 0..lines.len() {
        let l = lines[line_number].trim();
        let (n_line,b_line) = reduce_words(l , &words, &i_words);
        let line_value  =solve_line(n_line.as_str());
        let line_value2 = solve_line(b_line.as_str());

        let line_value = solve_line2(n_line.as_str(), b_line.as_str());

        sum += line_value;


        println!("{} \t {} \t {}", l, n_line, line_value);
        println!("{} \t {} \t {}", l, b_line, line_value2);
        //println!("{}", line_value);
        //println!("{} -> {}",l,  line_value);


        // println!("---------------------------------------------");
        // println!(" current line: {} ", l);
        // println!(" new line: {} ", n_line);
        // println!(" line result: {} ",line_value );


        //new_lines.push(n_line);
     //   println!("---------------------------------------------");
    }




    println!("{}  (target is {})", sum, 54770);
    println!("too high by {}", sum - 54770 );



    // for l in lines.iter() {
    //
    //     let (front,back) = solve_line_part1(*l);
    //     let line_answer = (front * 10) + back;
    //     //     println!("{} => {}", l, line_answer);
    //     sum += line_answer;
    // }
    return sum.to_string();
}

fn solve_line2(forward: &str, backword: &str) -> i32 {

    let letters: Vec<char> = backword.chars().collect();
    let mut first:Option<i32> = None;
    let mut back:Option<i32> = None;

    for c in letters.iter() {

        if c.is_digit( 10) {
            let c_v = (*c as i32)  - ('0' as i32);
            if first.is_none() {
                first = Some(c_v);
            }
            back = Some(c_v);
        }
    }
    let f_back = back.unwrap();

    let letters: Vec<char> = forward.chars().collect();
    let mut first:Option<i32> = None;
    let mut back:Option<i32> = None;

    for c in letters.iter() {

        if c.is_digit( 10) {
            let c_v = (*c as i32)  - ('0' as i32);
            if first.is_none() {
                first = Some(c_v);
            }
            back = Some(c_v);
        }
    }


    return (10*first.unwrap())+f_back;


}

fn reduce_words (line:&str, words: &Vec<(&str, &str)>, i_words: &Vec<&str>) -> (String, String) {
    let mut n_string = line.to_string();
    let mut b_string = line.to_string();
    println!("reducing {line}");
    let mut map_start_to_num:HashMap<usize,usize> = HashMap::new();
    let mut match_spots:Vec<(usize,usize)>  = Vec::new();

    for i in 0..i_words.len() {
       print!("\t {} ", i_words[i]);

        let f_results = n_string.find(i_words[i]);
     println!(" {:?}", f_results);
        match f_results {
            None => {}
            Some(x) => {
                match_spots.push((i+1,x));
                map_start_to_num.insert(x,i );

            }
        }
    }
    println!("match_spots: {:?}", match_spots);
   println!("map_start_to_num: {:?}", map_start_to_num);

    for c_i in 0..line.len() {
       let n = map_start_to_num.get(&c_i);
       match n {
           None => {}
           Some(i) => {
               let (w,w_n) = words[*i];
               n_string = n_string.replace(w,w_n);
              println!("replacing {} with {} to get {}", w, w_n, n_string);
           }
       }

    }

    for c_i in (0..line.len()).rev() {
        let n = map_start_to_num.get(&c_i);
        match n {
            None => {}
            Some(i) => {
                let (w,w_n) = words[*i];
                b_string = b_string.replace(w,w_n);
                println!("replacing {} with {} to get {}", w, w_n, b_string);
            }
        }
    }



    return (n_string,b_string);
}


fn solve_line(input_line: &str) -> i32 {
    let letters: Vec<char> = input_line.chars().collect();
    let mut first:Option<i32> = None;
    let mut back:Option<i32> = None;

    for c in letters.iter() {

        if c.is_digit( 10) {
            let c_v = (*c as i32)  - ('0' as i32);
            if first.is_none() {
                first = Some(c_v);
            }
            back = Some(c_v);
        }
    }

    return (10*first.unwrap())+back.unwrap();
}
