use std::collections::HashSet;

fn calc_score(deck: &Vec<usize>) -> usize {
    let n_cards = deck.len();
    deck.iter().zip((1..=n_cards).rev()).map(|(card, score_mult)| card*score_mult).sum()
}

fn part_1(deck_1: &Vec<usize>, deck_2: &Vec<usize>) {
    let mut d1 = deck_1.clone();
    let mut d2 = deck_2.clone();
    while d1.len() > 0 && d2.len() > 0 {
        let c1 = d1.remove(0);
        let c2 = d2.remove(0);
        assert!(c1 != c2);
        if c1 > c2 {
            d1.push(c1);
            d1.push(c2)
        } else {
            d2.push(c2);
            d2.push(c1)
        }
    }
    if d1.len() > 0 {
        println!("Player 1 wins part 1, score: {}", calc_score(&d1))
    } else {
        println!("Player 2 wins part 2, score: {}", calc_score(&d2))
    }
}

fn play_recursive(deck_1: &Vec<usize>, deck_2: &Vec<usize>) -> (Vec<usize>, Vec<usize>) {
    let mut d1 = deck_1.clone();
    let mut d2 = deck_2.clone();
    let mut previous_deck_states: HashSet<(Vec<usize>, Vec<usize>)> = HashSet::new();
    let mut deck_state = (d1.clone(), d2.clone());
    while d1.len() > 0 && d2.len() > 0 && !previous_deck_states.contains(&deck_state) {
        previous_deck_states.insert(deck_state);
        let c1 = d1.remove(0);
        let c2 = d2.remove(0);
        if d1.len() >= c1 && d2.len() >= c2 {
            let recursive_deck_1: Vec<_> = d1.iter().take(c1).map(|c| *c).collect();
            let recursive_deck_2: Vec<_> = d2.iter().take(c2).map(|c| *c).collect();
            let (end_deck_1, end_deck_2) = play_recursive(&recursive_deck_1, &recursive_deck_2);
            if end_deck_2.len() == 0 || (end_deck_1.len() > 0 && end_deck_2.len() > 0) {
                d1.push(c1);
                d1.push(c2)
            } else {
                d2.push(c2);
                d2.push(c1)
            }
        } else {
            if c1 > c2 {
                d1.push(c1);
                d1.push(c2)
            } else {
                d2.push(c2);
                d2.push(c1)
            }
        }
        deck_state = (d1.clone(), d2.clone());
    }
    (d1, d2)
}

fn part_2(deck_1: &Vec<usize>, deck_2: &Vec<usize>) {
    let (end_deck_1, end_deck_2) = play_recursive(&deck_1, &deck_2);
    if end_deck_2.len() == 0 || (end_deck_1.len() > 0 && end_deck_2.len() > 0) {
        println!("Player 1 wins part 1, score: {}", calc_score(&end_deck_1))
    } else {
        println!("Player 2 wins part 2, score: {}", calc_score(&end_deck_2))
    }
}

fn main() {
    let decks: Vec<_> = include_str!("../input.dat").split("\n\n").map(|s|String::from(s)).collect();
    let deck_1: Vec<_> = decks[0].split('\n').map(|s| s.parse::<usize>().unwrap()).collect();
    let deck_2: Vec<_> = decks[1].split('\n').map(|s| s.parse::<usize>().unwrap()).collect();

    part_1(&deck_1, &deck_2);
    part_2(&deck_1, &deck_2);
}
