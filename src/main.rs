#![allow(unused_variables)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(dead_code)]
#![allow(unused_assignments)]
#![allow(unreachable_code)]
#![allow(unused_labels)]

/*
    Advent of Code 2023: Day 14
        part1 answer:   110779
        part2 answer:   86069

 */


use std::arch::x86_64::_mm_sha1msg1_epu32;
use std::ptr::eq;
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
   for n in 0..NUM_CYCLES
{

     tilt_north(&mut grid);
     rotate_west(&mut grid);
    // tilt_south(&mut grid);
    //
    //
    //    rotate_south(&mut grid);
    let mut g1 = grid.clone();
    let mut g2 = grid.clone();

    rotate_south(&mut g1);
    tilt_south(&mut g2);
    let r =equal_grid(&g1, &g2);
    if !r {
        _compare_grid(&g1,&g2);
        return String::new();
    }

    grid = g1;
     rotate_east(&mut grid);


    /*
    println!("---- start postioni-----");
    _print_grid(&grid);
    println!("-----end s position-----\n");
    let mut g1 = grid.clone();
    let mut g2= grid.clone();

    println!("g1 ?= g2: {}",equal_grid(&g1,&g2));
    rotate_south(&mut g1);
    tilt_south(&mut g2);
    _compare_grid(&g1,&g2);
    return String::new();
*/


    }

    let answer = total_load(&grid);

    return answer.to_string();
}

fn tilt_via_rotations(mut grid: &mut Vec<Vec<char>>) {
    tilt_north(&mut grid);        // North Tilt
    *grid = rotate_clockwise(&grid);         // North->West
    tilt_north(&mut grid);       // West Tilt
    *grid = rotate_clockwise(&grid);         // West->South
    tilt_north(&mut grid);        // South Tilt
    *grid = rotate_clockwise(&grid);         // South->East
    tilt_north(&mut grid);        // East Tile
    *grid = rotate_clockwise(&grid);         // East->North
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

fn rotate_north(field: &mut Vec<Vec<char>>) {
    for row in 1..field.len() {
        'colloop:
        for col in 0..field[row].len() {
            if field[row][col] == 'O' && field[row - 1][col] == '.' {
                for i in (0..row).rev() {
                    if field[i][col] != '.' {
                        field[i + 1][col] = 'O';
                        field[row][col] = '.';
                        continue 'colloop;
                    }
                }
                field[0][col] = 'O';
                field[row][col] = '.';
            }
        }
    }
}


fn rotate_south(field: &mut Vec<Vec<char>>) {
    let field_len = field.len();
    for row in (0..field.len() - 1).rev() {
        'colloop:
        for col in 0..field[row].len() {
            if field[row][col] == 'O' && field[row + 1][col] == '.' {
                for i in row + 1..field.len() {
                    if field[i][col] != '.' {
                        field[i - 1][col] = 'O';
                        field[row][col] = '.';
                        continue 'colloop;
                    }
                }
                field[field_len - 1][col] = 'O';
                field[row][col] = '.';
            }
        }
    }
}

fn rotate_east(field: &mut Vec<Vec<char>>) {
    let row_len = field[0].len();
    for col in (0..field[0].len() - 1).rev() {
        'rowloop:
        for row in 0..field.len() {
            if field[row][col] == 'O' && field[row][col + 1] == '.' {
                for i in col + 1..field[row].len() {
                    if field[row][i] != '.' {
                        field[row][i - 1] = 'O';
                        field[row][col] = '.';
                        continue 'rowloop;
                    }
                }
                field[row][row_len - 1] = 'O';
                field[row][col] = '.';
            }
        }
    }
}

fn rotate_west(field: &mut Vec<Vec<char>>) {
    for col in 1..field[0].len() {
        'rowloop:
        for row in 0..field.len() {
            if field[row][col] == 'O' && field[row][col - 1] == '.' {
                for i in (0..col).rev() {
                    if field[row][i] != '.' {
                        field[row][i + 1] = 'O';
                        field[row][col] = '.';
                        continue 'rowloop;
                    }
                }
                field[row][0] = 'O';
                field[row][col] = '.';
            }
        }
    }
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




fn tilt_south(grid: &mut Vec<Vec<char>>) {
    let field_len = grid.len();
    // for row in 0..grid.len() {
    //     for col in 0..grid[0].len() {
    //         let ch = grid[row][col];
    //         if ch == 'O' {
    //             grid[row][col]= 'X';
    //         }
    //     }
    // }

    for row in (0..grid.len() -1).rev() {
        for col in 0..grid[0].len() {
            let ch = grid[row][col];
            if ch != 'O'  {
                continue;
            }

            let mut o_y = row;
            while o_y < grid.len() -1 {
                let ch = grid[o_y + 1][col];
                if ch == '.' {
                    o_y += 1;
                } else {
                    // hit something, so must stop
                    break;
                }
            }
            if o_y != row {
                grid[o_y][col] = 'O';
                grid[row][col] = '.';
            } else {
           //     assert_eq!(grid[row][col], 'X');
          //      grid[row][col] = 'O';
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

}
fn equal_grid(g1: &Vec<Vec<char>>, g2:&Vec<Vec<char>>) -> bool {
    if (g1.len() != g2.len() ) || (g1[0].len() != g2[0].len()) {
        return false;
    }
    for y in 0..g1.len() {
        for x in 0..g1[0].len() {
            let ch1 = g1[y][x];
            let ch2 = g2[y][x];
            if ch1 != ch2 {
                return false;
            }
        }
    }
    return true;
}



fn _compare_grid(g1: &Vec<Vec<char>>, g2:&Vec<Vec<char>>) -> () {
    let mut r = 10;
    for y in 0..g1.len() {
        for x in 0..g1[0].len() {
            let ch = g1[y][x];
            print!("{ch}");
        }
        print!("\t {r:2} \t");
        for x in 0..g2[0].len() {
            let ch = g2[y][x];
            print!("{ch}");
        }
        println!();

        r -= 1;
    }

}


