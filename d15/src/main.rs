use std::collections::HashMap;

struct Picker {
    most_recent_ind: usize,
    most_recent: usize,
    less_recent: HashMap<usize, usize>
}

impl Picker {
    fn new(mut start_list: Vec<usize>) -> Self {
        let last_ind = start_list.len() - 1;
        let most_recent = start_list.remove(last_ind);
        let mut most_recent_ind = 0;
        let mut less_recent = HashMap::new();
        for p in start_list {
            less_recent.insert(p, most_recent_ind);
            most_recent_ind += 1
        }
        Picker { most_recent_ind, most_recent, less_recent }
    }

    fn get_n_picked(&self) -> usize {
        self.most_recent_ind + 1
    }

    fn pick(&mut self) -> usize {
        let recent_ind = self.less_recent.get(&self.most_recent);
        let pick = match recent_ind {
            None => 0,
            Some(ind) => self.most_recent_ind - ind
        };
        self.less_recent.insert(self.most_recent, self.most_recent_ind);
        self.most_recent = pick;
        self.most_recent_ind += 1;
        pick
    }
}

fn main() {
    let start_list = vec!(0,12,6,13,20,1,17);
    let mut picker = Picker::new(start_list.clone());

    for _ in picker.get_n_picked()..(2020-1) {
        picker.pick();
    }
    println!("Answer part 1: {}", picker.pick());

    let mut picker = Picker::new(start_list.clone());    
    for _ in picker.get_n_picked()..(30000000-1) {
        picker.pick();
    }
    println!("Answer part 2: {}", picker.pick())
}