use std::collections::HashSet;

#[derive(Debug)]
struct BagSpec {
    color: String,
    contains: Vec<(usize, String)>
}

impl BagSpec {
    fn parse(s: &str) -> BagSpec {
        let mut split_1 = s.split(" bags contain ");
        let color = String::from(split_1.next().unwrap());
        let mut contains: Vec<(usize, String)> = vec!();
        let split_2 = split_1.next().unwrap().split(", ");
        for ss in split_2 {
            if ss.starts_with(|c: char| c.is_numeric()) {
                let first_space =  ss.find(' ').unwrap();
                let num: usize = ss[0..first_space].parse().unwrap();
                let color = ss[first_space..].trim_end_matches(" bags");
                let color = color.trim_end_matches(" bags.");
                let color = color.trim_end_matches(" bag.");
                let color = color.trim_end_matches(" bag");
                contains.push((num, String::from(color.trim())));
            }
        }
        BagSpec{color, contains}
    }
}

fn part_1(bag_specs: &Vec<BagSpec>) {
    let mut found_colors: HashSet<String> = HashSet::new();
    let mut search_for_colors = vec!(String::from("shiny gold"));
    loop {
        let newly_found_colors: HashSet<_> = search_for_colors.iter().flat_map(|s| 
            bag_specs.iter().filter(move |bs| bs.contains.iter().any(|(_, c)| c == s)).map(|bs| bs.color.clone())
        ).collect();
        let found_clone = found_colors.clone();
        let unique_found_colors: HashSet<_> = newly_found_colors.difference(&found_clone).collect();
        if unique_found_colors.len() == 0 {
            break;
        }
        search_for_colors = unique_found_colors.iter().map(|s| (*s).clone()).collect();
        for new_color in unique_found_colors {
            found_colors.insert(new_color.clone());
        }
    }
    println!("Answer part 1 = {}", found_colors.len())
}

fn count_contained_bags(bag_color: &String, bag_specs: &Vec<BagSpec>) -> usize {
    let bag = bag_specs.iter().find(|bs| bs.color == *bag_color).unwrap();
    bag.contains.iter().map(|(n, c)| n*(1 + count_contained_bags(c, bag_specs))).sum()
}

fn part_2(bag_specs: &Vec<BagSpec>) {
    let count = count_contained_bags(&String::from("shiny gold"), bag_specs);
    println!("Answer part 2 = {}", count)
}   

fn main() {
    let bag_specs: Vec<_> = include_str!("../input.dat").split('\n').map(
        |s| BagSpec::parse(&s)
    ).collect();
    part_1(&bag_specs);
    part_2(&bag_specs)
}
