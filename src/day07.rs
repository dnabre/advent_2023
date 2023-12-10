/*
    Advent of Code 2023: Day 07
        part1 answer:   245794640
        part2 answer:   247899149

 */

use std::cmp::Ordering;
use std::collections::{HashMap};
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::time::Instant;


const ANSWER: (&str, &str) = ("245794640", "247899149");

fn main() {
    let _filename_test = "data/day07/test_input_01.txt";
    let _filename_test2 = "data/day07/test_input_02.txt";

    let filename_part1 = "data/day07/part1_input.txt";
    let filename_part2 = "data/day07/part2_input.txt";

    let start1 = Instant::now();
    let answer1 = part1(filename_part1);
    let duration1 = start1.elapsed();

    let start2 = Instant::now();
    let answer2 = part2(filename_part2);
    let duration2 = start2.elapsed();

    println!("Advent of Code, Day 07");
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

//noinspection DuplicatedCode
fn file_to_lines(input_file: &str) -> Vec<String> {
    let file = File::open(input_file).expect(&*format!("error opening file {}", input_file));
    let bfile = BufReader::new(file);
    let lines: Vec<String> = bfile.lines().filter_map(|x| x.ok()).collect();
    return lines;
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd,Ord)]
enum Score {
    HighCard(usize),
    OnePair(usize),
    TwoPair(usize, usize),
    ThreeOfKind(usize),
    FullHouse(usize, usize),
    FourOfKind(usize),
    FiveOfKind(usize),
}

#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
enum StrengthType {
    HighCard = 1,
    OnePair = 2,
    TwoPair = 3,
    ThreeOfAKind = 4,
    FullHouse = 5,
    FourOfAKind = 6,
    FiveOfAKind = 7,
}
#[derive(Copy, Clone, Debug)]
struct Hand {
    str:StrengthType,
    cards:[usize;5],
    bid:usize,
}
impl PartialEq for Hand {
    fn eq(&self, other: &Self) -> bool {
        self.str == other.str &&
            other.bid == other.bid &&
            self.cards == other.cards
    }
}

impl PartialOrd for Hand {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        if self.eq(other) {
            return Some(Ordering::Equal);
        } else if self.str == other.str {
            for i in 0..HAND_SIZE {
                let (a,b) =  (self.cards[i], other.cards[i]);
                if a != b {
                    return a.partial_cmp(&b);
                }
            }
            return Some(Ordering::Equal)
        } else {
            self.str.partial_cmp(&other.str)
        }
    }
}

const NUM_CARDS:usize =13;
const CARD_VALUES: [char; NUM_CARDS] = ['2', '3', '4', '5', '6', '7', '8', '9', 'T', 'J', 'Q', 'K', 'A'];
const CARD_VALUES_2: [char; NUM_CARDS] = ['J','2', '3', '4', '5', '6', '7', '8', '9', 'T', 'Q', 'K', 'A'];

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
    let mut str_hands:Vec<
        (StrengthType,[usize;5],usize)> = Vec::new();
    let mut hands: Vec<[usize; 5]> = Vec::new();

    for l in lines
    {
        let (chand, cbid) = l.split_once(" ").unwrap();
        let bid: usize = cbid.parse().unwrap();
        let mut counts: [usize; NUM_CARDS] = [0; NUM_CARDS];
        let mut hand: [usize; 5] = [0, 0, 0, 0, 0];
        let char_array: Vec<_> = chand.chars().collect();
        for i in 0..HAND_SIZE {
            let c = char_array[i];
            let count_i: usize = char_to_index[&c];
            counts[count_i] += 1;
            hand[i] = count_i;

        }
        let hand_max = counts.iter().max().unwrap();
        let (score,str):(Score,StrengthType) = match hand_max {
            &5 => { (Score::FiveOfKind(hand[0]),StrengthType::FiveOfAKind) },
            &4 => {
                let mut k:usize = 0;
                for i in 0..counts.len() {
                    if counts[i] == 4 {
                        k = i;
                        break;
                    }
                }
                (Score::FourOfKind(k),StrengthType::FourOfAKind)
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
                    (Score::ThreeOfKind(three.unwrap()),StrengthType::ThreeOfAKind)
                } else {
                    (Score::FullHouse(three.unwrap(),two.unwrap() ), StrengthType::FullHouse)
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
                                let n_a = x.max(i);
                                let n_b = x.min(i);
                                (a,b) = (Some(n_a), Some(n_b));
                            break;
                            }
                            (Some(a), Some(b)) => { panic!("found a third pair! Pair({a}), Pair({b}), Pair({i} "); }
                            _ => {panic!("no pairs found with hand_max = {hand_max}, in hand: {:?}",hand); }
                        }
                    }
                }
                if b.is_none() {
                    (Score::OnePair(a.unwrap()),StrengthType::OnePair)
                } else {
                    (Score::TwoPair(a.unwrap(), b.unwrap()),StrengthType::TwoPair)
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
                (Score::HighCard(k),StrengthType::HighCard)
            }
            _ => { panic!    ("unable to score hand: {:?} with max {}", hand, hand_max);

            }
        };
        hands.push(hand);
        scored_hands.push((score,bid));
        str_hands.push((str, hand,bid));
    }
    str_hands.sort();
    let mut answer:usize = 0;
    for i in 1..=str_hands.len() {
        let (_,_,bid) = str_hands[i-1];
         answer += bid * i;
    }
    return answer.to_string()
}

fn part2(input_file: &str) -> String {
    let lines = file_to_lines(input_file);
    let mut char_to_index: HashMap<char, usize> = HashMap::with_capacity(CARD_VALUES_2.len());

    for i in 0..CARD_VALUES_2.len() {
        char_to_index.insert(CARD_VALUES_2[i], i);
    }
    char_to_index.shrink_to_fit();

    let char_to_index = char_to_index;
    let mut scored_hands: Vec<(Score, usize)> = Vec::new();
    let mut str_hands:Vec<
        (StrengthType,[usize;5],usize)> = Vec::new();
    let mut hands: Vec<[usize; 5]> = Vec::new();

    let joker_index =0;
    for l in lines
    {
        let (chand, cbid) = l.split_once(" ").unwrap();
        let bid: usize = cbid.parse().unwrap();
        let mut counts: [usize; NUM_CARDS] = [0; NUM_CARDS];
        let mut hand: [usize; 5] = [0, 0, 0, 0, 0];
        let char_array: Vec<_> = chand.chars().collect();
        for i in 0..HAND_SIZE {
            let c = char_array[i];
            let count_i: usize = char_to_index[&c];
            counts[count_i] += 1;
            hand[i] = count_i;

        }
        let hand_max = counts.iter().max().unwrap();
        let joker_count = counts[joker_index];
        let (score,str):(Score,StrengthType) = match hand_max {
            &5 => { (Score::FiveOfKind(hand[0]),StrengthType::FiveOfAKind) },
            &4 => {
                let mut k:usize = 0;
                for i in 0..counts.len() {
                    if counts[i] == 4 {
                        k = i;
                        break;
                    }
                }
                if joker_count == 0
                {
                    (Score::FourOfKind(k), StrengthType::FourOfAKind)
                } else {
                    (Score::FiveOfKind(k), StrengthType::FiveOfAKind)
                }
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
                    if joker_count == 0 {
                        (Score::ThreeOfKind(three.unwrap()), StrengthType::ThreeOfAKind)
                    } else {
                        if three.unwrap() != joker_index {
                            if joker_count == 1 {
                                (Score::FourOfKind(three.unwrap()),StrengthType::FourOfAKind)
                            } else {
                                (Score::FiveOfKind(three.unwrap()), StrengthType::FiveOfAKind)
                            }
                        }       else {
                            (Score::FourOfKind(three.unwrap()), StrengthType::FourOfAKind)
                        }
                    }

                } else {
                    let l3 = three.unwrap();
                    let l2 = two.unwrap();

                    if joker_count ==0 {
                        (Score::FullHouse(three.unwrap(), two.unwrap()), StrengthType::FullHouse)
                    } else if l2==joker_index {
                        (Score::FiveOfKind(l3), StrengthType::FiveOfAKind)
                    } else {
                        (Score::FiveOfKind(l2), StrengthType::FiveOfAKind)
                    }
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
                                let n_a = x.max(i);
                                let n_b = x.min(i);
                                (a,b) = (Some(n_a), Some(n_b));
                                break;
                            }
                            (Some(a), Some(b)) => {
                                panic!("found a third pair! Pair({a}), Pair({b}), Pair({i} "); }
                            _ => {panic!("no pairs found with hand_max = {hand_max}, in hand: {:?}",hand); }
                        }
                    }
                }

                if b.is_none() {
                    let ch = a.unwrap();
                    if joker_count == 0 {
                        (Score::OnePair(ch), StrengthType::OnePair)
                    } else {
                        (Score::ThreeOfKind(ch), StrengthType::ThreeOfAKind)
                    }
                } else {
                    let aa = a.unwrap();
                    let bb = b.unwrap();
                    if joker_count == 0 {
                        (Score::TwoPair(aa, bb), StrengthType::TwoPair)
                    } else {
                        if aa==joker_index {
                            (Score::FourOfKind(bb), StrengthType::FourOfAKind)
                        } else if bb==joker_index {
                            (Score::FourOfKind(aa), StrengthType::FourOfAKind)
                        } else {
                            let hi = aa.max(bb);
                            let lo = aa.min(bb);
                            (Score::FullHouse(hi, lo), StrengthType::FullHouse)
                        }
                    }
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
                if joker_count==0 {
                    (Score::HighCard(k), StrengthType::HighCard)
                } else {
                    (Score::OnePair(k), StrengthType::OnePair)
                }
            }
            _ => {panic!    ("unable to score hand: {:?} with max {}", hand, hand_max); }

        };

        hands.push(hand);
        scored_hands.push((score,bid));
        str_hands.push((str, hand,bid));
    }
    str_hands.sort();

    let mut answer:usize = 0;
    for i in 1..=str_hands.len() {
        let (_,_,bid) = str_hands[i-1];
        answer += bid * i;
    }


    return answer.to_string()
}
