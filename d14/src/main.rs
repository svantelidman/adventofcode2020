use regex::Regex;
use std::collections::HashMap;

#[derive(Debug)]
enum Operation {
    Mask {
        mask: String
    },
    Mem {
        location: u64,
        value: u64
    }
}

fn apply_mask_1(val: u64, mask: &String) -> u64 {
    let mut new_val = val;
    let mut bit_ind = 0;
    for c in mask.chars().rev() {
        let c_mask = 1 << bit_ind; 
        match c {
            '1' => {
                new_val = new_val | c_mask
            },
            '0' => {
                if new_val & c_mask > 0 {
                    new_val -= c_mask
                } 
            },
            _ => {}
        }
        bit_ind += 1
    }
    new_val
}

fn part_1(program: &Vec<Operation>) {
    let mut memory: HashMap<u64, u64> = HashMap::new();
    let mut current_mask = String::from("XXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXX");
    for op in program {
        match op {
            Operation::Mask{mask} => {
                current_mask = mask.clone()
            },
            Operation::Mem{location, value} => {
                let masked_value = apply_mask_1(*value, &current_mask);
                memory.insert(*location, masked_value);
            }
        }
    }
    let answer = memory.values().sum::<u64>();
    println!("Answer part 1: {}", answer)
}

fn spawn_floating_masks(current_masks: &Vec<u64>, bit_ind: u64) -> Vec<u64> {
    let mut new_masks = vec!();
    for mask in current_masks {
        new_masks.push(*mask);
        let spawned = *mask | 1 << bit_ind;
        new_masks.push(spawned);
    }
    new_masks
}

fn apply_mask_2(loc: u64, mask: &String) -> Vec<u64> {
    let mut spawned_locs = vec!();
    let mut new_loc = loc;
    let mut bit_ind = 0;
    let mut floating_inds = vec!();
    for c in mask.chars().rev() {
        let c_mask = 1 << bit_ind; 
        match c {
            '1' => {
                new_loc = new_loc | c_mask
            },
            '0' => {},
            'X' => {
                floating_inds.push(bit_ind)
            },
            _ => panic!("Unknown bit in mask")
        }
        bit_ind += 1
    }

    let mut floating_masks: Vec<u64> = vec!(0);
    for flb in &floating_inds {
        floating_masks = spawn_floating_masks(&floating_masks, *flb)
    }
    for flm in floating_masks {
        let mut spawned_loc = new_loc;
        for bit_ind in &floating_inds {
            let bit_value = 1 << bit_ind;
            if flm & bit_value > 0 {
                // Mask is set
                spawned_loc |= bit_value;
                
            } else {
                // Mask is not set
                if spawned_loc & bit_value > 0 {
                    spawned_loc -= bit_value
                }
            }
        }
        spawned_locs.push(spawned_loc)
    }
    spawned_locs
}

fn part_2(program: &Vec<Operation>) {
    let mut memory: HashMap<u64, u64> = HashMap::new();
    let mut current_mask = String::from("XXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXX");
    for op in program {
        match op {
            Operation::Mask{mask} => {
                current_mask = mask.clone()
            },
            Operation::Mem{location, value} => {
                let locs = apply_mask_2(*location, &current_mask);
                for loc in locs {
                    memory.insert(loc, *value);
                }
            }
        }
    }
    let answer = memory.values().sum::<u64>();
    println!("Answer part 2: {}", answer)
}

fn main() {
    let re_match_mask = Regex::new(r"^mask").unwrap();
    let re_parse_mask = Regex::new(r"^mask = (.+)$").unwrap();
    let re_parse_mem = Regex::new(r"^mem\[(\d+)\] = (\d+)$").unwrap();

    let program: Vec<_> = include_str!("../input.dat").split('\n').map(
        |s| {
            if re_match_mask.is_match(s) {
                let captures = re_parse_mask.captures_iter(s).next().unwrap();
                let mask = String::from(&captures[1]);
                Operation::Mask{ mask }
            } else {
                let captures = re_parse_mem.captures_iter(s).next().unwrap();
                let location = captures[1].parse::<u64>().unwrap();
                let value = captures[2].parse::<u64>().unwrap();
                Operation::Mem{ location, value }
            }
        }
    ).collect();

    part_1(&program);
    part_2(&program);
}
