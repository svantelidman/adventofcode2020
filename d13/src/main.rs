fn calc_wait_time(earliest: usize, bus_id: usize) -> usize {
    bus_id - earliest % bus_id
}

fn part_1(earliest: usize, bus_ids: &Vec<usize>) {
    let wait_times: Vec<_> = bus_ids.iter()
        .map(|id| (*id, calc_wait_time(earliest, *id))).collect();
    let best_bus = wait_times.iter().min_by_key(|(_id, t)| *t).unwrap();
    let answer = best_bus.0 * best_bus.1;
    println!("Answer part 1: {}", answer)
}

fn part_2(bus_ids_with_delay: &Vec<(usize, usize)>) {
    let all_buses: Vec<_> = bus_ids_with_delay.iter().map(|x| x.clone()).collect();
    let mut cursor = 0;
    let (first_id, _) = all_buses.first().unwrap();
    let mut incr = *first_id;
    for bus_ind in 1..all_buses.len() {
        let (id, delay) = all_buses[bus_ind];
        loop {
            cursor += incr;
            if (cursor + delay) % id == 0 {
                incr *= id;
                break;
            }
        }
    }
    println!("Answer part 2: {}", cursor)
}

fn main() {
    let mut lines = include_str!("../input.dat").split('\n');
    let earliest: usize = lines.next().unwrap().parse().unwrap();
    let bus_ids: Vec<usize> = lines.next().unwrap().split(',')
        .filter(|s| *s != "x")
        .map(|s| s.parse::<usize>().unwrap()).collect();
    part_1(earliest, &bus_ids);

    let mut lines = include_str!("../input.dat").split('\n');
    let _earliest: usize = lines.next().unwrap().parse().unwrap();
    let bus_ids: Vec<&str> = lines.next().unwrap().split(',').collect();
    let bus_ids_with_delay = bus_ids.iter().enumerate()
        .filter(|(_, id_str)| **id_str != "x")
        .map(|(delay, id_str)| (id_str.parse::<usize>().unwrap(), delay)).collect();
    part_2(&bus_ids_with_delay);
}