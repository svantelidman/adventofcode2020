fn main() {
    let nums: Vec<_> = include_str!("../input.dat")
        .split('\n')
        .map(|s| s.parse::<usize>().unwrap())
        .collect();
    for i in 0..(nums.len()-2) {
        for j in (i + 1)..(nums.len()-1) {
            for k in (j + 1)..nums.len() {
                if nums[i] + nums[j] + nums[k] == 2020 {
                    println!("{}+{}+{}={}", nums[i], nums[j], nums[k], nums[i] + nums[j] + nums[k]);
                    println!("{}*{}*{}={}", nums[i], nums[j], nums[k], nums[i] * nums[j] * nums[k]);
                }
            }
        }
    }
}
