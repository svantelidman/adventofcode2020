#[derive(Clone, Eq, PartialEq)]
enum State {
    Floor,
    Empty,
    Occupied
}

fn count_visible_occupied(row: usize, col: usize, map: &Vec<Vec<State>>, only_adjacent: bool) -> usize {
    let n_row = map.len();
    let n_col = map[0].len();
    let mut n_visible_occupied = 0;
    let directions = vec!(
        (0 ,  1),
        (-1,  1),
        (-1,  0),
        (-1, -1),
        (0,  -1),
        (1,  -1),
        (1,   0),
        (1,   1)
    );
    for (dr, dc) in directions {
        let mut current_row = row as i64;
        let mut current_col = col as i64;
        loop {
            current_row += dr;
            current_col += dc;
            if current_row < 0 || current_col < 0 || current_row >= n_row as i64 || current_col >= n_col as i64 {
                break
            }
            if map[current_row as usize][current_col as usize] == State::Occupied {
                n_visible_occupied += 1;
                break
            }
            if map[current_row as usize][current_col as usize] == State::Empty {
                break
            }
            if only_adjacent {
                break
            }
        }
    }
    n_visible_occupied
} 

fn transform(map: &Vec<Vec<State>>, only_adjacent: bool, free_trigger: usize) -> Vec<Vec<State>> {
    let mut new_map = map.clone();
    let n_rows = map.len();
    let n_cols = map[0].len();
    for row in 0..n_rows {
        for col in 0..n_cols {
            match map[row][col] {
                State::Empty => {
                    if count_visible_occupied(row, col, map, only_adjacent) == 0 {
                        new_map[row][col] = State::Occupied
                    }
                },
                State::Occupied => {
                    if count_visible_occupied(row, col, map, only_adjacent) >= free_trigger {
                        new_map[row][col] = State::Empty
                    }
                },
                _ => {}
            }
        }
    }
    new_map
}

fn calc_stable_occupied(map: &Vec<Vec<State>>, only_adjacent: bool, free_trigger: usize) -> usize {
    let mut previous_map = map.clone();
    loop {
        let new_map = transform(&previous_map, only_adjacent, free_trigger);
        if new_map == previous_map {
            break
        }
        previous_map = new_map;
    }
    previous_map.iter().fold(0, |acc, row| acc + row.iter().filter(|seat| **seat == State::Occupied).count())
}

fn main() {
    let map: Vec<_> = include_str!("../input.dat").split('\n').map(
        |s| s.chars().map(|c|
            match c {
                'L' => State::Empty,
                _   => State::Floor
            }
        ).collect::<Vec<_>>()
    ).collect();
    println!("Answer part 1 = {}", calc_stable_occupied(&map, true, 4));
    println!("Answer part 2 = {}", calc_stable_occupied(&map, false, 5));
}
