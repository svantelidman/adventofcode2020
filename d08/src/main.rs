use std::collections::HashSet;

#[derive(Debug, Clone)]
enum OpCode{
    Nop,
    Acc,
    Jmp
}

impl From<&str> for OpCode {
    fn from(s: &str) -> OpCode{
        match s {
            "acc" => OpCode::Acc,
            "jmp" => OpCode::Jmp,
            _ => OpCode::Nop,
        }
    }
}

#[derive(Debug, Clone)]
struct Operation {
    op_code: OpCode,
    param: i64
}

impl Operation {
    fn new(s: &str) -> Operation {
        let mut splitter = s.split(' ');
        let op_code = OpCode::from(splitter.next().unwrap());
        let param = splitter.next().unwrap().parse::<i64>().unwrap();
        Operation{op_code, param}
    }
}

enum TerminationCode {
    Normal,
    InfiniteLoop
}

fn execute(prog: &Vec<Operation>) -> (TerminationCode, i64) {
    let mut visited: HashSet<i64> = HashSet::new();
    let mut acc: i64 = 0;
    let mut instruction_ptr: i64 = 0;
    let mut exit = false;
    while !visited.contains(&instruction_ptr) && !exit {
        visited.insert(instruction_ptr);
        match prog[instruction_ptr as usize].op_code {
            OpCode::Nop => instruction_ptr += 1,
            OpCode::Acc => {acc += prog[instruction_ptr as usize].param; instruction_ptr += 1},
            OpCode::Jmp => { instruction_ptr += prog[instruction_ptr as usize].param}
        }
        exit = instruction_ptr == prog.len() as i64;
    }
    let termination_code = if exit {
        TerminationCode::Normal
    } else {
        TerminationCode::InfiniteLoop
    };
    (termination_code, acc)
}

fn part_1(prog: &Vec<Operation>) {
    let (_, acc) = execute(prog);
    println!("Accumuluator value part 1 = {}", acc)
}

fn part_2(original_prog: &Vec<Operation>) {
    for mod_ind in 0..original_prog.len() {
        let mut prog = original_prog.clone();
        match prog[mod_ind].op_code {
            OpCode::Nop => prog[mod_ind] = Operation{op_code: OpCode::Jmp, param: prog[mod_ind].param},
            OpCode::Jmp => prog[mod_ind] =  Operation{op_code: OpCode::Nop, param: prog[mod_ind].param},
            _ => ()
        };
        if let (TerminationCode::Normal, acc) = execute(&prog) {
            println!("Accumuluator value part 2 = {}", acc);
            break
        }
    }
}

fn main() {
    let prog: Vec<_> = include_str!("../input.dat").split('\n').map(
        |s| Operation::new(s)
    ).collect();
    part_1(&prog);
    part_2(&prog)
}
