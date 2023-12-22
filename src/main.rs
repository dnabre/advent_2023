#![allow(unused_variables)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(dead_code)]
#![allow(unused_assignments)]
#![allow(unreachable_code)]


use std::collections::{HashSet, VecDeque};
use std::fmt::{Display, Formatter};
use std::path::Component::ParentDir;
use std::time::Instant;

use grid::Grid;

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

    //println!("Advent of Code, Day 21");
    println!("    ---------------------------------------------");
    let start1 = Instant::now();
    let answer1_op = part1(filename_part1, false);
    let answer1 = answer1_op.0.unwrap();
    let duration1 = start1.elapsed();

    println!("\t Part 1: {:14} time: {:?}", answer1, duration1);
    if ANSWER.0 != answer1 {
        println!("\t\t ERROR: Answer is WRONG. Got: {}, Expected {}", answer1, ANSWER.0);
    }


    let start2 = Instant::now();
    let answer2 = part2(filename_part2);
    let duration2 = start2.elapsed();

    println!("\t Part 2: {:14} time: {:?}", answer2, duration2);
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


#[derive(Debug, Clone, PartialEq)]
enum Cell {
    Rock,
    GardenPlot(Option<u32>),
}

impl Display for Cell {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Cell::Rock => { write!(f, " . ") }
            Cell::GardenPlot(n) => {
                match n {
                    None => { write!(f, " O ") }
                    Some(num) => { write!(f, "{:3}", *num) }
                }
            }
        }
    }
}

#[derive(Debug, PartialEq)]
struct Garden {
    start: (usize, usize),
    grid: Vec<Vec<Cell<>>>,
}


fn neighbors(
    loc: (usize, usize),
    nrows: usize,
    ncols: usize,
    diagonally: bool,
) -> Vec<(usize, usize)> {
    let (r, c) = loc;
    debug_assert!(r < nrows && c < ncols); // r and c are positive since they are unsigned.
    let mut res = vec![];
// Quite verbose but adding a signed integer (-1, 0, 1) to a `usize`
// by casting into different integers types did not feel great.
    let (r0, r1) = (r != 0, r + 1 != nrows);
    let (c0, c1) = (c != 0, c + 1 != ncols);
// SE
    if diagonally && r1 && c1 {
        res.push((r + 1, c + 1));
    }
// S
    if r1 {
        res.push((r + 1, c));
    }
// E
    if c1 {
        res.push((r, c + 1));
    }
// NE
    if diagonally && r0 && c1 {
        res.push((r - 1, c + 1));
    }
// SW
    if diagonally && r1 && c0 {
        res.push((r + 1, c - 1));
    }
// W
    if c0 {
        res.push((r, c - 1));
    }
// N
    if r0 {
        res.push((r - 1, c));
    }
// NW
    if diagonally && r0 && c0 {
        res.push((r - 1, c - 1));
    }
    res.sort();
    return res;
}

impl Garden {
    fn exact_steps(&self, steps: u32) -> u64 {
        let size = self.shape().0;
        // The square grid has 4 corner zones:
        // +-----+
        // |  ^  |
        // | / \ |
        // |<   >|
        // | \ / |
        // |  v  |
        // +-----+
        let strictly_in_corner = |r, c| {
            usize::min(size - 1 - r, r) + c < (size - 1) / 2 || usize::max(size - 1 - r, r) + c > 3 * (size - 1) / 2
        };
        let in_corner = |r, c| {
            usize::min(size - 1 - r, r) + c <= (size - 1) / 2 || usize::max(size - 1 - r, r) + c >= 3 * (size - 1) / 2
        };
        // Count the plots accessible with an even/odd number of steps for the non-infinite whole grid.
        let whole_even = self.grid.iter().flatten().filter(|cell| matches!(cell, Cell::GardenPlot(Some(dist)) if *dist % 2 == 0)).count() as u64;
        let whole_odd = self.grid.iter().flatten().filter(|cell| matches!(cell, Cell::GardenPlot(Some(dist)) if *dist % 2 == 1)).count() as u64;
        // The whole center square (3x3 with x) is repeated 1 (center) + 4 * 2k for k in 1..
        // Then another whole square (3x3 with o) is repeated 4 * (2k-1) for k in 1..
        // On the exterior of the diamond, (upper) squares are truncated.
        //           O
        //          OOO
        //         XOOOX
        //        OOxxxOO
        //       OOOxxxOOO
        //      XOOOxxxOOOX
        //     OOxxxoooxxxOO
        //    OOOxxxoooxxxOOO
        //   XOOOxxxoooxxxOOOX
        //  OOxxxoooxxxoooxxxOO
        // OOOxxxoooxSxoooxxxOOO
        //  OOxxxoooxxxoooxxxOO
        //   XOOOxxxoooxxxOOOX
        //    OOOxxxoooxxxOOO
        //     OOxxxoooxxxOO
        //      XOOOxxxOOOX
        //       OOOxxxOOO
        //        OOxxxOO
        //         XOOOX
        //          OOO
        //           O
        let (center, other) = if steps % 2 == 0 {
            (whole_even, whole_odd)
        } else {
            (whole_odd, whole_even)
        };
        let middle = self.start.0;
        let q = (steps - middle as u32) / size as u32;
        let r = (steps - middle as u32) % size as u32;
        assert_eq!(r, 0);
        center * (1 + 4 * (2..).step_by(2).take_while(|k| *k < q).map(u64::from).sum::<u64>()) + other * 4 * (1..).step_by(2).take_while(|k| *k < q).map(u64::from).sum::<u64>() + self.grid.iter().enumerate().flat_map(|(r, col)| col.iter().enumerate().map(move |(c, cell)| ((r, c), cell))).map(|((r, c), cell)| {
            if let Cell::GardenPlot(Some(dist)) = cell {
                if dist % 2 == q % 2 {
                    if in_corner(r, c) {
                        // X
                        u64::from(q)
                    } else {
                        0
                    }
                } else {
                    //  OO        O
                    // OOO s and OOO s
                    // OOO       OOO
                    if strictly_in_corner(r, c) {
                        3 * u64::from(q - 1) + 2
                    } else {
                        4 * u64::from(q - 1) + 4
                    }
                }
            } else {
                0
            }
        }).sum::<u64>()
    }
    fn read_garden(input_file: &str) -> Garden {
        let lines = advent_2023::file_to_lines(input_file);
        let c_grid = advent_2023::parse_grid(&lines);
        let mut cell_grid: Vec<Vec<Cell>> = Vec::new();
        let mut start = None;
        for y in 0..c_grid.len() {
            let mut row_vec = Vec::new();
            for x in 0..c_grid[0].len() {
                let ch = c_grid[y][x];
                let g = match ch {
                    '#' => Cell::Rock,
                    '.' => Cell::GardenPlot(None),
                    'S' => {
                        start = Some((y, x));
                        Cell::GardenPlot(None)
                    }
                    xx => panic!("Wrong char: {}", xx),
                };
                row_vec.push(g);
            }
            cell_grid.push(row_vec);
        }
        Garden {
            start: start.unwrap(),
            grid: cell_grid,
        }
    }

    fn exact_steps_no_infinite(&self, steps: u32) -> u64 {
        self.grid.iter().flatten().filter(|cell| matches!(cell, Cell::GardenPlot(Some(dist)) if *dist <= steps && *dist % 2 == steps % 2)).count() as u64
    }


    fn shape(&self) -> (usize, usize) {
        let nrows = self.grid.len();
        let ncols = self.grid[0].len();
        return (nrows, ncols);
    }

    fn read_distances_o(&mut self) {
        let (nrows, ncols) = self.shape();
        let mut queue = VecDeque::from([(0, self.start)]);
        while let Some((dist, (r, c))) = queue.pop_front() {
            if let Cell::GardenPlot(rc_dist @ None) = &mut self.grid[r][c] {
                *rc_dist = Some(dist);
                for (r0, c0) in neighbors((r, c), nrows, ncols, false) {
                    if matches!(self.grid[r0][c0], Cell::GardenPlot(None)) {
                        queue.push_back((dist + 1, (r0, c0)));
                    }
                }
            }
        }
    }


    fn read_distances_m(&mut self) {
        let (nrows, ncols) = self.shape();
        let mut queue: VecDeque<(u32, (usize, usize))> = VecDeque::from([(0, self.start)]);

        while let Some((dist, (r, c))) = queue.pop_front() {
            if let Cell::GardenPlot(rc_dist @ None) = &mut self.grid[r][c] {
                *rc_dist = Some(dist);

                let his_neigh = neighbors((r, c), nrows, ncols, false);
                let my_neigh = advent_2023::checked_neighbor_points((r, c), nrows, ncols, false);

                assert_eq!(his_neigh, my_neigh);


                for (r0, c0) in advent_2023::checked_neighbor_points((c, r), nrows, ncols, false) {
                    if matches!(self.grid[r0][c0], Cell::GardenPlot(None)) {
                        queue.push_back((dist + 1, (r0, c0)));
                    }
                }
            }
        }
    }
}


fn mod_with_neg(k: i64, m: i64) -> i64 {
    if k >= 0 {
        k % m
    } else {
        (k % m) + m
    }
}

impl Coord {
    fn clear_at(&self, grid: &Vec<Vec<char>>) -> bool {
        let ch = grid[self.y as usize][self.x as usize];
        return ch != BLOCK_SQUARE;
    }
    fn clear_at2(&self, grid: &Vec<Vec<char>>) -> bool {
        let tx = mod_with_neg(self.x, 131) as usize;
        let ty = mod_with_neg(self.y, 131) as usize;
        if (tx >= 131) || (ty >= 131) {
            panic!("modded out of map (tx,ty) = ({tx},{ty})");
        }


        let ch = grid[ty][tx];
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


fn part1(input_file: &str, from_2: bool) -> (Option<String>, Option<HashSet<Coord>>) {
    let lines = advent_2023::file_to_lines(input_file);

    let mut grid = advent_2023::parse_grid(&lines);


    let target_steps = 64_u64;
    let start = find_start(&mut grid);


    let target_spots = search_upto_steps(&mut grid, start, target_steps);
    let answer1 = target_spots.len();
    if from_2 {
        return (None, Some(target_spots));
    } else {
        return (Some(answer1.to_string()), None);
    }
}

fn count_locations(grid: &Grid<i32>) -> usize {
    grid.iter().filter(|x| **x > 0).count()
}

fn v_count_locations(grid: &Vec<Vec<i32>>) -> usize {
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

fn from_grid(grid: &Grid<i32>) -> Vec<Vec<i32>> {
    let mut ng: Vec<Vec<i32>> = Vec::new();
    let dim = grid.cols();

    for _ in 0..dim {
        ng.push(vec![0; dim]);
    }
    for y in 0..dim {
        for x in 0..dim {
            ng[y][x] = grid[(y, x)];
        }
    }
    return ng;
}


fn count_block(grid: &Grid<i32>, is_odd: bool) -> usize {
    grid.indexed_iter().filter(|((y, x), v)| **v != -2 && (y + x) % 2 == usize::from(is_odd)).count()
}

fn read(text: &str) -> Grid<i32> {
    text.lines().map(|line| {
        line.chars().map(|x| match x {
            '#' => -2,
            '.' => -1,
            'S' => 0,
            _ => panic!("Unknown char {x}"),
        }).collect()
    }).collect::<Vec<Vec<_>>>().into()
}


fn parse_grid(lines: &Vec<String>) -> Vec<Vec<i32>> {
    let mut grid: Vec<Vec<char>> = Vec::new();
    for l in lines {
        let line = str_to_char_vec(l);
        grid.push(line);
    }
    let n_grid = advent_2023::convert_grid_using(&grid, |ch| char_to_i(ch));
    return n_grid;
}

fn char_to_i(x: char) -> i32 {
    match x {
        '#' => -2,
        '.' => -1,
        'S' => 0,
        _ => panic!("Unknown char {x}"),
    }
}


fn v_count_block(grid: &Vec<Vec<i32>>, is_odd: bool) -> usize {
    let mut count: usize = 0;
    for y in 0..grid.len() {
        for x in 0..grid[0].len() {
            let v = grid[y][x];
            if v != -2 && (y + x) % 2 == usize::from(is_odd) {
                count += 1;
            }
        }
    }
    return count;
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
            //     grid[pos.y][pos.x] = 'O';
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

fn expand(grid: &Grid<i32>) -> Grid<i32> {
    let mut new_grid = Grid::new(grid.rows() * 3, grid.cols() * 3);
    for i in 0..3 {
        for j in 0..3 {
            for ((y, x), v) in grid.indexed_iter() {
                let new_val = if *v == 0 && (i != 1 || j != 1) {
                    -1
                } else {
                    *v
                };
                new_grid[(y + i * grid.rows(), x + j * grid.cols())] = new_val;
            }
        }
    }
    new_grid
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

fn steps(grid: Grid<i32>, dist: usize) -> Grid<i32> {
    let mut grid = grid;
    for i in 0..i32::try_from(dist).unwrap()
    {

        let mut new_grid = grid.clone();
        for y in 0..grid.rows() {
            for x in 0..grid.cols() {
                if grid[(y, x)] == i {
                    for (dy, dx) in &[(0, 1), (0, -1), (1, 0), (-1, 0)] {
                        let ty = i32::try_from(y).unwrap() + dy;
                        let tx = i32::try_from(x).unwrap() + dx;

                        if let Some(v) = grid.get(ty, tx) {
                            if *v >= -1 {
                                *new_grid.get_mut(ty, tx).unwrap() = i + 1;
                                new_grid[(y, x)] = -1;
                            }
                        }
                    }
                }
            }
        }
        grid = new_grid;
    }
    grid
}

fn compute2(text: &str) -> usize {
    let grid = read(text);
    count_block(&grid, true)
}

fn v_compute2(lines: &Vec<String>) -> usize {
    let v_grid = parse_grid(&lines);
    return v_count_block(&v_grid, true);
}

fn print_grid(grid: &Grid<i32>) {
    for row in grid.iter_rows() {
        for x in row {
            let c = match x {
                -2 => '#',
                -1 => '.',
                0 => 'S',
                1.. => 'O',
                _ => panic!("Unknown char {x}"),
            };
            print!("{c}");
        }
        println!();
    }
}

fn print_vgrid(grid: &Vec<Vec<i32>>) {
    for y in 0..grid.len() {
        for x in 0..grid[0].len() {
            let c = match grid[y][x] {
                -2 => '#',
                -1 => '.',
                0 => 'S',
                1.. => 'O',
                _ => panic!("Unknown char {x}"),
            };
            print!("{c}");
        }
        println!();
    }
}

fn v_grid_expand(v_grid: &Vec<Vec<i32>>) -> Vec<Vec<i32>> {
    let dim = v_grid.len();
    let new_dim = dim * 3;
    let mut new_grid = Vec::new();
    for _ in 0..new_dim {
        new_grid.push(vec![0; new_dim]);
    }
    for i in 0..3 {
        for j in 0..3 {
            let v_index = v_grid_to_index(&v_grid);
            for ((y,x), v) in v_index {
                   let new_val = if *v == 0 && ( i != 1 || j != 1) {
                       -1
                   }     else {
                       *v
                   };
                    new_grid[y+i*dim][ x + j * dim] = new_val;
                }
            }
        }
    return new_grid;
}

fn v_grid_to_index(v_grid: &Vec<Vec<i32>>) -> Vec<((usize, usize), &i32)> {
    let mut fake_index_output = Vec::new();
    for y in 0..v_grid.len() {
        for x in 0..v_grid[0].len() {
            let ch = v_grid[y][x];
            let e = ((y, x), &v_grid[y][x]);
            fake_index_output.push(e);
        }
    }
    return fake_index_output;
}

fn part2(input_file: &str) -> String {
    let lines = advent_2023::file_to_lines(input_file);

    let v_grid = parse_grid(&lines);

    let text = std::fs::read_to_string(input_file).unwrap();
    let grid = read(&text);
    let grid = steps(grid, 65);


    let v_grid = v_steps(v_grid,   65);





    let result = compute2(&text);
    let v_result = v_compute2(&lines);
    println!("Full block (odd steps) = {result}\t {v_result} =->{} ", result == v_result);
    let val = 26_501_365;
    println!("Steps = {val}");

    let grid = read(&text);
    let v_grid = parse_grid(&lines);
    println!("parse compare: {}", compare_v_grid_to_grid(&v_grid, &grid));

    let grid = steps(grid, 65);

    let v_grid = v_steps(v_grid ,65);
    println!("steps compare: {}", compare_v_grid_to_grid(&v_grid, &grid));
    let y_0 = count_locations(&grid);
    let v_y_0 = v_count_locations(&v_grid);
    println!("131*0 = 65 = {y_0}  \t {v_y_0}");

    let grid = read(&text);
    let grid = expand(&grid);
    let v_grid = parse_grid(&lines);
    let v_grid = v_grid_expand(&v_grid);
    println!("compare: {}", compare_v_grid_to_grid(&v_grid, &grid));


    let grid = steps(grid, 65 + 131);
    let v_grid = v_steps(v_grid, 65 + 131);
    println!("compare: {}", compare_v_grid_to_grid(&v_grid, &grid));

    let y_1 = count_locations(&grid);
    let v_y_1 = v_count_locations(&v_grid);
    println!("131*1 + 65 = {y_1} \t {v_y_1}");

    let grid = read(&text);
    let grid = expand(&grid);

    let v_grid = parse_grid(&lines);
    let v_grid = v_grid_expand(&v_grid);
    println!("expand 1, compare: {}", compare_v_grid_to_grid(&v_grid, &grid));

    let grid = expand(&grid);
    let v_grid = v_grid_expand(&v_grid);
    println!("expand 2, compare: {}", compare_v_grid_to_grid(&v_grid, &grid));
    let grid = steps(grid, 65 + 131 * 2);

    let v_grid = v_steps(v_grid, 65 + 131 * 2);


    println!("steps compare: {}", compare_v_grid_to_grid(&v_grid, &grid));
      let y_2 = count_locations(&grid);
    let v_y_2 = v_count_locations(&v_grid);
    println!("131*2 + 65 = {y_2} \t {v_y_2}");


    let a2 = y_2 - 2 * y_1 + y_0;
    let b2 = 4 * y_1 - 3 * y_0 - y_2;
    let c = y_0;

    println!("{a2}/2 x^2 +{b2}/2 x + {c} = y");
    println!("x=0, y={c}");
    println!("x=1, y={}", (a2 + b2) / 2 + c);
    println!("x=2, y={}", (4 * a2 + 2 * b2) / 2 + c);
    println!(
        "x=202300, y={}",
        (202_300 * 202_300 * a2 + 202_300 * b2) / 2 + c
    );

    println!();
    let a2 = v_y_2 - 2 * v_y_1 + v_y_0;
    let b2 = 4 * v_y_1 - 3 * v_y_0 - v_y_2;
    let c = v_y_0;

    println!("{a2}/2 x^2 +{b2}/2 x + {c} = y");
    println!("x=0, y={c}");
    println!("x=1, y={}", (a2 + b2) / 2 + c);
    println!("x=2, y={}", (4 * a2 + 2 * b2) / 2 + c);
    println!(
        "x=202300, y={}",
        (202_300 * 202_300 * a2 + 202_300 * b2) / 2 + c
    );

let answer = (202_300 * 202_300 * a2 + 202_300 * b2) / 2 + c;






    return answer.to_string();
}

fn v_steps(grid:  Vec<Vec<i32>>, dist: usize) ->  Vec<Vec<i32>> {
   let dim = grid.len();

    let mut grid = grid;
    for i in 0..i32::try_from(dist).unwrap()
    {
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
                        let tx = x-1;
                        let v = grid[ty][tx];
                        if v >= -1 {
                            new_grid[ty][tx] = i + 1;
                            new_grid[y][x] = -1;
                        }
                    }

                    if y+1 < dim {
                        let ty = y + 1;
                        let tx = x;
                        let v = grid[ty][tx];
                        if v >= -1 {
                            new_grid[ty][tx] = i + 1;
                            new_grid[y][x] = -1;
                        }
                    }

                    if y > 0 {
                        let ty = y-1;
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


fn compare_v_grid_to_grid(v_g: &Vec<Vec<i32>>, g: &Grid<i32>) -> bool {
    let dim = v_g.len();
    let dim2 = g.cols();
    if dim != dim2 { return false; };
    for y in 0..dim {
        for x in 0..dim {
            if v_g[y][x] != g[(y, x)] {
                return false;
            }
        }
    }
    return true;
}

fn compare_w_count_v_grid_to_grid(v_g: &Vec<Vec<i32>>, g: &Grid<i32>) -> usize {
    let dim = v_g.len();
    let dim2 = g.rows();
    if dim != dim2 {
        println!("compare v_grid to grid, dims don't match: v {} vs {}", v_g.len(), g.rows());

                 return 0;
    };


    let mut diff_count:usize = 0;
    for y in 0..dim {
        for x in 0..dim {
            if v_g[y][x] != g[(y, x)] {
                diff_count +=1;
            }
        }
    }
    return diff_count;
}