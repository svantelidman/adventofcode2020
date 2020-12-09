fn has_matching_sum(sample: usize, nums: &[usize]) -> bool {
    for i in 0..nums.len() {
        for j in 0..nums.len() {
            if i != j && nums[i] + nums[j] == sample {
                return true
            }
        }
    }
    false
}

fn part_1(nums: &Vec<usize>, n_win: usize) -> usize {
    for ind in n_win..nums.len() {
        if !has_matching_sum(nums[ind], &nums[ind-n_win..ind]) {
            return nums[ind];
        }
    }
    panic!("Could not solve part 1.")
}

fn part_2(nums: &Vec<usize>, weakness_1: usize) -> usize {
    for ind_start in 0..nums.len()-1 {
        let mut acc = 0;
        for ind in ind_start..nums.len() {
            acc += nums[ind];
            if acc > weakness_1 {
                break;
            }
            if acc == weakness_1 && ind != ind_start {
                let smallest = nums[ind_start..=ind].iter().min().unwrap();
                let largest = nums[ind_start..=ind].iter().max().unwrap();
                return smallest + largest
            }
        }
    }
    panic!("Could not solve part 2.")
}

fn main() {
    let nums: Vec<_> = include_str!("../input.dat").split('\n').map(
        |s| s.parse::<usize>().unwrap()
    ).collect();
    let n_win = 25;
    let weakness_1 = part_1(&nums, n_win);
    println!("Answer part 1 = {}", weakness_1);
    let weakness_2 = part_2(&nums, weakness_1);
    println!("Answer part 2 = {}", weakness_2)
}
