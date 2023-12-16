/*
    Advent of Code 2023: Day 14
        part1 answer:   110779
        part2 answer:   86069

 */


use std::time::Instant;

const ANSWER: (&str, &str) = ("110779", "86069");

fn main() {
    let _filename_test1 = "data/day14/test_input_01.txt";
    let _filename_test2 = "data/day14/test_input_02.txt";

    let filename_part1 = "data/day14/part1_input.txt";
    let filename_part2 = "data/day14/part2_input.txt";

    let start1 = Instant::now();
    let answer1 = part1(filename_part1);
    let duration1 = start1.elapsed();

    let start2 = Instant::now();
    let answer2 = part2(filename_part2);
    let duration2 = start2.elapsed();

    println!("Advent of Code, Day 14");
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
    let lines = advent_2023::file_to_lines(input_file);

    let mut grid: Vec<Vec<char>> = Vec::new();
    for i in 0..lines.len() {
        let a = advent_2023::str_to_char_vec(&lines[i]);
        grid.push(a);
    }

    //north tilt
    tilt_north(&mut grid);

    let answer = total_load(&grid);

    return answer.to_string();
}


fn part2(input_file: &str) -> String {
    let lines = advent_2023::file_to_lines(input_file);

    let mut grid: Vec<Vec<char>> = Vec::new();
    for i in 0..lines.len() {
        let a = advent_2023::str_to_char_vec(&lines[i]);
        grid.push(a);
    }


    const NUM_CYCLES:usize =1000;
    for _ in 0..NUM_CYCLES {
        tilt_north(&mut grid);        // North Tilt
        grid = rotate_clockwise(&grid);         // North->West
        tilt_north(&mut grid);       // West Tilt
        grid = rotate_clockwise(&grid);         // West->South
        tilt_north(&mut grid);        // South Tilt
        grid = rotate_clockwise(&grid);         // South->East
        tilt_north(&mut grid);        // East Tile
        grid = rotate_clockwise(&grid);         // East->North
    }

    let answer = total_load(&grid);

    return answer.to_string();
}

fn rotate_clockwise(grid: & Vec<Vec<char>>) -> Vec<Vec<char>> {
    let mut new_grid: Vec<Vec<char>> = Vec::new();
    for _ in 0..grid.len() {
        new_grid.push(Vec::new());
    }
    for x in 0..grid[0].len() {
        for y in (0..grid.len()).rev() {
            let ch = grid[y][x];
            new_grid[x].push(ch);
        }
    }
    return new_grid;
}


fn tilt_north(grid: &mut Vec<Vec<char>>) {
            for y in 0..grid.len() {
                for x in 0..grid[0].len() {
                    let ch = grid[y][x];
                    if ch != 'O' || y == 0 {
                        continue;
                    }
                    let mut o_y = y;
                    while o_y > 0 {
                        let ch = grid[o_y - 1][x];
                        if ch == '.' {
                            o_y -= 1;
                        } else {
                            // hit something, so must stop
                            break;
                        }
                    }
                    if o_y != y {
                        grid[o_y][x] = 'O';
                        grid[y][x] = '.';
                    }
                }
            }
        }

fn total_load(grid: &Vec<Vec<char>>) -> usize {
    let max_rows = grid.len();
    let mult = grid.len();
    let mut total_load: usize = 0;
    for r in (1..=mult).rev() {
        let mut rocks_in_row = 0;
        for x in 0..grid[0].len() {
            if grid[max_rows - r][x] == 'O' {
                rocks_in_row += 1;
            }
        }
        total_load += rocks_in_row * r;
    }
    total_load
}

fn _print_grid(grid: &Vec<Vec<char>>) -> () {
    let mut r = 10;
    for y in 0..grid.len() {
        for x in 0..grid[0].len() {
            let ch = grid[y][x];
            print!("{ch}");
        }
        println!("\t {r}");
        r -= 1;
    }
    println!("----------------------------------------------");
}



