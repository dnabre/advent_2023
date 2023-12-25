#![allow(unused_variables)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(dead_code)]
#![allow(unused_assignments)]
#![allow(unreachable_code)]


use std::collections::{BTreeSet, HashMap, HashSet, VecDeque};
use std::time::Instant;
use advent_2023::{parse_grid, print_grid};

/*
    Advent of Code 2023: Day 23
        part1 answer:
        part2 answer:


*/
const ANSWER: (&str, &str) = ("2502", "6726");

fn main() {
    let _filename_test = "data/day23/test_input_01.txt";
    let _filename_test2 = "data/day23/test_input_02.txt";

    let filename_part1 = "data/day23/part1_input.txt";
    let filename_part2 = "data/day23/part2_input.txt";

  //  println!("Advent of Code, Day 23");
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
    // let answer2 = part2(_filename_test2);
    // let duration2 = start2.elapsed();
    //
    // println!("\t Part 2: {:14} time: {:?}", answer2, duration2);
    // if ANSWER.1 != answer2 {
    //     println!("\t\t ERROR: Answer is WRONG. Got: {}, Expected {}", answer2, ANSWER.1);
    // }
    println!("    ---------------------------------------------");
}

#[derive(Debug, PartialOrd, Ord, PartialEq, Eq, Hash, Clone, Copy)]
struct Coord {
    row: usize,
    col: usize
}
const START_POINT:Coord=Coord{row: 0, col: 1};

#[derive(Debug, PartialEq, Eq, Hash, Clone)]
struct State {
    pos: Coord,
    distance: usize,
    visited_squares:BTreeSet<Coord>
}
 static CARD_DELTA: [(i32, i32); 4] = [ (-1, 0), (0, -1),(0, 1), (1, 0)];
static SLOPS: [char; 4] =  ['<','>','^', 'v'];

fn part1(input_file: &str) -> String {
    let lines = advent_2023::file_to_lines(input_file);
    let mut grid = parse_grid(&lines);

    let max_row = grid.len();
    let max_col = grid[0].len();
    let mut goal_point = Coord { row: max_row-1, col: max_col-1 };
    let mut ch = grid[goal_point.row][goal_point.col];
    while ch != '.' {
        goal_point.col -= 1;
        ch = grid[goal_point.row][goal_point.col];
    }
    println!("located goal @ {:?}", goal_point);


    print_grid(&grid);


    let start_state = State {
        pos: START_POINT,
        distance: 0,
        visited_squares:BTreeSet::from([START_POINT])
    };

    let mut work_stack:VecDeque<State> = VecDeque::new();
    work_stack.push_back(start_state);

    let mut records:HashMap<Coord, usize> = HashMap::new();

    let mut max_steps = usize::MIN;
    while let Some(current_state) = work_stack.pop_back() {
        println!("current_state: {:?}", current_state);
        let m_rec = records.get(&current_state.pos);
        if let Some(rec) = m_rec {
            if current_state.distance > *rec {
                records.insert(current_state.pos,current_state.distance );
            }
        } else {
            records.insert(current_state.pos, current_state.distance);
        }


        let (pr,pc) = (current_state.pos.row, current_state.pos.col);

        if current_state.pos == goal_point {
            let max_steps = max_steps.max(current_state.distance);
            println!("reached goal in {} steps",current_state.distance );
            continue;
        }
        let ch = grid[pr][pc];
        if ch == '#' {
            continue;
        }
        if SLOPS.contains(&ch) {
            let new_p = match ch {
                '^' => { (pr-1, pc)},
                'v' => { (pr+1, pc)},
                '<' => { (pr, pc -1)},
                '>' =>{ (pr, pc+1)},
                x => {panic!("bad slop {}", x)}
            };
            let new_c = Coord{ row: new_p.0, col: new_p.1 };
            let mut new_state = State{
                pos: new_c,
                distance: current_state.distance + 1,
                visited_squares: current_state.visited_squares,
            };
            new_state.visited_squares.insert(current_state.pos);
            work_stack.push_back(new_state);
            continue;
        }


        if pr > 0 {
            let new_pos = Coord{ row: pr-1, col: pc };
            if current_state.visited_squares.contains(&new_pos) {
                continue;
            }
            let mut new_state = State {
                pos: new_pos,
                distance: current_state.distance + 1,
                visited_squares: current_state.visited_squares.clone()
            };
            new_state.visited_squares.insert(current_state.pos);
            work_stack.push_back(new_state);
        }


        if pc > 0 {
            let new_pos = Coord{ row: pr, col: pc-1 };
            if current_state.visited_squares.contains(&new_pos) {
                continue;
            }
            let mut new_state = State {
                pos: new_pos,
                distance: current_state.distance + 1,
                visited_squares: current_state.visited_squares.clone()
            };
            new_state.visited_squares.insert(current_state.pos);
            work_stack.push_back(new_state);
        }


        if pr < max_row -1 {
            let new_pos = Coord{ row: pr+1, col: pc };
            if current_state.visited_squares.contains(&new_pos) {
                continue;
            }
            let mut new_state = State {
                pos: new_pos,
                distance: current_state.distance + 1,
                visited_squares: current_state.visited_squares.clone()
            };
            new_state.visited_squares.insert(current_state.pos);
            work_stack.push_back(new_state);
        }


        if pc < max_col - 1 {
            let new_pos = Coord{ row: pr+1, col: pc + 1 };
            if current_state.visited_squares.contains(&new_pos) {
                continue;
            }
            let mut new_state = State {
                pos: new_pos,
                distance: current_state.distance + 1,
                visited_squares: current_state.visited_squares.clone()
            };
            new_state.visited_squares.insert(current_state.pos);
            work_stack.push_back(new_state);
        }

    }
    for (pos,d) in records {
        grid[pos.row][pos.col] = 'O';
    }
    print_grid(&grid);






    return String::new();
}

fn part2(input_file: &str) -> String {
    let lines = advent_2023::file_to_lines(input_file);

    return String::new();
}