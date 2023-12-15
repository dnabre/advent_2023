use std::fs::File;
use std::io::{BufRead, BufReader};
use std::str::FromStr;

pub fn all_pairs_from_list<T: Clone>(list: Vec<T>) -> Vec<(T, T)> {
    let mut pair_list: Vec<(T, T)> = Vec::new();
    for i in 0..list.len() {
        for j in i + 1..list.len() {
            pair_list.push((list[i].clone(), list[j].clone()));
        }
    }
    return pair_list;
}

pub fn file_to_lines(input_file: &str) -> Vec<String> {
    let file = File::open(input_file).expect(&*format!("error opening file {}", input_file));
    let bfile = BufReader::new(file);
    let lines: Vec<String> = bfile.lines().filter_map(|x| x.ok()).collect();
    return lines;
}


pub fn file_to_single_line(input_file: &str, merge_on:Option<char>) -> String {
    let file = File::open(input_file).expect(&*format!("error opening file {}", input_file));
    let bfile = BufReader::new(file);
    let lines: Vec<String> = bfile.lines().filter_map(|x| x.ok()).collect();
    if lines.len() == 1 {
        return lines[0].clone();
    } else {
        let mut sb = String::new();
        for i in 0..(lines.len()-1) {
            sb.push_str(lines[i].as_str());
             match merge_on {
                None => {}
                Some(ch) => {sb.push(ch); }
            }

        }
        sb.push_str(lines.last().unwrap().as_str());
        return sb.to_string()
    }
}

pub fn gcd_of_two_numbers(a: usize, b: usize) -> usize {
    if b == 0 {
        return a;
    }
    gcd_of_two_numbers(b, a % b)
}

pub fn get_diffs(series: &Vec<i32>) -> Vec<i32> {
    let mut diffs: Vec<i32> = Vec::new();
    if series.len() < 2 {
        return series.clone();
    }
    let mut left = series[0];
    for i in 1..series.len() {
        let d = series[i] - left;
        diffs.push(d);
        left = series[i];
    }
    return diffs;
}

pub fn get_distance_m1((x1, y1): (usize, usize), (x2, y2): (usize, usize)) -> usize {
    return x1.abs_diff(x2) + y1.abs_diff(y2);
}

pub fn get_neighbor_points((x, y): (i32, i32), diag: bool) -> Vec<(i32, i32)> {
    static CARD_DELTA: [(i32, i32); 4] = [(-1, 0), (1, 0), (0, -1), (0, 1)];
    static DIAG_DELTA: [(i32, i32); 4] = [(-1, -1), (-1, 1), (1, -1), (1, 1)];
    let r = y;
    let c = x;

    let mut neighs: Vec<(i32, i32)> = Vec::new();
    for i in 0..CARD_DELTA.len() {
        let (dr, dc) = CARD_DELTA[i];
        neighs.push((r + dr, c + dc));
    }
    if diag {
        for i in 0..DIAG_DELTA.len() {
            let (dr, dc) = DIAG_DELTA[i];
            neighs.push((c + dc, r + dr));
        }
    }
    return neighs;
}

pub fn group_newline_separated_lines(lines: &Vec<String>) -> Vec<String> {
    let mut group_vec: Vec<String> = Vec::new();
    let mut sb = String::new();
    for i in 0..lines.len() {
        let l = &lines[i];
        if l.len() == 0 {
            group_vec.push(sb);
            sb = String::new();
        } else {
            sb.push_str(l);
            sb.push('\n');
        }
    }
    group_vec.push(sb);
    group_vec
}

pub fn is_all_foo<T: std::cmp::PartialEq>(series: &Vec<T>, element: T) -> bool {
    for n in series {
        if *n != element {
            return false;
        }
    }
    return true;
}

pub fn is_all_zero(series: &Vec<i32>) -> bool {
    return is_all_foo(series, 0);
}

pub fn lcm(nums: &[usize]) -> usize {
    if nums.len() == 1 {
        return nums[0];
    }
    let a = nums[0];
    let b = lcm(&nums[1..]);
    a * b / gcd_of_two_numbers(a, b)
}

pub fn list_to_pairs<T: Copy>(galaxy_list: Vec<(T, T)>) -> Vec<((T, T), (T, T))> {
    let mut pair_list: Vec<((T, T), (T, T))> = Vec::new();
    for i in 0..galaxy_list.len() {
        for j in i + 1..galaxy_list.len() {
            pair_list.push((galaxy_list[i], galaxy_list[j]));
        }
    }
    return pair_list;
}

pub fn parse_number_list_comma<T: FromStr>(number_string: &str) -> Vec<T> {
    let oo = number_string.split(",").map(|s| s.trim().parse());
    let un_oo: Vec<T> = oo.map(|r| match r {
        Ok(n) => { n }
        Err(_) => { panic!("Error parsing") }
    }).collect();
    return un_oo;
}

pub fn parse_number_list_whitespace<T: FromStr>(number_string: &str) -> Vec<T> {
    let oo = number_string.split_whitespace().map(|s| s.trim().parse());
    let un_oo: Vec<T> = oo.map(|r| match r {
        Ok(n) => { n }
        Err(_) => { panic!("Error parsing") }
    }).collect();
    return un_oo;
}


pub fn str_to_char_vec(s: &str) -> Vec<char> {
    let mut r_vec: Vec<char> = Vec::with_capacity(s.len());
    let b_array = s.as_bytes();
    for i in 0..s.len() {
        let ch: char = b_array[i] as char;
        r_vec.push(ch);
    }
    return r_vec;
}
