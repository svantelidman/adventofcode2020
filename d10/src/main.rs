fn part_1(nums: &Vec<usize>) -> usize {
    let (mut n1, mut n3) = (0, 0);
    for ind in 1..nums.len() {
        let diff = nums[ind] - nums[ind - 1];
        if diff == 1 {
            n1 += 1
        } else if diff == 3 {
            n3 += 1
        }
    }
    n1 * n3
}

fn split_segs(nums: &Vec<usize>) -> Vec<Vec<usize>> {
    let mut segs = vec!();
    let mut seg = vec!();
    for n in nums {
        if seg.len() == 0 || n - seg[seg.len()-1] < 3 {
            seg.push(*n)
        } else {
            segs.push(seg);
            seg = vec!(*n)
        }
    }
    segs
}

fn part_2(nums: &Vec<usize>) -> usize {
    let seg_lens: Vec<_> = split_segs(&nums).iter().map(|seg| seg.len()).collect();
    let seg_combos: Vec<_> = seg_lens.iter().map(|l|
        match *l {
            3 => 2,
            4 => 4,
            5 => 7,
            _ => 1
        }
    ).collect();
    seg_combos.iter().fold(1, |acc, sc| acc*sc)
}

fn main() {
    let mut nums: Vec<_> = include_str!("../input.dat").split('\n').map(
        |s| s.parse::<usize>().unwrap()
    ).collect();
    nums.push(0);
    nums.sort();
    nums.push(nums.last().unwrap() + 3);
    println!("Answer part 1 = {}", part_1(&nums));
    println!("Answer part 2 = {}", part_2(&nums))
}
