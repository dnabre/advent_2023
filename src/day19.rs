use std::collections::HashMap;
use std::fmt::{Display, Formatter};
use std::time::Instant;

/*
    Advent of Code 2023: Day 19
        part1 answer:   386787
        part2 answer:   131029523269531


 */

const ANSWER: (&str, &str) = ("386787", "131029523269531");


fn main() {
    let _filename_test = "data/day19/test_input_01.txt";
    let _filename_test2 = "data/day19/test_input_02.txt";

    let filename_part1 = "data/day19/part1_input.txt";
    let filename_part2 = "data/day19/part2_input.txt";

    let start1 = Instant::now();
    let answer1 = part1(filename_part1);
    let duration1 = start1.elapsed();

    let start2 = Instant::now();
    let answer2 = part2(filename_part2);
    let duration2 = start2.elapsed();

    println!("Advent of Code, Day 19");
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

#[derive(Debug, Clone, Copy)]
#[derive(PartialEq)]
struct PartRange {
    x: (i64, i64),
    m: (i64, i64),
    a: (i64, i64),
    s: (i64, i64),
}

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
struct Part {
    x: i64,
    m: i64,
    a: i64,
    s: i64,
}

impl Display for Part {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "[x: {:5}, m: {:5}, a: {:5}, s: {:5}]", self.x, self.m, self.a, self.s)
    }
}

// px{a<2006:qkq,m>2090:A,rfg}
#[derive(Debug, PartialEq, Eq, Hash, Clone)]
struct Rule {
    start_queue: String,
    last_queue: String,
    compares: Vec<RulePart>,
}

impl Display for Rule {
    fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
        write!(f, " {}: {{ {} : {} }}", self.start_queue, advent_2023::list_displayables_to_string(&self.compares), self.last_queue)
    }
}

#[derive(Debug, PartialEq, Eq, Hash, Clone)]
struct RulePart {
    xmas_letter: char,
    op: char,
    value: i64,
    queue_name: String,
}

impl Display for RulePart {
    fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
        write!(f, "{} {} {} : {}", self.xmas_letter, self.op, self.value, self.queue_name)
    }
}


fn parse_line(input_line: &String) -> Rule {
    let (s_queue, rest) = input_line.split_once("{").unwrap();
    let r_parts: Vec<&str> = rest.split(",").collect();

    let mut n_compares: Vec<RulePart> = Vec::new();

    for i in 0..r_parts.len() - 1 {
        let rp = r_parts[i];
        let letter = rp.as_bytes()[0] as char;
        let comparison = rp.as_bytes()[1] as char;
        let num_c_queue = &rp[2..];
        let (num, n_queue) = num_c_queue.split_once(":").unwrap();
        let z: i64 = num.parse().unwrap();
        let new_rule_part = RulePart {
            xmas_letter: letter,
            op: comparison,
            value: z,
            queue_name: n_queue.to_string(),
        };
        n_compares.push(new_rule_part);
    }
    let rp: &str = *r_parts.last().unwrap();
    let n_last_queue = rp[0..rp.len() - 1].to_string();
    let new_rule = Rule {
        start_queue: s_queue.to_string(),
        last_queue: n_last_queue,
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


fn build_rule_map(rule_list: &Vec<Rule>) -> HashMap<&str, usize> {
    let mut rule_map: HashMap<&str, usize> = HashMap::new();
    for i in 0..rule_list.len() {
        rule_map.insert(rule_list[i].start_queue.as_str(), i);
    }
    return rule_map;
}

fn parse_everything(lines: &Vec<String>) -> (Vec<Rule>, Vec<Part>) {
    let mut rule_list: Vec<Rule> = Vec::new();
    let mut index: usize = 0;


    while lines[index] != "" {
        let new_rule: Rule = parse_line(&lines[index]);
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

    (rule_list, parts_list)
}

fn find_range(range: PartRange, current: &str, rule_list: &Vec<Rule>, rule_map: &HashMap<&str, usize>) -> i64 {
    if current == "R" {
        return 0;
    }
    if current == "A" {
        return (range.x.1 - range.x.0 + 1) *
            (range.m.1 - range.m.0 + 1) *
            (range.a.1 - range.a.0 + 1) *
            (range.s.1 - range.s.0 + 1);
    }
    let rule_idx = rule_map[current];
    let mut curr_range = Some(range);
    let mut total = 0;
    let rule = &rule_list[rule_idx];
    for rule_part in &rule_list[rule_idx].compares {
        if let Some(r) = curr_range {
            let (matching, not_matching) = split_range(r, rule_part);
            if let Some(m) = matching {
                total += find_range(m, rule_part.queue_name.as_str(), rule_list, rule_map);
            }
            curr_range = not_matching;
        }
    }
    if let Some(r) = curr_range {
        total += find_range(r, rule.last_queue.as_str(), rule_list, rule_map);
    }
    return total;
}

fn single_split((low, high): (i64, i64), xmas: char, op: char, value: i64, super_range: PartRange) -> (Option<PartRange>, Option<PartRange>) {
    if high < value {
        return (Some(super_range), None);
    }
    if low >= value {
        return (None, Some(super_range));
    }

    let (mut r1, mut r2) = (super_range, super_range);

    let (left, right) = if op == '<' {
        ((low, value - 1),
         (value, high))
    } else {
        ((value + 1, high),
         (low, value))
    };

    match xmas {
        'x' => {
            r1.x = left; r2.x = right;
        }
        'm' => {
            r1.m = left;r2.m = right;
        }
        'a' => {
            r1.a = left;r2.a = right;
        }
        's' => {
            r1.s = left;r2.s = right;
        }
        zz => { panic!("bad letter: {}", zz) }
    }
    return (Some(r1), Some(r2));
}

fn split_range(range: PartRange, rule_part:&RulePart) -> (Option<PartRange>, Option<PartRange>) {
    match rule_part.xmas_letter {
        'x' => { single_split(range.x, 'x', rule_part.op, rule_part.value, range) }
        'm' => { single_split(range.m, 'm', rule_part.op, rule_part.value, range) }
        'a' => { single_split(range.a, 'a', rule_part.op, rule_part.value, range) }
        's' => { single_split(range.s, 's', rule_part.op, rule_part.value, range) }
        zz => { panic!("bad letter: {}", zz) }
    }
}



fn part1(input_file: &str) -> String {
    let lines = advent_2023::file_to_lines(input_file);


    let (rule_list, parts_list) = parse_everything(&lines);
    let rule_map = build_rule_map(&rule_list);


    let mut accept_list: Vec<usize> = Vec::new();
    let mut reject_list: Vec<usize> = Vec::new();
    for part_idx in 0..parts_list.len() {
        let mut current_queue = "in";
        let part = parts_list[part_idx];
        while current_queue != "A" && current_queue != "R" {
            let rule_for_q = &rule_list[rule_map[current_queue]];
            let mut all_rules_passed = true;
            for r in &rule_for_q.compares {
                let v = match r.xmas_letter {
                    'x' => { part.x }
                    'm' => { part.m }
                    'a' => { part.a }
                    's' => { part.s }
                    ch => { panic!("being asked to compared against value of {}", ch) }
                };

                if ((r.op == '<') && (v < r.value)) || ((r.op == '>') && (v > r.value)) {
                    current_queue = r.queue_name.as_str();
                    all_rules_passed = false;
                    break;
                }
            }
            if all_rules_passed {
                current_queue = rule_for_q.last_queue.as_str();
            }
        }
        if current_queue == "A" {
            accept_list.push(part_idx);
        } else {
            reject_list.push(part_idx);
        }
    }

    let mut answer: i64 = 0;
    for a_idx in 0..accept_list.len() {
        let part = parts_list[accept_list[a_idx]];
        let part_total = part.x + part.m + part.a + part.s;
        answer += part_total;
    }


    return answer.to_string();
}


fn part2(input_file: &str) -> String {
    let lines = advent_2023::file_to_lines(input_file);

    let (rule_list, _) = parse_everything(&lines);
    let rule_map = build_rule_map(&rule_list);

    let answer = find_range(
        PartRange {
            x: (1, 4000),
            m: (1, 4000),
            a: (1, 4000),
            s: (1, 4000),
        },
        "in",
        &rule_list, &rule_map,
    );


    return answer.to_string();
}