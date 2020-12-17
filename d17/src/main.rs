use std::collections::HashMap;
use itertools::iproduct;

fn neighbours(x: i64, y: i64, z:i64, w:i64) -> Vec<(i64, i64, i64, i64)> {
    let mut neighbours: Vec<_> = iproduct!((x-1)..=(x+1), (y-1)..=(y+1), (z-1)..=(z+1), (w-1)..=(w+1)).collect();
    let center_ind = neighbours.iter().position(|pt| *pt == (x, y, z, w)).unwrap();
    neighbours.remove(center_ind);
    neighbours
}

fn boundaries(map: &HashMap<(i64, i64, i64, i64), char>, consider_4th_dimension: bool) -> ((i64, i64), (i64, i64), (i64, i64), (i64, i64)) {
    let (mut min_x, mut max_x, mut min_y, mut max_y, mut min_z, mut max_z, mut min_w, mut max_w) = map.iter().fold(
        (i64::MAX, i64::MIN, i64::MAX, i64::MIN, i64::MAX, i64::MIN, i64::MAX, i64::MIN), |(acc_min_x, acc_max_x, acc_min_y, acc_max_y, acc_min_z, acc_max_z, acc_min_w, acc_max_w), ((x, y, z, w), c)| {
            if *c == '#' {
                (acc_min_x.min(*x), acc_max_x.max(*x), acc_min_y.min(*y), acc_max_y.max(*y), acc_min_z.min(*z), acc_max_z.max(*z), acc_min_w.min(*w), acc_max_w.max(*w))
            } else {
                (acc_min_x, acc_max_x, acc_min_y, acc_max_y, acc_min_z, acc_max_z, acc_min_w, acc_max_w)
            }
        }
    );
    min_x -= 1;
    max_x += 1;
    min_y -= 1;
    max_y += 1;
    min_z -= 1;
    max_z += 1;
    if consider_4th_dimension {
        min_w -= 1;
        max_w += 1;
    } else {
        min_w = 0;
        max_w = 0;
    }
    ((min_x, max_x), (min_y, max_y), (min_z, max_z), (min_w, max_w))
}

fn transform(map: &HashMap<(i64, i64, i64, i64), char>, consider_4th_dimension: bool) -> HashMap<(i64, i64, i64, i64), char> {
    let mut new_map: HashMap<(i64, i64, i64, i64), char> = HashMap::new();
    let  ((min_x, max_x), (min_y, max_y), (min_z, max_z), (min_w, max_w)) = boundaries(map, consider_4th_dimension);
    let space = iproduct!(min_x..=max_x, min_y..=max_y, min_z..=max_z, min_w..=max_w);
    for (x, y, z, w) in space {
        let count = neighbours(x, y, z, w).iter().filter(
            |pt| match map.get(pt) {
                Some('#') => true,
                _         => false
            }
        ).count();
        let new_c = match map.get(&(x, y, z, w)) {
            Some('#') => if count == 2 || count == 3 { '#' } else { '.' }, 
            _         => if count == 3 { '#' } else { '.' }
        };
        new_map.insert((x, y, z, w), new_c);
    }
    new_map
}

fn part_1(start_map: &HashMap<(i64, i64, i64, i64), char>) {
    let mut map = start_map.clone();
    for _ in 0..6 {
        map = transform(&map, false)
    }
    let count = map.iter().filter(|(_, c)| **c == '#').count();
    println!("Answer part 1: {}", count)
}

fn part_2(start_map: &HashMap<(i64, i64, i64, i64), char>) {
    let mut map = start_map.clone();
    for _ in 0..6 {
        map = transform(&map, true)
    }
    let count = map.iter().filter(|(_, c)| **c == '#').count();
    println!("Answer part 2: {}", count)
}

fn main() {
    let mut map: HashMap<(i64, i64, i64, i64), char> = HashMap::new();
    let lines = include_str!("../input.dat");
    let mut y: i64 = 0;
    for line in lines.split('\n') {
        let mut x: i64 = 0;
        for c in line.chars() {
            map.insert((x, y, 0, 0), c);
            x += 1;
        }
        y += 1;
    }
    part_1(&map);
    part_2(&map);
}
