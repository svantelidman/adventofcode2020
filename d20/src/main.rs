use regex::Regex;
use std::collections::HashSet;

#[derive(Clone)]
struct Tile {
    id: usize,
    rows: Vec<Vec<char>>
}

impl Tile {
    fn merge_from_tile_grid(grid: &Vec<Vec<Tile>>) -> Tile {
        let mut rows: Vec<Vec<char>> = vec!();
        let grid_edge_len = grid[0].len();
        rows.resize(8*grid_edge_len, vec!());
        let mut irow = 0;
        for grid_row in grid {
            for tile_row in 0..8usize {
                for grid in grid_row {
                    for tile_col in 0..8usize {
                        let ch = grid.rows[tile_row][tile_col];
                        rows[irow].push(ch);
                    }
                }
                irow += 1
            }
        }


        Tile{ id: 0, rows}
    }

    fn new(s: &str) -> Tile {
        let tile_re = Regex::new(r"^Tile (\d+):$").unwrap();
        let mut rows: Vec<Vec<char>> = vec!();
        let lines: Vec<_> = s.split('\n').map(|s| String::from(s)).collect();
        let caps = tile_re.captures(lines[0].as_str()).unwrap();
        let id: usize  = caps[1].parse().unwrap();
        for row_ind in 1..=10 {
            rows.push(lines[row_ind].chars().collect::<Vec<_>>())
        }
        Tile { id, rows }
    }

    fn strip_border(&mut self) {
        let edge_len = self.rows[0].len();
        for row in 0..edge_len {
            self.rows[row].remove(edge_len-1);
            self.rows[row].remove(0);
        }
        self.rows.remove(edge_len-1);
        self.rows.remove(0);
    }

    fn make_left_edge(&mut self, edge_code: usize) {
        for _ in 0..4 {
            self.rotate();
            if self.left_edge_code(false) == edge_code {
                return
            }
        }
        self.flip();
        for _ in 0..4 {
            self.rotate();
            if self.left_edge_code(false) == edge_code {
                return
            }
        }
        panic!("Could not find left edge")
    }

    fn make_top_edge(&mut self, edge_code: usize) {
        for _ in 0..4 {
            self.rotate();
            if self.top_edge_code(false) == edge_code {
                return
            }
        }
        self.flip();
        for _ in 0..4 {
            self.rotate();
            if self.top_edge_code(false) == edge_code {
                return
            }
        }
        panic!("Could not find top edge")        
    }

    fn make_top_and_left_edge(&mut self, edge_code_1: usize, edge_code_2: usize) {
        for _ in 0..4 {
            self.rotate();
            if self.top_edge_code(false) == edge_code_1 && self.left_edge_code(false) == edge_code_2 ||
            self.top_edge_code(false) == edge_code_2 && self.left_edge_code(false) == edge_code_1 
            {
                return
            }
        }
        self.flip();
        for _ in 0..4 {
            self.rotate();
            if self.top_edge_code(false) == edge_code_1 && self.left_edge_code(false) == edge_code_2 ||
            self.top_edge_code(false) == edge_code_2 && self.left_edge_code(false) == edge_code_1 
            {
                return
            }
        }
        panic!("Could not find top left")        
    }

    fn get_possible_edge_codes(&self) -> Vec<usize> {
        vec!(self.left_edge_code(false), self.right_edge_code(false), self.top_edge_code(false), self.bottom_edge_code(false),
        self.left_edge_code(true), self.right_edge_code(true), self.top_edge_code(true), self.bottom_edge_code(true))
    }

    fn get_current_edge_codes(&self) -> Vec<usize> {
        vec!(self.left_edge_code(false), self.right_edge_code(false), self.top_edge_code(false), self.bottom_edge_code(false))
    }

    fn left_edge_code(&self, reverse: bool) -> usize {
        let mut left_edge: Vec<_> = self.rows.iter().map(|row| row[0]).collect();
        if reverse {
            left_edge.reverse()
        } 
        Tile::calc_edge_code(&left_edge)
    }

    fn right_edge_code(&self, reverse: bool) -> usize {
        let mut right_edge: Vec<_> = self.rows.iter().map(|row| row[9]).collect();
        if reverse {
            right_edge.reverse()
        } 
        Tile::calc_edge_code(&right_edge)
    }

    fn top_edge_code(&self, reverse: bool) -> usize {
        let mut top_edge = self.rows[0].clone();
        if reverse {
            top_edge.reverse()
        } 
        Tile::calc_edge_code(&top_edge)
    }

    fn bottom_edge_code(&self, reverse: bool) -> usize {
        let mut bottom_edge = self.rows[9].clone();
        if reverse {
            bottom_edge.reverse()
        } 
        Tile::calc_edge_code(&bottom_edge)
    }

    fn calc_edge_code(edge_chars: &Vec<char>) -> usize {
        let mut edge_code = 0;
        for ind in 0..10 {
            if edge_chars[ind] == '#' {
                edge_code |= 1 << ind
            }
        }        
        edge_code
    }

    fn rotate(&mut self) {
        let edge_length = self.rows.len();
        let mut new_rows: Vec<Vec<char>> = vec!();
        new_rows.resize(edge_length, vec!());
        for ind_col in 0..edge_length {
            for ind_row in (0..edge_length).rev() {
                new_rows[ind_col].push(self.rows[ind_row][ind_col])
            }
        }
        self.rows = new_rows
    } 

    fn flip(&mut self) {
        self.rows.reverse()
    }
}

impl std::fmt::Display for Tile {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "Tile id: {}", self.id).unwrap();
        for row in &self.rows {
            let row_string: String = row.iter().collect();
            writeln!(f, "{}", row_string).unwrap()
        }
        std::fmt::Result::Ok(())
    }
}

fn all_codes_except(tiles: &Vec<Tile>, exclude_id: usize) -> HashSet<usize> {
    tiles.iter().filter(|t| t.id != exclude_id).flat_map(|tile| tile.get_possible_edge_codes()).collect()
}

fn part_1(tiles: &Vec<Tile>) {
    let corner_codes: Vec<_> = tiles.iter().filter(
        |tile| 
        tile.get_current_edge_codes().iter().filter(|code| !all_codes_except(tiles, tile.id).contains(code)).count() == 2).map(|t| t.id).collect();
    let answer: usize = corner_codes.iter().product();
    println!("Answer part 1: {}", answer)
}

fn part_2(in_tiles: &Vec<Tile>, monster_inds: &Vec<(usize, usize)>) {
    let corner_ids: Vec<_> = in_tiles.iter().filter(
        |tile| 
        tile.get_current_edge_codes().iter().filter(|code| !all_codes_except(in_tiles, tile.id).contains(code)).count() == 2).map(|t| t.id).collect();
    let edge_length = (in_tiles.len() as f64).sqrt() as usize;
    let mut grid: Vec<Vec<Tile>> = vec!();
    grid.resize(edge_length, vec!());

    // Based on trial and error, which is not evident from the code
    let upper_left_corner_id = if edge_length == 3 { corner_ids[2] } else { corner_ids[3] };
    let mut tiles = (*in_tiles).clone();
    let candidate_pos = tiles.iter().position(|t| t.id == upper_left_corner_id).unwrap();
    let mut corner_candidate = tiles.remove(candidate_pos);
    let all_other_edge_codes = all_codes_except(&tiles, 0);
    let current_edge_codes = corner_candidate.get_current_edge_codes();
    let outer_edge_codes: Vec<_> = current_edge_codes.iter().filter(|code| !all_other_edge_codes.contains(code)).collect();
    corner_candidate.make_top_and_left_edge(*outer_edge_codes[0], *outer_edge_codes[1]);
    let mut next_edge_code = corner_candidate.right_edge_code(false);
    grid[0].push(corner_candidate);
    for _col in 1..edge_length {
        let next_tile_pos = tiles.iter().position(|t| t.get_possible_edge_codes().contains(&next_edge_code)).unwrap();
        let mut next_tile = tiles.remove(next_tile_pos);
        next_tile.make_left_edge(next_edge_code);
        next_edge_code = next_tile.right_edge_code(false);
        grid[0].push(next_tile);
    }
    for row in 1..edge_length {
        for col in 0..edge_length {
            let next_edge_code = (grid[row-1][col]).bottom_edge_code(false);
            let next_tile_pos = tiles.iter().position(|t| t.get_possible_edge_codes().contains(&next_edge_code)).unwrap();
            let mut next_tile = tiles.remove(next_tile_pos);
            next_tile.make_top_edge(next_edge_code);
            grid[row].push(next_tile);
        }
    }
    for tile_row in 0..edge_length {
        for tile_col in 0..edge_length {
            grid[tile_row][tile_col].strip_border();
        }            
    }

    let mut mega_tile = Tile::merge_from_tile_grid(&grid);
    let tile_edge_length = mega_tile.rows[0].len();
    let monster_width = 20;
    let monster_height = 3;

    let mut monster_positions: Vec<(usize, usize)> = vec!();

    // Based on trial and error, which is not evident from the code 
    if edge_length == 3 {
        mega_tile.rotate();
    } else {
        mega_tile.flip();
        mega_tile.rotate();
        mega_tile.rotate();
    }
    for ir in 0..(tile_edge_length - monster_height) {
        for ic in 0..(tile_edge_length - monster_width) {
            if monster_inds.iter().all(|(mr, mc)| mega_tile.rows[ir + mr][ic + mc] == '#') {
                monster_positions.push((ir, ic))
            }
        }
    }
    for (mr, mc) in monster_positions {
        for (ir, ic) in monster_inds {
            mega_tile.rows[mr+ir][mc+ic] = 'O';
        }
    }
    let hash_count = mega_tile.rows.iter().fold(0, |acc, row| acc + row.iter().filter(|c| **c == '#').count());
    println!("Answer part 2: {}", hash_count);
}

fn main() {
    let mut tiles: Vec<_> = include_str!("../input.dat").split("\n\n").map(
        |s| Tile::new(s)
    ).collect();

    let monster_lines: Vec<_> = include_str!("../monster.dat").split('\n').collect();
    let mut monster_inds: Vec<(usize, usize)> = vec!();
    let mut row_ind = 0;
    for line in monster_lines {
        let mut col_ind = 0;
        for c in line.chars() {
            if c == '#' {
                monster_inds.push((row_ind, col_ind))
            }
            col_ind += 1;
        }
        row_ind += 1;
    }

    part_1(&tiles);
    part_2(&mut tiles, &monster_inds);
}
