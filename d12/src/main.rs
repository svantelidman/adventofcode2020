fn rotate(x: i64, y: i64, a: i64) -> (i64, i64) {
    match (a + 360) % 360 {
        90 => (-y, x),
        180 => (-x, -y),
        270 => (y, -x),
        _ => panic!("Could not rotate to: {}", a)
    }
}

fn apply_move_part_1(cp: &((i64, i64), (i64, i64)), mv: &(char, i64)) -> ((i64, i64), (i64, i64)) {
    let ((mut cdx, mut cdy), (mut cx, mut cy)) = cp;
    match mv {
        ('N', dist) => cy -= dist,
        ('E', dist) => cx += dist,
        ('S', dist) => cy += dist,
        ('W', dist) => cx -= dist,
        ('F', dist) => {
            cx += cdx * dist;
            cy += cdy * dist;
        },
        ('L', deg) => {let (ncdx, ncdy) = rotate(cdx, cdy, -*deg); cdx = ncdx; cdy = ncdy},
        ('R', deg) => {let (ncdx, ncdy) = rotate(cdx, cdy, *deg); cdx = ncdx; cdy = ncdy},
        _ => panic!("Unexpected move")
    }
    ((cdx, cdy), (cx, cy))
}

fn part_1(moves: &Vec<(char, i64)>) {
    let mut position: ((i64, i64), (i64, i64)) =  ((1, 0), (0, 0));
    for mv in moves {
        position = apply_move_part_1(&position, &mv)
    }
    let (_, (x, y)) = position;
    println!("Answer part 1; {}", x.abs() + y.abs())
}

fn apply_move_part_2(cp: &(i64, i64), mv: &(char, i64), wp: &(i64, i64)) -> ((i64, i64), (i64, i64)) {
    let (mut cx, mut cy) = cp;
    let (mut wx, mut wy) = wp;
    match mv {
        ('N', dist) => wy -= dist,
        ('E', dist) => wx += dist,
        ('S', dist) => wy += dist,
        ('W', dist) => wx -= dist,
        ('F', count) => {
            cx += wx * count;
            cy += wy * count;
        },
        ('L', deg) => {let (nwx, nwy) = rotate(wx, wy, -*deg); wx = nwx; wy = nwy},
        ('R', deg) => {let (nwx, nwy) = rotate(wx, wy, *deg); wx = nwx; wy = nwy},
        _ => panic!("Unexpected move")
    }
    ((cx, cy), (wx, wy))
}

fn part_2(moves: &Vec<(char, i64)>) {
    let mut position: (i64, i64) =  (0, 0);
    let mut waypoint: (i64, i64) = (10, -1); 
    for mv in moves {
        let (new_position, new_waypoint) = apply_move_part_2(&position, &mv, &waypoint);
        position = new_position;
        waypoint = new_waypoint;
    }
    let (x, y) = position;
    println!("Answer part 2: {}", x.abs() + y.abs())
}

fn main() {
    let moves: Vec<_> = include_str!("../input.dat").split('\n').map(
        |s| (s.chars().next().unwrap(), s[1..].parse::<i64>().unwrap())
    ).collect();
    part_1(&moves);
    part_2(&moves)
}