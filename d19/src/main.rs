use regex::Regex;
use std::collections::HashMap;
use std::collections::HashSet;

#[derive(Debug)]
enum Rule {
    Direct {
        id: usize,
        ch: char
    },
    Indirect {
        id: usize,
        clauses: Vec<Vec<usize>>
    }
}

impl Rule {
    fn get_id(&self) -> usize {
        match self {
            Rule::Direct {id, ch: _} => *id,
            Rule::Indirect {id, clauses: _} => *id
        }
    }

    fn is_match(&self, chars: &Vec<char>, ind: usize, rules: &HashMap<usize, Rule>, part_2_rules: bool) -> (bool, usize) {
        match self {
            Rule::Direct{id: _, ch} => {
                if ind < chars.len() && chars[ind] == *ch {
                    (true, ind + 1)
                } else {
                    (false, ind)
                }
            },
            Rule::Indirect{id: _, clauses} => {
                let mut new_ind = ind;
                for clause in clauses {
                    for rule_id in clause {
                        let (matched, new_new_ind) = rules.get(rule_id).unwrap().is_match(chars, new_ind, rules, part_2_rules);
                        if !matched {
                            new_ind = ind;
                            break
                        } else {
                            new_ind = new_new_ind
                        }
                    }
                    if new_ind != ind {
                        return (true, new_ind)
                    }
                }
                (false, ind)
            }
        }
    }
}

fn parse_rule(s: &str) -> Rule {
    let re_direct = Regex::new(r#"^(\d+): "([a-z])"$"#).unwrap();
    let re_indirect = Regex::new(r"^(\d+): (.+)$").unwrap();
    if re_direct.is_match(s) {
        let caps = re_direct.captures_iter(s).next().unwrap();
        let id = caps[1].parse::<usize>().unwrap();
        let ch = caps[2].parse::<char>().unwrap();
        Rule::Direct{ id, ch }
    } else {
        let caps = re_indirect.captures_iter(s).next().unwrap();
        let id = caps[1].parse::<usize>().unwrap();
        let clauses:Vec<Vec<_>> = caps[2].split('|').map(
            |cls| cls.split(' ').filter(|s| s.len() > 0).map(|s| s.parse::<usize>().unwrap()).collect::<Vec<_>>()
        ).collect();
        Rule::Indirect{ id, clauses}
    }
}

fn main() {
    let sections: Vec<_> = include_str!("../input.dat").split("\n\n").map(
        |s| String::from(s)
    ).collect();
    let rules: HashMap<usize, Rule> = sections[0].split('\n').map(
        |s| {let r = parse_rule(s); (r.get_id(), r)}
    ).collect();
    let messages: Vec<_> = sections[1].split('\n').map(
        |s| String::from(s)
    ).collect();

    // Part 1
    let mut matched_messages: HashSet<String> = HashSet::new();
    for m in &messages {
        let m_chars: Vec<_> = m.chars().collect();
        let is_match = rules.iter().any(
            |(_, r)| {
                let (mtx , new_ind) = r.is_match(&m_chars, 0, &rules, false);
                mtx && new_ind == m.len()
            }
        );
        if is_match {
            matched_messages.insert(String::from(m));
        }
    }
    println!("Answer part 1: {}", matched_messages.len());

    // Part 2
    let r42 = rules.get(&42).unwrap();
    let r31 = rules.get(&31).unwrap();
    for m in &messages {
        let m_chars: Vec<_> = m.chars().collect();
        let mut ind_42s: Vec<(usize, usize)> = vec!((0, 0));
        let mut next_ind = 0;
        let mut count_42 = 1;
        loop {
            let (was_match, new_ind) = r42.is_match(&m_chars, next_ind, &rules, false);
            if was_match {
                next_ind = new_ind;
                ind_42s.push((new_ind, count_42));
                count_42 += 1;
            } else {
                break
            }
        }
        for (start_ind, count_42) in &ind_42s[..] {
            let mut next_ind = *start_ind;
            let mut count_31 = 0;
            loop {
                let (was_match, new_ind) = r31.is_match(&m_chars, next_ind, &rules, false);
                if was_match {
                    count_31 += 1;
                    next_ind = new_ind;
                    if m.len() == next_ind && *count_42 > 1 && count_31 > 0 && *count_42 > count_31 {
                        matched_messages.insert(String::from(m));
                    }
                } else {
                    break
                }
            }
        }    
    }
    println!("Answer part 2: {}", matched_messages.len());
}
