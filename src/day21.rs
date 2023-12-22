use std::collections::{HashSet, VecDeque};
use std::time::Instant;

use advent_2023::str_to_char_vec;

/*
    Advent of Code 2023: Day 21
        part1 answer:   3642
        part2 answer:   608603023105276

*/

const ANSWER: (&str, &str) = ("3642", "608603023105276");

fn main() {
    let _filename_test = "data/day21/test_input_01.txt";
    let _filename_test2 = "data/day21/test_input_02.txt";

    let filename_part1 = "data/day21/part1_input.txt";
    let filename_part2 = "data/day21/part2_input.txt";

    println!("Advent of Code, Day 21");
    println!("    ---------------------------------------------");
    let start1 = Instant::now();
    let answer1 = part1(filename_part1);
    let duration1 = start1.elapsed();

    println!("\t Part 1: {:15} time: {:?}", answer1, duration1);
    if ANSWER.0 != answer1 {
        println!("\t\t ERROR: Answer is WRONG. Got: {}, Expected {}", answer1, ANSWER.0);
    }

    let start2 = Instant::now();
    let answer2 = part2(filename_part2);
    let duration2 = start2.elapsed();

    println!("\t Part 2: {:15} time: {:?}", answer2, duration2);
    if ANSWER.1 != answer2 {
        println!("\t\t ERROR: Answer is WRONG. Got: {}, Expected {}", answer2, ANSWER.1);
    }
    println!("    ---------------------------------------------");
}


const BLOCK_SQUARE: char = '#';
pub static CARD_DELTA: [(i64, i64); 4] = [(-1, 0), (1, 0), (0, -1), (0, 1)];

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
struct Coord {
    x: i64,
    y: i64,
}

impl Coord {
    fn clear_at(&self, grid: &Vec<Vec<char>>) -> bool {
        let ch = grid[self.y as usize][self.x as usize];
        return ch != BLOCK_SQUARE;
    }
    fn offset(&self, dx: i64, dy: i64, max_x: usize, max_y: usize) -> Option<Coord> {
        let (cx, cy) = (self.x, self.y);
        let (nx, ny) = (dx + cx as i64, dy + cy as i64);
        if (nx < 0) || (ny < 0) || (nx as usize >= max_x) || (ny as usize >= max_y) {
            return None;
        }
        let r = Coord { x: nx, y: ny };
        return Some(r);
    }
}

fn count_locations(grid: &Vec<Vec<i32>>) -> usize {
    let mut count = 0;
    for y in 0..grid.len() {
        for x in 0..grid[0].len() {
            if grid[y][x] > 0 {
                count += 1;
            }
        }
    }
    return count;
}

fn parse_grid(lines: &Vec<String>) -> Vec<Vec<i32>> {
    let mut grid: Vec<Vec<char>> = Vec::new();
    for l in lines {
        let line = str_to_char_vec(l);
        grid.push(line);
    }
    let n_grid = advent_2023::convert_grid_using(&grid, |ch|
        match ch {
            '#' => -2,
            '.' => -1,
            'S' => 0,
            _ => panic!("Unknown char {ch}"),
        },
    );
    return n_grid;
}


fn search_upto_steps(grid: &Vec<Vec<char>>, start: Coord, target_steps: u64) -> HashSet<Coord> {
    let max_y = grid.len();
    let max_x = grid[0].len();

    let mut queue: VecDeque<(Coord, u64)> = VecDeque::new();
    queue.push_front((start, 0));

    let mut target_points: HashSet<Coord> = HashSet::new();
    let mut visited: HashSet<(Coord, u64)> = HashSet::new();

    while let Some((pos, step_count)) = queue.pop_front() {
        if step_count == target_steps {
            target_points.insert(pos);
        } else {
            for (i_x, i_y) in CARD_DELTA {
                let n_coord = pos.offset(i_x, i_y, max_x, max_y);
                if let Some(n_coord) = n_coord {
                    let pair = (n_coord, step_count + 1);
                    if n_coord.clear_at(&grid) && !visited.contains(&(pair)) {
                        queue.push_front((n_coord, step_count + 1));
                        visited.insert(pair);
                    }
                }
            }
        }
    }
    return target_points;
}


fn find_start(grid: &mut Vec<Vec<char>>) -> Coord {
    let max_y: usize = grid.len();
    let max_x: usize = grid[0].len();
    let mut start = Coord {
        x: 0,
        y: 0,
    };
    for y in 0..max_y {
        for x in 0..max_x {
            let ch = grid[y][x];
            if ch == 'S' {
                (start.x, start.y) = (x as i64, y as i64);
                grid[y][x] = '.';
                return start;
            }
        }
    }
    return start;
}

fn grid_expand(v_grid: &Vec<Vec<i32>>) -> Vec<Vec<i32>> {
    let dim = v_grid.len();
    let new_dim = dim * 3;
    let mut new_grid = Vec::new();
    for _ in 0..new_dim {
        new_grid.push(vec![0; new_dim]);
    }
    for i in 0..3 {
        for j in 0..3 {
            let v_index = grid_to_index(&v_grid);
            for ((y, x), v) in v_index {
                let new_val = if *v == 0 && (i != 1 || j != 1) {
                    -1
                } else {
                    *v
                };
                new_grid[y + i * dim][x + j * dim] = new_val;
            }
        }
    }
    return new_grid;
}

fn grid_to_index(v_grid: &Vec<Vec<i32>>) -> Vec<((usize, usize), &i32)> {
    let mut enum_values = Vec::new();
    for y in 0..v_grid.len() {
        for x in 0..v_grid[0].len() {
            let e = ((y, x), &v_grid[y][x]);
            enum_values.push(e);
        }
    }
    return enum_values;
}

fn steps(grid: Vec<Vec<i32>>, dist: usize) -> Vec<Vec<i32>> {
    let dim = grid.len();
    let mut grid = grid;
    for i in 0..i32::try_from(dist).unwrap() {
        let mut new_grid = grid.clone();
        for y in 0..dim {
            for x in 0..dim {
                if grid[y][x] == i {
                    if x + 1 < dim {
                        let ty = y;
                        let tx = x + 1;
                        let v = grid[ty][tx];
                        if v >= -1 {
                            new_grid[ty][tx] = i + 1;
                            new_grid[y][x] = -1;
                        }
                    }
                    if x > 0 {
                        let ty = y;
                        let tx = x - 1;
                        let v = grid[ty][tx];
                        if v >= -1 {
                            new_grid[ty][tx] = i + 1;
                            new_grid[y][x] = -1;
                        }
                    }
                    if y + 1 < dim {
                        let ty = y + 1;
                        let tx = x;
                        let v = grid[ty][tx];
                        if v >= -1 {
                            new_grid[ty][tx] = i + 1;
                            new_grid[y][x] = -1;
                        }
                    }
                    if y > 0 {
                        let ty = y - 1;
                        let tx = x;
                        let v = grid[ty][tx];
                        if v >= -1 {
                            new_grid[ty][tx] = i + 1;
                            new_grid[y][x] = -1;
                        }
                    }
                }
            }
        }
        grid = new_grid;
    }
    return grid;
}

fn part1(input_file: &str) -> String {
    let lines = advent_2023::file_to_lines(input_file);

    let mut grid = advent_2023::parse_grid(&lines);
    let target_steps = 64_u64;

    let start = find_start(&mut grid);
    let target_spots = search_upto_steps(&mut grid, start, target_steps);
    let answer = target_spots.len();
    return answer.to_string();
}

fn part2(input_file: &str) -> String {
    let lines = advent_2023::file_to_lines(input_file);

    let v_grid = parse_grid(&lines);
    let v_grid = steps(v_grid, 65);
    let v_y_0 = count_locations(&v_grid);

    let v_grid = parse_grid(&lines);
    let v_grid = grid_expand(&v_grid);
    let v_grid = steps(v_grid, 65 + 131);
    let v_y_1 = count_locations(&v_grid);

    let v_grid = parse_grid(&lines);
    let v_grid = grid_expand(&v_grid);
    let v_grid = grid_expand(&v_grid);
    let v_grid = steps(v_grid, 65 + 131 * 2);
    let v_y_2 = count_locations(&v_grid);

    let a2 = v_y_2 - 2 * v_y_1 + v_y_0;
    let b2 = 4 * v_y_1 - 3 * v_y_0 - v_y_2;
    let c = v_y_0;

    let answer = (202_300 * 202_300 * a2 + 202_300 * b2) / 2 + c;
    return answer.to_string();
}
