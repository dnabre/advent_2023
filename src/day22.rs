use std::collections::HashMap;
use std::hash::Hash;
use std::time::Instant;

/*
    Advent of Code 2023: Day 22
        part1 answer:   485
        part2 answer:   74594

*/
const ANSWER: (&str, &str) = ("485", "74594");

fn main() {
    let _filename_test = "data/day22/test_input_01.txt";
    let _filename_test2 = "data/day22/test_input_02.txt";

    let filename_part1 = "data/day22/part1_input.txt";
    let filename_part2 = "data/day22/part2_input.txt";

    println!("Advent of Code, Day 22");
    println!("    ---------------------------------------------");
    let start1 = Instant::now();
    let answer1 = part1(filename_part1);
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

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
struct Coord {
    x: i32,
    y: i32,
    z: i32,
}

fn parse_coords_pair(line: &String) -> (Coord, Coord) {
    let (left, right) = line.split_once("~").unwrap();
    let (cc1, cc2) = (parse_coords(left), parse_coords(right));
    return (cc1, cc2);
}


fn parse_coords(l: &str) -> Coord {
    let p1: Vec<&str> = l.split(",").collect();
    let cc1 = Coord {
        x: p1[0].parse().unwrap(),
        y: p1[1].parse().unwrap(),
        z: p1[2].parse().unwrap(),
    };
    return cc1;
}

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
struct Brick {
    id: usize,
    coords1: Coord,
    coords2: Coord,
}


fn parse_brick_list(lines:Vec<String>)->Vec<Brick> {
    let mut brick_list = Vec::new();
    for i in 0..lines.len() {
        let l = &lines[i];
        let (cc1, cc2) = parse_coords_pair(l);

        let b = Brick {
            id: i,
            coords1: cc1,
            coords2: cc2,
        };
        brick_list.push(b);
    }
    brick_list.sort_by_cached_key(|b| b.coords1.z.min(b.coords2.z));
    return brick_list;
}


fn part1(input_file: &str) -> String {
    let lines = advent_2023::file_to_lines(input_file);
    let mut bricks = parse_brick_list(lines);

    let mut change = true;
    while change {
        let mut new_bricks = Vec::with_capacity(bricks.len());
        change = false;
        for brick in &bricks {
            let new_brick = if brick.get_height_above_ground() > 1 {
                let potential_brick = brick.move_by(0, 0, -1);
                let mut intersects = false;
                for other_brick in bricks.iter().filter(|b| b.id != brick.id) {
                    if potential_brick.intersects(other_brick) {
                        intersects = true;
                        break;
                    }
                }
                if intersects {
                    brick.clone()
                } else {
                    change = true;
                    potential_brick
                }
            } else {
                brick.clone()
            };
            new_bricks.push(new_brick);
        }
        bricks = new_bricks;
    }
    let (supported_by,supports) = generate_supports(&mut bricks);


//  break this chain out to something more readable
    let answer = (0..bricks.len())
        .filter(|i| supports.get(i)
            .map_or(true, |s| s.iter().all(|j| supported_by[j].len() > 1))).count();
    return answer.to_string();
}





fn part2(input_file: &str) -> String {
    let lines = advent_2023::file_to_lines(input_file);
    let mut bricks = Vec::new();
    for i in 0..lines.len() {
        let l = &lines[i];
        let (cc1, cc2) = parse_coords_pair(l);

        let b = Brick {
            id: i,
            coords1: cc1,
            coords2: cc2,
        };
        bricks.push(b);
    }
    bricks.sort_by_cached_key(|b| b.coords1.z.min(b.coords2.z));



    let mut change = true;
    while change {
        let mut new_bricks = Vec::with_capacity(bricks.len());
        change = false;
        for brick in &bricks {
            let new_brick = if brick.get_height_above_ground() > 1 {
                let potential_brick = brick.move_by(0, 0, -1);
                let mut intersects = false;
                for other_brick in bricks.iter().filter(|b| b.id != brick.id) {
                    if potential_brick.intersects(other_brick) {
                        intersects = true;
                        break;
                    }
                }
                if intersects {
                    brick.clone()
                } else {
                    change = true;
                    potential_brick
                }
            } else {
                brick.clone()
            };
            new_bricks.push(new_brick);
        }
        bricks = new_bricks;
    }

    let (supported_by,_) = generate_supports(&mut bricks);

    let mut answer = 0;
    for brick in &bricks {
        let mut supported_by_temp = supported_by.clone();
        // remove current brick from supported_by
        supported_by_temp.iter_mut().for_each(|(_, l)| l.retain(|i| *i != brick.id));
        let mut change = true;
        while change {
            change = false;
            // Check if any bricks are now unsupported and remove them from supports
            let unsupported = supported_by_temp.iter().filter(|(_, l)| l.is_empty()).map(|(i, _)| *i).collect::<Vec<_>>();
            for brick_id in &unsupported {
                supported_by_temp.iter_mut().for_each(|l| l.1.retain(|i| *i != *brick_id));
                supported_by_temp.remove(&brick_id);
                change = true;
            }
            answer += unsupported.len();
        }
    }

    return answer.to_string();
}

fn generate_supports(bricks: &mut Vec<Brick>) -> (HashMap<usize, Vec<usize>>, HashMap<usize, Vec<usize>>) {
    let mut supported_by: HashMap<usize, Vec<usize>> = HashMap::new();
    let mut supports: HashMap<usize, Vec<usize>> = HashMap::new();

    for (i, brick) in bricks.iter().enumerate() {
        let down_brick = brick.move_by(0, 0, -1);
        for (j, other_brick) in bricks.iter().enumerate() {
            if i != j && down_brick.intersects(other_brick) {
                supported_by.entry(i).or_default().push(j);
                supports.entry(j).or_default().push(i);
            }
        }
    }

    for (i, brick) in bricks.iter().enumerate() {
        if brick.get_height_above_ground() == 1 {
            supported_by.entry(i).or_default().push(usize::MAX);
        }
    }
    (supported_by,supports)
}


impl Brick {
    fn intersects(&self, other: &Self) -> bool {
        let (x1, y1, z1) = (self.coords1.x, self.coords1.y, self.coords1.z);
        let (x2, y2, z2) = (self.coords2.x, self.coords2.y, self.coords2.z);
        let highx = x1.max(x2);
        let lowx = x1.min(x2);
        let highy = y1.max(y2);
        let lowy = y1.min(y2);
        let highz = z1.max(z2);
        let lowz = z1.min(z2);
        let (x3, y3, z3) = (other.coords1.x, other.coords1.y, other.coords1.z);
        let (x4, y4, z4) = (other.coords2.x, other.coords2.y, other.coords2.z);
        let otherhighx = x3.max(x4);
        let otherlowx = x3.min(x4);
        let otherhighy = y3.max(y4);
        let otherlowy = y3.min(y4);
        let otherhighz = z3.max(z4);
        let otherlowz = z3.min(z4);
        let xintersects = (otherlowx >= lowx && otherlowx <= highx) || (otherhighx >= lowx && otherhighx <= highx) || (lowx >= otherlowx && lowx <= otherhighx) || (highx >= otherlowx && highx <= otherhighx);
        let yintersects = (otherlowy >= lowy && otherlowy <= highy) || (otherhighy >= lowy && otherhighy <= highy) || (lowy >= otherlowy && lowy <= otherhighy) || (highy >= otherlowy && highy <= otherhighy);
        let zintersects = (otherlowz >= lowz && otherlowz <= highz) || (otherhighz >= lowz && otherhighz <= highz) || (lowz >= otherlowz && lowz <= otherhighz) || (highz >= otherlowz && highz <= otherhighz);
        xintersects && yintersects && zintersects
    }

    fn move_by(&self, x: i32, y: i32, z: i32) -> Self {
        let (x1, y1, z1) = (self.coords1.x, self.coords1.y, self.coords1.z);
        let (x2, y2, z2) = (self.coords2.x, self.coords2.y, self.coords2.z);
        Self {
            id: self.id,
            coords1: Coord {
                x: x1 + x,
                y: y1 + y,
                z: z1 + z,
            },
            coords2: Coord {
                x: x2 + x,
                y: y2 + y,
                z: z2 + z,
            },
        }
    }

    fn get_height_above_ground(&self) -> i32 {
        let (_, _, z1) = (self.coords1.x, self.coords1.y, self.coords1.z);
        let (_, _, z2) = (self.coords2.x, self.coords2.y, self.coords2.z);
        z1.min(z2)
    }
}