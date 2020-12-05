fn split_vec(vec: Vec<usize>) -> (Vec<usize>, Vec<usize>) {
    let mid = vec.len() / 2;
    let (l, r) = vec.split_at(mid);
    let l = l.iter().map(|n| *n).collect();
    let r = r.iter().map(|n| *n).collect();
    (l, r)
} 

fn decode(code: &String) -> (usize, usize, usize)  {  //row, column, code
    let mut possible_row: Vec<usize> = (0..128).collect();
    let mut possible_col: Vec<usize> = (0..8).collect();
    let row_code = &code.as_str()[..7];
    let col_code = &code.as_str()[7..];
    for c in row_code.chars() {
        let (lower, upper) = split_vec(possible_row);
        match c {
            'F' => possible_row = lower,
            'B' => possible_row = upper,
            _ => panic!("Unknown row partition: {}", c)
        }
    }
    for c in col_code.chars() {
        let (left_most, right_most) = split_vec(possible_col);
        match c {
            'L' => possible_col = left_most,
            'R' => possible_col = right_most,
            _ => panic!("Unknown col partition: {}", c)
        }
    }
    let row = possible_row[0];
    let col = possible_col[0];
    (row, col, row * 8 + col)
}

fn main() {
    let codes: Vec<_> = include_str!("../input.dat").split('\n').map(|s| String::from(s)).collect();
    let mut all_ids: Vec<_> = codes.iter().map(|code| decode(code)).map(|(_, _, id)| id).collect();
    let highest_id = all_ids.iter().fold(0, |acc, id| acc.max(*id));
    println!("Highest id = {}", highest_id);
    all_ids.sort();
    for ind in 0..all_ids.len()-1 {
        if all_ids[ind+1] - all_ids[ind] == 2 {
            println!("My Id = {}", all_ids[ind] + 1);
            break
        }
    }
}
