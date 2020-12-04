fn main() {
    let map: Vec<_> = include_str!("../input.dat").split('\n').map(
        |line| line.chars().map(|ch| ch == '#').collect::<Vec<_>>()
    ).collect();
    let slopes = vec!((1, 1), (3, 1), (5, 1), (7, 1), (1, 2));
    let prod_trees = slopes.into_iter().fold(1, |acc, (move_right, move_down)| acc * calc_trees_for_slope(move_right, move_down, &map));
    println!("Tree count product: {}", prod_trees)
}

fn calc_trees_for_slope(move_right: usize, move_down: usize, map: &Vec<Vec<bool>>) -> usize {
    let (n_rows, n_cols)  = (map.len(), map[0].len());
    let  (mut current_col, mut current_row, mut n_trees) = (0, 0, 0);
    while current_row < n_rows {
        current_row += move_down;
        current_col += move_right;
        if current_row >= n_rows {
            break
        }
        if current_col >= n_cols {
            current_col -= n_cols
        }
        if map[current_row][current_col] {
            n_trees += 1
        }
    }
    n_trees
}