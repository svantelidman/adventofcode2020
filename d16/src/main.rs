use regex::Regex;
use std::ops::RangeInclusive;
use std::collections::HashSet;

struct Rule {
    name: String,
    range_a: RangeInclusive<usize>,
    range_b: RangeInclusive<usize>,
}

impl Rule {
    fn fulfils(&self, val: usize) -> bool {
        self.range_a.contains(&val) || self.range_b.contains(&val)
    }
}

fn fulfils_any_rule(val: usize, rules: &Vec<Rule>) -> bool {
    rules.iter().any(|rule| rule.fulfils(val))
}

fn part_1(rules: &Vec<Rule>, other_tickets: &Vec<Vec<usize>>) {
    let mut error_rate = 0;
    for tic in other_tickets {
        for val in tic {
            if !fulfils_any_rule(*val, rules) {
                error_rate += val
            }
        }
    }
    println!("Answer part 1: {}", error_rate)
}

fn find_matching_rules_for_inds(inds: &Vec<usize>, rules: &Vec<Rule>, tickets: &Vec<Vec<usize>>, matched_rules: &Vec<(String, usize)>) -> Vec<(String, usize)> { // returns (rule_name, field_on_ticket_ind)
    let mut matches: Vec<(String, usize)> = vec!(); 
    for field_ind in inds {
        let field_values: Vec<_> = tickets.iter().map(|tic| tic[*field_ind]).collect();
        let mut matching_rules: Vec<_> = rules.iter().filter(|r| field_values.iter().all(|v| r.fulfils(*v))).collect();
        matching_rules = matching_rules.into_iter().filter(|r| !matched_rules.iter().any(|(rn, _)| *rn == r.name)).collect();
        if matching_rules.len() == 1 {
            let rule_name = matching_rules[0].name.clone();
            matches.push((rule_name, *field_ind))
        }
    }
    matches
}

fn part_2(rules: &Vec<Rule>, my_ticket: &Vec<usize>, valid_tickets: &Vec<Vec<usize>>) {
    let mut all_tickets = valid_tickets.clone();
    all_tickets.push(my_ticket.clone());
    let mut matched_rules: Vec<(String, usize)> = vec!();
    let mut remaining_field_inds: Vec<usize> = (0..rules.len()).collect();
    while matched_rules.len() < rules.len() {
        let mut matches = find_matching_rules_for_inds(&remaining_field_inds, rules, &all_tickets, &matched_rules);
        matched_rules.append(&mut matches);
        let matched_inds: HashSet<_> = matched_rules.iter().map(|(_, ind)| *ind).collect();
        remaining_field_inds = (0..rules.len()).filter(|ind| !matched_inds.contains(ind)).collect();
    }
    let departure_inds: Vec<_> = matched_rules.iter().filter(|(name, _)| name.starts_with("departure ")).map(|(_, ind)| *ind).collect();
    let answer: usize = departure_inds.iter().map(|ind| my_ticket[*ind]).product();
    println!("Answer part 2: {}", answer)
}

fn main() {
    let rule_re = Regex::new(r"\s*([^:]+): (\d+)-(\d+) or (\d+)-(\d+)").unwrap();
    let sections: Vec<_> = include_str!("../input.dat")
        .split("\n\n")
        .map(|s| String::from(s))
        .collect();
    let rules_sec = &sections[0];
    let rules: Vec<_> = rule_re
        .captures_iter(rules_sec)
        .map(|cap| {
            let name = String::from(&cap[1]);
            let lower_a = &cap[2].parse::<usize>().unwrap();
            let upper_a = &cap[3].parse::<usize>().unwrap();
            let lower_b = &cap[4].parse::<usize>().unwrap();
            let upper_b = &cap[5].parse::<usize>().unwrap();
            let range_a = *lower_a..=*upper_a;
            let range_b = *lower_b..=*upper_b;
            Rule {
                name,
                range_a,
                range_b,
            }
        })
        .collect();
    let my_tic_sec = &sections[1];
    let other_tic_sec = &sections[2];
    let my_ticket: Vec<_> = my_tic_sec.split(',').map(|s| s.parse::<usize>().unwrap()).collect();
    let other_tickets: Vec<_> = other_tic_sec
        .split('\n')
        .map(|line| {
            line.split(',')
                .map(|s| s.parse::<usize>().unwrap())
                .collect::<Vec<_>>()
        })
        .collect();

    part_1(&rules, &other_tickets);

    let valid_tickets: Vec<_> = other_tickets.into_iter().filter(|tic| 
        tic.iter().all(|val| fulfils_any_rule(*val, &rules))
    ).collect();

    part_2(&rules, &my_ticket, &valid_tickets);
}
