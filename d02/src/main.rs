fn main() {
    #[derive(Debug)]
    struct PwdEntry {
        pwd: String,
        ch: char,
        min: usize,
        max: usize
    }

    impl PwdEntry {
        fn is_valid_part_1(&self) -> bool {
            let occurences = self.pwd.chars().filter(|c| *c == self.ch).count();
            occurences >= self.min && occurences <= self.max 
        }

        fn is_valid_part_2(&self) -> bool {
            let mut pos = 1;
            let mut match_count = 0;
            for c in self.pwd.chars() {
                if (pos == self.min || pos == self.max) && c == self.ch {
                    match_count += 1
                }
                pos +=1
            }
            match_count == 1
        }
    }

    let entries: Vec<_> = include_str!("../input.dat").split('\n').map(
        |l| {
            let mut splitter = l.split(|c| c == ':' || c == ' ' || c == '-');
            let min = splitter.next().unwrap().parse::<usize>().unwrap();
            let max = splitter.next().unwrap().parse::<usize>().unwrap();
            let ch = splitter.next().unwrap().parse::<char>().unwrap();
            let _pwd = splitter.next().unwrap();
            let pwd = splitter.next().unwrap();
            PwdEntry {
                min: min,
                max: max,
                ch: ch,
                pwd: String::from(pwd)
            }
        }
    ).collect();

    let n_valid = entries.iter().filter(|e| e.is_valid_part_1()).count();
    println!("Number of valid entries part 1: {}", n_valid);
    let n_valid = entries.iter().filter(|e| e.is_valid_part_2()).count();
    println!("Number of valid entries part 2: {}", n_valid)
}
