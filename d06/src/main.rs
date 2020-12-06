use std::collections::HashSet;

fn parse_group(sg: &str) -> Vec<HashSet<char>> {
    let mut group: Vec<HashSet<char>> = vec!();
    for s in sg.split('\n') {
        let mut chars: HashSet<char> = HashSet::new();
        for c in s.chars() {
            chars.insert(c);
        }
        group.push(chars)
    }
    group
}

fn count_present_in_groups(group: &Vec<HashSet<char>>) -> usize {
    let mut merged = group[0].clone();
    for ind in 1..group.len() {
        merged = merged.union(&group[ind]).map(|c| *c).collect();
    }
    merged.len()
}

fn count_common_in_groups(group: &Vec<HashSet<char>>) -> usize {
    let mut merged = group[0].clone();
    for ind in 1..group.len() {
        merged = merged.intersection(&group[ind]).map(|c| *c).collect();
    }
    merged.len()
}

fn main() {
    let groups: Vec<_> = include_str!("../input.dat").split("\n\n").map(
        |s| parse_group(s)
    ).collect();
    let n_any_group = groups.iter().fold(0, |acc, g| acc + count_present_in_groups(g));
    let n_all_groups = groups.iter().fold(0, |acc, g| acc + count_common_in_groups(g));
    println!("Count part 1: {}", n_any_group);
    println!("Count part 2: {}", n_all_groups) 
}
