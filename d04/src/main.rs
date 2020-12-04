use regex::Regex;

const EYE_COLORS: [&'static str; 7] = ["amb", "blu", "brn", "gry", "grn", "hzl", "oth"];

struct Passport{
    byr: Option<String>,
    iyr: Option<String>,
    eyr: Option<String>,
    hgt: Option<String>,
    hcl: Option<String>,
    ecl: Option<String>,
    pid: Option<String>,
    _cid: Option<String>
}

impl Passport {
    fn new_from_string(s: &str) -> Self {
        let mut byr: Option<String> = None;
        let mut  iyr: Option<String> = None;
        let mut  eyr: Option<String> = None;
        let mut  hgt: Option<String> = None;
        let mut  hcl: Option<String> = None;
        let mut  ecl: Option<String> = None;
        let mut  pid: Option<String> = None;
        let mut  _cid: Option<String> = None;
        for kvp in s.split(|c| c == '\n' || c == ' ') {
            let mut splitter = kvp.split(':');
            let key = splitter.next().unwrap();
            let val = String::from(splitter.next().unwrap());
            match key {
                "byr" => byr = Some(String::from(val)),
                "iyr" => iyr = Some(String::from(val)),
                "eyr" => eyr = Some(String::from(val)),
                "hgt" => hgt = Some(String::from(val)),
                "ecl" => ecl = Some(String::from(val)),
                "hcl" => hcl = Some(String::from(val)),
                "pid" => pid = Some(String::from(val)),
                "cid" => _cid = Some(String::from(val)),
                _ => panic!("Unknown passport property {}", key)
            }
        }
        Passport{byr, iyr, eyr, hgt, ecl, hcl, pid, _cid}
    }

    fn is_valid_1(&self) -> bool {
        if let Passport{byr: Some(_), iyr: Some(_), eyr: Some(_), hgt: Some(_), ecl: Some(_), hcl: Some(_), pid: Some(_), _cid: _} = self {
            true
        } else {
            false
        }
    }

    fn is_byr_valid(&self) -> bool {
        if let Ok(byr) = (&self.byr).as_ref().unwrap().parse::<usize>() {
            byr >= 1920 && byr <= 2002
        } else {
            return false
        }
    }

    fn is_iyr_valid(&self) -> bool {
        if let Ok(iyr) = (&self.iyr).as_ref().unwrap().parse::<usize>() {
            iyr >= 2010 && iyr <= 2020
        } else {
            return false
        }
    }

    fn is_eyr_valid(&self) -> bool {
        if let Ok(eyr) = (&self.eyr).as_ref().unwrap().parse::<usize>() {
            eyr >= 2020 && eyr <= 2030
        } else {
            return false
        }
    }

    fn is_hgt_valid(&self) -> bool {
        let hgt = (self.hgt.as_ref().unwrap()).as_str();
        if hgt.ends_with("cm") {
            let n = hgt.len() - 2;
            let hgt_num = hgt[..n].parse::<usize>().unwrap();
            hgt_num >= 150 && hgt_num <= 193
        } else if hgt.ends_with("in") {
            let n = hgt.len() - 2;
            let hgt_num = hgt[..n].parse::<usize>().unwrap();
            hgt_num >= 59 && hgt_num <= 76
        } else {
            false
        }
    }

    fn is_hcl_valid(&self) -> bool {
        let hcl = (self.hcl.as_ref().unwrap()).as_str();
        let re = Regex::new(r"^\#(\d|[abcdef]){6}$").unwrap();
        re.is_match(hcl)
    }

    fn is_ecl_valid(&self) -> bool {
        let ecl = (self.ecl.as_ref().unwrap()).as_str();
        EYE_COLORS.iter().any(|ec| *ec == ecl)
    }

    fn is_pid_valid(&self) -> bool {
        let pid = self.pid.as_ref().unwrap();
        let re = Regex::new(r"^\d{9}$").unwrap();
        re.is_match(pid)
    }

    fn is_valid_2(&self) -> bool {
        self.is_valid_1() && self.is_byr_valid() && self.is_iyr_valid() && self.is_eyr_valid() &&
            self.is_hgt_valid() && self.is_ecl_valid() && self.is_hcl_valid() && self.is_pid_valid()
    }
}

fn main() {
    let passports: Vec<_> = include_str!("../input.dat").split("\n\n").map(
        |pp_string| Passport::new_from_string(pp_string)
    ).collect();
    println!("Number of valid passports: {}", passports.iter().filter(|pp| pp.is_valid_2()).count())
}
