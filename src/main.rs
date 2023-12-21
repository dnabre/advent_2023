#![allow(unused_variables)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(dead_code)]
#![allow(unused_assignments)]
#![allow(unreachable_code)]

use std::collections::{HashSet, VecDeque};
use std::fmt::{Display, Formatter};
use std::mem::needs_drop;
use std::time::Instant;


/*
    Advent of Code 2023: Day 21
        part1 answer:   3642
        part2 answer:


 */

const ANSWER: (&str, &str) = ("3642", "131029523269531");


fn main() {
    let _filename_test = "data/day21/test_input_01.txt";
    let _filename_test2 = "data/day21/test_input_02.txt";

    let filename_part1 = "data/day21/part1_input.txt";
    let filename_part2 = "data/day21/part2_input.txt";

    let start1 = Instant::now();
    let answer1 = part1(filename_part1);
    let duration1 = start1.elapsed();

    let start2 = Instant::now();
    let answer2 = part2(_filename_test2);
    let duration2 = start2.elapsed();

    //println!("Advent of Code, Day 21");
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

const BLOCK_SQUARE:char = '#';

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
struct Coord {
     x: usize   ,
     y: usize
}

impl Coord {
     fn clear_at(&self, grid: &Vec<Vec<char>>) -> bool {
        let ch = grid[self.y][self.x];
         return ch != BLOCK_SQUARE;
    }

   fn offset(&self, dx:i32, dy:i32, max_x:usize, max_y:usize) -> Option<Coord>{
       let (cx,cy) = (self.x,self.y);
       let (nx, ny) = (dx + cx as i32, dy + cy as i32);
       if (nx < 0) || (ny < 0) || (nx as usize  >= max_x ) || ( ny as usize >= max_y) {
           return None;
       }
       let r = Coord {x: nx as usize, y: ny as usize};
       return Some(r);
    }
}



fn part1(input_file: &str) -> String {
    let lines = advent_2023::file_to_lines(input_file);

    let mut grid = advent_2023::parse_grid(&lines);

    let mut start = Coord{
        x: 0,
        y: 0,
    };

    let max_y = lines.len();
    let max_x = lines[0].len();
   'y_loop: for y in 0..max_y {
        for x in 0..max_x {
            let ch = grid[y][x];
            if ch == 'S' {
                (start.x, start.y) = (x,y);
                break 'y_loop;
            }
        }
    }
    println!("start@ {:?}", start);
    grid[start.y][start.x] = '.';


    let mut queue:VecDeque<(Coord,u32)> = VecDeque::new();
    queue.push_front((start,0));

    let target_steps = 64;
    let mut target_points:HashSet<Coord> = HashSet::new();
    let mut visited :HashSet<(Coord,u32)>= HashSet::new();

    while let Some((pos,step_count)) = queue.pop_front(){
        if step_count == target_steps {
            target_points.insert(pos);
            grid[pos.y][pos.x] = 'O';
        } else {
            for (i_x, i_y) in advent_2023::CARD_DELTA {
                let n_coord = pos.offset(i_x, i_y, max_x, max_y);
                if let Some(n_coord) = n_coord {
                    let pair = (n_coord,step_count+1);
                    if n_coord.clear_at(&grid) && !visited.contains(&(pair)){
                        queue.push_front((n_coord, step_count + 1));
                        visited.insert(pair);
                    }
                }
            }
        }

    }

  //  advent_2023::print_grid(&grid);


    let answer = target_points.len();
    return answer.to_string();
}

fn part2(input_file: &str) -> String {
    let lines = advent_2023::file_to_lines(input_file);

    let answer =0;
    return answer.to_string();
}