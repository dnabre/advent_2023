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

 part1: 248455100  - too high

 */


use std::cmp::Ordering;
use std::collections::{HashMap, HashSet};
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::io::ErrorKind::AddrNotAvailable;
use std::str::FromStr;
use std::time::Instant;


const ANSWER: (&str, &str) = ("245794640", "247899149");

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
#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd,Ord)]
enum Score {
    HighCard(usize),
    OnePair(usize),
    TwoPair(usize, usize),
    // first will be higher rank
    ThreeOfKind(usize),
    FullHouse(usize, usize),
    // first is triple, second pair
    FourOfKind(usize),
    FiveOfKind(usize),
}
// impl Ord for Score {
//     fn cmp(&self, other: &Self) -> Ordering {
//         if self < other {
//             return Ordering::Less;
//         } else if self > other {
//             return Ordering::Greater;
//         }
//         return Ordering::Equal;
//     }
// }

const CARD_VALUES: [char; 13] = ['2', '3', '4', '5', '6', '7', '8', '9', 'T', 'J', 'Q', 'K', 'A'];
const HAND_SIZE:usize = 5;

fn part1(input_file: &str) -> String {
    let lines = file_to_lines(input_file);


    let mut char_to_index: HashMap<char, usize> = HashMap::with_capacity(CARD_VALUES.len());
    for i in 0..CARD_VALUES.len() {
        char_to_index.insert(CARD_VALUES[i], i);
    }
    char_to_index.shrink_to_fit();
    let char_to_index = char_to_index;
    let mut scored_hands: Vec<(Score, usize)> = Vec::new();
    let mut hands: Vec<[usize; 5]> = Vec::new();
    //let l = lines[0].clone();
    print!("Card Mapping: ");
    for i in (1..CARD_VALUES.len()).rev() {
        let ch = CARD_VALUES[i];
        print!(" {ch}:{i},")
    }
    println!(" {}:{}", CARD_VALUES[0], 0);
    for l in lines
    {
        print!("{}    ", l);
        let mut counts: [usize; 13] = [0; 13];
        let mut hand: [usize; 5] = [0, 0, 0, 0, 0];
        let (chand, cbid) = l.split_once(" ").unwrap();
        let bid: usize = cbid.parse().unwrap();
        let char_array: Vec<_> = chand.chars().collect();
        for i in 0..HAND_SIZE {
            let c = char_array[i];
            let count_i: usize = char_to_index[&c];
            counts[count_i] += 1;
            hand[i] = count_i;

        }

        let hand_max = counts.iter().max().unwrap();

        let score:Score = match hand_max {
            &5 => { Score::FiveOfKind(hand[0]) },
            &4 => {
                let mut k:usize = 0;
                for i in 0..counts.len() {
                    if counts[i] == 4 {
                        k = i;
                        break;
                    }
                }
                Score::FourOfKind(k)
            }
            &3 => {
                let mut three:Option<usize> = None;
                let mut two:Option<usize> = None;
                for i in 0..counts.len() {
                    if counts[i] == 3 {
                        three = Some(i);
                        if two.is_some() {
                            break;
                        }
                    }
                    if counts[i] == 2 {
                        two = Some(i);
                        if three.is_some() {
                            break;
                        }
                    }
                }
                if two.is_none() {
                    Score::ThreeOfKind(three.unwrap())
                } else {
                    assert_eq!(true, three.is_some());
                    Score::FullHouse(three.unwrap(),two.unwrap() )
                }
            }
            &2 => {
                let mut a:Option<usize> = None;
                let mut b:Option<usize> = None;
                for i in 0..counts.len() {
                    if counts[i] == 2 {
                        (a,b) = match (a,b) {
                            (None,None) => { (Some(i), None)}
                            (Some(x), None)=> {
                                //(a,b) = (Some(x), Some(i));
                                let n_a = x.max(i);
                                let n_b = x.min(i);
                                (a,b) = (Some(n_a), Some(n_b));

                            break;
                            }
                            (Some(a), Some(b)) => {
                                println!("found a third pair! Pair({a}), Pair({b}), Pair({i} ");
                                return String::new();
                            }
                            _ => {println!("no pairs found with hand_max = {hand_max}, in hand: {:?}",hand);
                                return String::new();}
                        }
                    }
                }
                if b.is_none() {
                    Score::OnePair(a.unwrap())
                } else {
                    Score::TwoPair(a.unwrap(), b.unwrap())
                }
            }
            &1 => { //High card
                let mut k:usize =0;
                for i in (0..counts.len()).rev() {
                    if counts[i] == 1 {
                        k = i;
                        break;
                    }
                }
                Score::HighCard(k)
            }
            _ => {
                println!    ("unable to score hand: {:?} with max {}", hand, hand_max);
                return String::new();
            }
        };


        print!("\t hand: {},{:?},{bid} \t scored to {:?}", chand,hand, score );
        let d = std::mem::discriminant(&score);
        print!("disc: {:?}", d);
        println!();
        hands.push(hand);
        scored_hands.push((score,bid));
    }
    println!("{:?}", scored_hands);




    return String::new()
}


fn part2(input_file: &str) -> String {
    let lines = file_to_lines(input_file);

    return String::new();
}

