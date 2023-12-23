#![allow(unused_variables)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(dead_code)]
#![allow(unused_assignments)]
#![allow(unreachable_code)]

use std::collections::{BTreeMap, HashMap, HashSet, VecDeque};
use std::fmt::{Display, Formatter};
use std::hash::Hash;
use std::sync::atomic::AtomicU64;
use std::sync::atomic::Ordering::SeqCst;
use std::time::Instant;

/*
    Advent of Code 2023: Day 20
        part1 answer:   825896364
        part2 answer:   243566897206981

*/

const ANSWER: (&str, &str) = ("825896364", "243566897206981");
static BUTTON_PRESS_COUNT:usize = 1000;

fn main() {
    let _filename_test = "data/day20/test_input_01.txt";
    let _filename_test2 = "data/day20/test_input_02.txt";

    let filename_part1 = "data/day20/part1_input.txt";
    let filename_part2 = "data/day20/part2_input.txt";

    // println!("Advent of Code, Day 20");
    println!("    ---------------------------------------------");
    let start1 = Instant::now();
    let answer1 = part1(filename_part1);
    let duration1 = start1.elapsed();

    println!("\t Part 1: {:14} time: {:?}", answer1, duration1);
    if ANSWER.0 != answer1 {
        println!("\t\t ERROR: Answer is WRONG. Got: {}, Expected {}", answer1, ANSWER.0);
    }

    let start2 = Instant::now();
    let answer2 = part2(filename_part1);
    let duration2 = start2.elapsed();

    println!("\t Part 2: {:14} time: {:?}", answer2, duration2);
    if ANSWER.1 != answer2 {
        println!("\t\t ERROR: Answer is WRONG. Got: {}, Expected {}", answer2, ANSWER.1);
    }
    println!("    ---------------------------------------------");
}

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
enum Pulse {
    High,
    Low,
}

impl Display for Pulse {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "-{}-", match self {
            Pulse::High => { "high" }
            Pulse::Low => { "low" }
        })
    }
}


#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
struct PulseInstance {
    strength: Pulse,
    source: usize,
    dest: usize,
}

static HIGH_PULSE_COUNT: AtomicU64 = AtomicU64::new(0);
static LOW_PULSE_COUNT: AtomicU64 = AtomicU64::new(0);

impl PulseInstance {
    fn new(strength: Pulse, source: usize, dest: usize) -> Self {
        match strength {
            Pulse::High => {
                HIGH_PULSE_COUNT.fetch_add(1, SeqCst);
            }
            Pulse::Low => {
                LOW_PULSE_COUNT.fetch_add(1, SeqCst);
            }
        }
        Self { strength, source, dest }
    }
}

#[derive(Debug, PartialEq, Eq, Hash, Clone)]
struct FlipFlop {
    on: bool,
    output: Vec<usize>,
}

#[derive(Debug, PartialEq, Eq, Hash, Clone)]
struct Conjunction {
    state: BTreeMap<usize, Pulse>,
    inputs: Vec<usize>,
    output: Vec<usize>,
}

#[derive(Debug, PartialEq, Eq, Hash, Clone)]
struct Broadcaster {
    output: Vec<usize>,
}

#[derive(Debug, PartialEq, Eq, Hash, Clone)]
struct Button {
    output: Vec<usize>,
}

#[derive(Debug, PartialEq, Eq, Hash, Clone)]
enum Module {
    EFlopFlop(FlipFlop),
    EConjunction(Conjunction),
    EBroadcaster(Broadcaster),
    EButton(Button),
}

const FLIP_FLOP_PREFIX: u8 = 37_u8;
const CONJ_PREFIX: u8 = 38u8;
const BROADCAST_PREFIX: u8 = 98_u8;

fn part1(input_file: &str) -> String {
    let lines = advent_2023::file_to_lines(input_file);

    let mut modules: Vec<Module> = Vec::new();
    let mut name_lookup: HashMap<&str, usize> = HashMap::new();
    let mut name_by_id: HashMap<usize, &str> = HashMap::new();
    let mut module_names: Vec<&str> = Vec::new();
    // let mut output_modules: Vec<(&str, usize)> = Vec::new();
    let mut output_modules: HashSet<usize> = HashSet::new();
    let mut m_broadcast_id: Option<usize> = None;

    let mut assign_id = 0;

    // setup button here
    let button_id = assign_id;
    let button_name = "button";
    assign_id += 1;
    name_lookup.insert(button_name, button_id);
    name_by_id.insert(button_id, button_name);
    module_names.push(button_name);
    let mut button_s = Button { output: vec![] };
    modules.push(Module::EButton(button_s));


    for l in &lines {
        let (src, dest) = l.split_once("->").unwrap();
        let mut src = trim_module_prefix(src.trim());
        module_names.push(src);
        name_by_id.insert(assign_id, src);
        name_lookup.insert(src, assign_id);
        assign_id += 1;
    }

    let mut conj_list: Vec<usize> = Vec::new();
    let mut id: usize = 0;
    for l in &lines {
        let (src, dest) = l.split_once("->").unwrap();
        let src = src.trim();
        let dest_list: Vec<&str> = dest.split(",").map(|s| s.trim()).collect();

        let src_id = name_lookup[trim_module_prefix(src)];

        let mut dest_ids: Vec<usize> = Vec::new();
        for i in 0..dest_list.len() {
            let key = dest_list[i];

            if !name_lookup.contains_key(&key) {
                let n_dest_id = assign_id;
                assign_id += 1;

                output_modules.insert(n_dest_id);
                name_lookup.insert(key, n_dest_id);
                name_by_id.insert(n_dest_id, key);
                module_names.push(key);
            }
            let dest_id = name_lookup[dest_list[i]];
            dest_ids.push(dest_id);
        }
        let f = src.as_bytes()[0];
        let m = match f {
            FLIP_FLOP_PREFIX => {
                let f = FlipFlop { on: false, output: dest_ids };
                Module::EFlopFlop(f)
            }
            CONJ_PREFIX => {
                conj_list.push(src_id);
                let c = Conjunction {
                    state: BTreeMap::new(),
                    inputs: vec![],
                    output: dest_ids,
                };
                Module::EConjunction(c)
            }
            BROADCAST_PREFIX => {
                let b = Broadcaster { output: dest_ids };
                m_broadcast_id = Some(src_id);
                Module::EBroadcaster(b)
            }
            _ => {
                panic!("No Untyped Modules should be sources: {}", l);
            }
        };
        modules.push(m);
    }
    let mut broadcast_id;
    if let Some(b) = m_broadcast_id {
        broadcast_id = b;


        if let Module::EButton(_) = &modules[0] {
            modules[0] = Module::EButton(Button {
                output: vec![broadcast_id],
            });
        }
    } else {
        panic!("didn't find broadcast id");
    }
    let broadcast_id = broadcast_id;


    // println!("    ---------------------------------------------");
    // println!("broadcast: {}", broadcast_id);
    // println!("output sinks: {:?}", output_modules);
    // println!("    ---------------------------------------------");

    let number_modules = modules.len();
    for i in 0..number_modules {
        if let Module::EConjunction(conj_i) = &modules[i] {
            let mut c = conj_i.clone();


            for j in 0..number_modules {
                if i == j { continue; }
                match &modules[j] {
                    Module::EFlopFlop(f) => {
                        if f.output.contains(&i) {
                            c.inputs.push(j);
                            c.state.insert(j, Pulse::Low);
                        }
                    }
                    Module::EConjunction(f) => {
                        if f.output.contains(&i) {
                            c.inputs.push(j);
                            c.state.insert(j, Pulse::Low);
                        }
                    }
                    Module::EBroadcaster(f) => {
                        if f.output.contains(&i) {
                            c.inputs.push(j);
                            c.state.insert(j, Pulse::Low);
                        }
                    }
                    Module::EButton(f) => {
                        if f.output.contains(&i) {
                            c.inputs.push(j);
                            c.state.insert(j, Pulse::Low);
                        }
                    }
                }
            }
            modules[i] = Module::EConjunction(c);
        }
    }



    let mut pulse_queue: VecDeque<PulseInstance> = VecDeque::new();

    let button_push = PulseInstance {
        strength: Pulse::Low,
        source: button_id,
        dest: broadcast_id,
    };

    let mut high_pulse_count:u64 =0;
    let mut low_pulse_count:u64 = 0;


    for _ in 0..BUTTON_PRESS_COUNT{
        pulse_queue.push_front(button_push);

        while let Some(p) = pulse_queue.pop_front() {
            match p.strength {
                Pulse::High => {high_pulse_count +=1 }
                Pulse::Low => { low_pulse_count +=1}
            }
            //println!("processing front of queue: \t {:?}", p);
            let src_name = name_by_id[&p.source];
            let dest_name = name_by_id[&p.dest];
        //    println!("{src_name} {} > {dest_name}", p.strength);

            if output_modules.contains(&p.dest) {
                let name = name_by_id[&p.dest];
                //       println!("output {name} received pulse: {}", p.strength);
                continue;
            }

            let mut target_module = &mut modules[p.dest];
            if let Module::EFlopFlop(ref mut ff) = target_module {
                if p.strength == Pulse::Low {
                    if ff.on {
                        ff.on = false;
                        for d in &ff.output {
                            let new_p = PulseInstance {
                                strength: Pulse::Low,
                                source: p.dest,
                                dest: *d,
                            };
                            pulse_queue.push_back(new_p);
                        }
                    } else {
                        ff.on = true;
                        for d in &ff.output {
                            let new_p = PulseInstance {
                                strength: Pulse::High,
                                source: p.dest,
                                dest: *d,
                            };
                            pulse_queue.push_back(new_p);
                        }
                    }
                }
            }
            if let Module::EConjunction(ref mut c) = target_module {
                c.state.insert(p.source, p.strength);
                let t = c.state.iter().all(|(k, v)| *v == Pulse::High);
                let out_pulse = if t {
                    Pulse::Low
                } else {
                    Pulse::High
                };
                for d in &c.output {
                    let new_p = PulseInstance {
                        strength: out_pulse,
                        source: p.dest,
                        dest: *d,
                    };
                    pulse_queue.push_back(new_p);
                }
            }
            if let Module::EBroadcaster(ref mut b) = target_module {
                for d in &b.output {
                    let new_p = PulseInstance {
                        strength: p.strength,
                        source: p.dest,
                        dest: *d,
                    };
                    pulse_queue.push_back(new_p);
                }
            }
            if let Module::EButton(ref mut b) = target_module {
                for d in &b.output {
                    let new_p = PulseInstance {
                        strength: Pulse::Low,
                        source: p.dest,
                        dest: *d,
                    };
                    pulse_queue.push_back(new_p);
                }
            }
        }
     //   println!("    ---------------------------------------------");

    }

    println!();
    println!("    ---------------------------------------------");
    println!(" low  pulse count: {}", low_pulse_count);
    println!(" high pulse count: {}", high_pulse_count);
    let answer = high_pulse_count * low_pulse_count;
    return answer.to_string();
}


fn part2(input_file: &str) -> String {
    let lines = advent_2023::file_to_lines(input_file);

    let mut modules: Vec<Module> = Vec::new();
    let mut name_lookup: HashMap<&str, usize> = HashMap::new();
    let mut name_by_id: HashMap<usize, &str> = HashMap::new();
    let mut module_names: Vec<&str> = Vec::new();
    // let mut output_modules: Vec<(&str, usize)> = Vec::new();
    let mut output_modules: HashSet<usize> = HashSet::new();
    let mut m_broadcast_id: Option<usize> = None;

    let mut assign_id = 0;

    // setup button here
    let button_id = assign_id;
    let button_name = "button";
    assign_id += 1;
    name_lookup.insert(button_name, button_id);
    name_by_id.insert(button_id, button_name);
    module_names.push(button_name);
    let mut button_s = Button { output: vec![] };
    modules.push(Module::EButton(button_s));


    for l in &lines {
        let (src, dest) = l.split_once("->").unwrap();
        let mut src = trim_module_prefix(src.trim());
        module_names.push(src);
        name_by_id.insert(assign_id, src);
        name_lookup.insert(src, assign_id);
        assign_id += 1;
    }

    let mut conj_list: Vec<usize> = Vec::new();
    let mut id: usize = 0;
    for l in &lines {
        let (src, dest) = l.split_once("->").unwrap();
        let src = src.trim();
        let dest_list: Vec<&str> = dest.split(",").map(|s| s.trim()).collect();

        let src_id = name_lookup[trim_module_prefix(src)];

        let mut dest_ids: Vec<usize> = Vec::new();
        for i in 0..dest_list.len() {
            let key = dest_list[i];

            if !name_lookup.contains_key(&key) {
                let n_dest_id = assign_id;
                assign_id += 1;

                output_modules.insert(n_dest_id);
                name_lookup.insert(key, n_dest_id);
                name_by_id.insert(n_dest_id, key);
                module_names.push(key);
            }
            let dest_id = name_lookup[dest_list[i]];
            dest_ids.push(dest_id);
        }
        let f = src.as_bytes()[0];
        let m = match f {
            FLIP_FLOP_PREFIX => {
                let f = FlipFlop { on: false, output: dest_ids };
                Module::EFlopFlop(f)
            }
            CONJ_PREFIX => {
                conj_list.push(src_id);
                let c = Conjunction {
                    state: BTreeMap::new(),
                    inputs: vec![],
                    output: dest_ids,
                };
                Module::EConjunction(c)
            }
            BROADCAST_PREFIX => {
                let b = Broadcaster { output: dest_ids };
                m_broadcast_id = Some(src_id);
                Module::EBroadcaster(b)
            }
            _ => {
                panic!("No Untyped Modules should be sources: {}", l);
            }
        };
        modules.push(m);
    }
    let mut broadcast_id;
    if let Some(b) = m_broadcast_id {
        broadcast_id = b;


        if let Module::EButton(_) = &modules[0] {
            modules[0] = Module::EButton(Button {
                output: vec![broadcast_id],
            });
        }
    } else {
        panic!("didn't find broadcast id");
    }
    let broadcast_id = broadcast_id;


    // println!("    ---------------------------------------------");
    // println!("broadcast: {}", broadcast_id);
    // println!("output sinks: {:?}", output_modules);
    // println!("    ---------------------------------------------");

    let number_modules = modules.len();
    for i in 0..number_modules {
        if let Module::EConjunction(conj_i) = &modules[i] {
            let mut c = conj_i.clone();


            for j in 0..number_modules {
                if i == j { continue; }
                match &modules[j] {
                    Module::EFlopFlop(f) => {
                        if f.output.contains(&i) {
                            c.inputs.push(j);
                            c.state.insert(j, Pulse::Low);
                        }
                    }
                    Module::EConjunction(f) => {
                        if f.output.contains(&i) {
                            c.inputs.push(j);
                            c.state.insert(j, Pulse::Low);
                        }
                    }
                    Module::EBroadcaster(f) => {
                        if f.output.contains(&i) {
                            c.inputs.push(j);
                            c.state.insert(j, Pulse::Low);
                        }
                    }
                    Module::EButton(f) => {
                        if f.output.contains(&i) {
                            c.inputs.push(j);
                            c.state.insert(j, Pulse::Low);
                        }
                    }
                }
            }
            modules[i] = Module::EConjunction(c);
        }
    }



    //
    // for i in 0..lines.len() {
    //     println!("_ {i:3}\t{:5} \t {:?}", module_names[i], modules[i]);
    //
    // }

    let zg_i = name_lookup["zg"];
    println!("_ {zg_i:3}\t{:5} \t {:?}", module_names[zg_i], modules[zg_i]);

    let zg_inputs:[usize;4] = [3,13,26,29];
    let mut cycles:HashMap<usize, Vec<u64> >  = HashMap::new();
    for z in zg_inputs {
        cycles.insert(z, Vec::new());
    }
    println!("z: {:?}", cycles);
    let mut pulse_queue: VecDeque<PulseInstance> = VecDeque::new();

    let button_push = PulseInstance {
        strength: Pulse::Low,
        source: button_id,
        dest: broadcast_id,
    };

    let mut high_pulse_count:u64 =0;
    let mut low_pulse_count:u64 = 0;

    // 59
let mut button_press:u64 =0;



'button: loop {
        button_press +=1;
        pulse_queue.push_front(button_push);

        while let Some(p) = pulse_queue.pop_front() {
            match p.strength {
                Pulse::High => {high_pulse_count +=1 }
                Pulse::Low => { low_pulse_count +=1}
            }


            if p.dest==zg_i && p.strength==Pulse::High {
                let t = p.source;
                let u = button_press;
                cycles.get_mut(&t).unwrap().push(u);
                let mut cc =0;
                for f in 0..zg_inputs.len() {
                    let ff = zg_inputs[f];
                    let tt = cycles[&ff].len();
                    cc += tt;
                }
                if cc > 16 {
                    println!("zg_hash:\n \t {:?}", cycles);
                    for f in 0..zg_inputs.len() {
                        let ff = zg_inputs[f];
                        let tt = cycles[&ff].clone();
                        print!{"_ {f:3}\t - "}
                        advent_2023::list_displayables_to_string(&tt);
                        println!();

                        println!("\n ----------------------- \n");
                    }
                    break 'button;
                }
            }

            if zg_i == p.dest && p.strength == Pulse::High {
                println!("    ---------------------------------------------");
                            println!("button press: {button_press} ");
                println!("pulse: {:?}", p);
                println!("zg: {:?}", modules[zg_i]);

            }
            if p.dest == 59 && p.strength == Pulse::Low{
                println!("button press number {}", button_press);
                break 'button;
            }

            //println!("processing front of queue: \t {:?}", p);
            let src_name = name_by_id[&p.source];
            let dest_name = name_by_id[&p.dest];
            //    println!("{src_name} {} > {dest_name}", p.strength);

            if output_modules.contains(&p.dest) {
                let name = name_by_id[&p.dest];
                //       println!("output {name} received pulse: {}", p.strength);
                continue;
            }

            let mut target_module = &mut modules[p.dest];
            if let Module::EFlopFlop(ref mut ff) = target_module {
                if p.strength == Pulse::Low {
                    if ff.on {
                        ff.on = false;
                        for d in &ff.output {
                            let new_p = PulseInstance {
                                strength: Pulse::Low,
                                source: p.dest,
                                dest: *d,
                            };
                            pulse_queue.push_back(new_p);
                        }
                    } else {
                        ff.on = true;
                        for d in &ff.output {
                            let new_p = PulseInstance {
                                strength: Pulse::High,
                                source: p.dest,
                                dest: *d,
                            };
                            pulse_queue.push_back(new_p);
                        }
                    }
                }
            }
            if let Module::EConjunction(ref mut c) = target_module {
                c.state.insert(p.source, p.strength);
                let t = c.state.iter().all(|(k, v)| *v == Pulse::High);
                let out_pulse = if t {
                    Pulse::Low
                } else {
                    Pulse::High
                };
                for d in &c.output {
                    let new_p = PulseInstance {
                        strength: out_pulse,
                        source: p.dest,
                        dest: *d,
                    };
                    pulse_queue.push_back(new_p);
                }
            }
            if let Module::EBroadcaster(ref mut b) = target_module {
                for d in &b.output {
                    let new_p = PulseInstance {
                        strength: p.strength,
                        source: p.dest,
                        dest: *d,
                    };
                    pulse_queue.push_back(new_p);
                }
            }
            if let Module::EButton(ref mut b) = target_module {
                for d in &b.output {
                    let new_p = PulseInstance {
                        strength: Pulse::Low,
                        source: p.dest,
                        dest: *d,
                    };
                    pulse_queue.push_back(new_p);
                }
            }
            if zg_i == p.dest && p.strength == Pulse::High {
                println!("zg: {:?}", modules[zg_i]);
                println!("    ---------------------------------------------");
            }
        }
    //     println!("    ---------------------------------------------");

    }


    let periods = [4057,3929,3907,3911];
    let lv = advent_2023::lcm(&periods);
    println!("lvm of peroids: {}", lv);
    return lv.to_string();



    println!();
    println!("    ---------------------------------------------");
    println!(" low  pulse count: {}", low_pulse_count);
    println!(" high pulse count: {}", high_pulse_count);
    let answer = high_pulse_count * low_pulse_count;
    return answer.to_string();
}

fn trim_module_prefix(i_src: &str) -> &str {
    let mut src = i_src;
    if i_src.starts_with("%") {
        src = i_src.strip_prefix("%").unwrap();
    }
    if i_src.starts_with("&") {
        src = i_src.strip_prefix("&").unwrap();
    }
    src
}