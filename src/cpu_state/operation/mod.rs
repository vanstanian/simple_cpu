use core::panic;

use crate::cpu_state::CpuState;
#[derive(Clone)]
pub enum Operation {
    Add {op1: String, op2: String, op3: String},
    Sub {op1: String, op2: String, op3: String},
    Mul {op1: String, op2: String, op3: String},
    Div {op1: String, op2: String, op3: String},
    Ld  {dir: String, at: String},
    Str {reg: String, to: String},
    Cmp {this:String, eq_this: String},
    Jmp {dir: String},
}

impl Operation {

    pub fn parse_op(line: String) -> Operation{

        let particles: Vec<&str> = Self::get_particles(&line);

        let op1: String = 
            match particles.get(1) {
                Some(op) => op.to_string(),
                None => "0".to_string()
            }; 
        let op2: String = 
            match particles.get(2) {
                Some(op) => op.to_string(),
                None => "0".to_string()
            };
        let op3: String = 
            match particles.get(3) {
                Some(op) => op.to_string(),
                None => "0".to_string()
            };

        match particles.get(0).unwrap().to_string().as_str() {
            "add" => Operation::Add {op1: op1, op2: op2, op3: op3},
            "sub" => Operation::Sub {op1: op1, op2: op2, op3: op3},
            "mul" => Operation::Mul {op1: op1, op2: op2, op3: op3},
            "div" => Operation::Div {op1: op1, op2: op2, op3: op3},
            "ld"  => Operation::Ld  {dir: op1, at:  op2},
            "str" => Operation::Str {reg: op1, to:  op2},
            "cmp" => Operation::Cmp {this: op1, eq_this: op2},
            "jmp" => Operation::Jmp {dir: op1},
            &_ => panic!("Not recognised op!: {}", line)
        }
    }

    pub fn print_ln(self){
        match self {
            Operation::Add {op1, op2, op3}      => println!("add {} {} {}", op1, op2, op3),
            Operation::Sub {op1, op2, op3}      => println!("sub {} {} {}", op1, op2, op3),
            Operation::Mul {op1, op2, op3}      => println!("mul {} {} {}", op1, op2, op3),
            Operation::Div {op1, op2, op3}      => println!("div {} {} {}", op1, op2, op3),
            Operation::Ld  {dir, at }      => println!("ld  {} {}", dir, at),
            Operation::Str {reg, to }      => println!("str {} {}", reg, to),
            Operation::Cmp {this, eq_this} => println!("cmp {} {}", this, eq_this),
            Operation::Jmp {dir}                   => println!("jmp {}", dir)
        };
    }

    fn get_particles(line: &String) -> Vec<&str> {
        let splitted = line.split(" ");
        let particles: Vec<&str> = splitted.collect();

        if particles.len() != 4 
            && particles.get(0).unwrap().to_string() == "add"
            && particles.get(0).unwrap().to_string() == "sub"
            && particles.get(0).unwrap().to_string() == "mul"
            && particles.get(0).unwrap().to_string() == "div" {
            panic!("Line {} is not of 3 elements!", line);
        } else if particles.len() != 3 
            && particles.get(0).unwrap().to_string() == "ld"
            && particles.get(0).unwrap().to_string() == "str"
            && particles.get(0).unwrap().to_string() == "cmp" {
            panic!("Line {} is not of 3 elements!", line);
        } else if particles.len() != 2 
            && particles.get(0).unwrap().to_string() == "jmp"{
            panic!("Line {} is not of 2 elements!", line);
        };
        particles
    }

    fn get_val(op: String, cpu_state: &CpuState) -> i32 {
        if op.clone().as_bytes()[0]=="r".as_bytes()[0] {
            let reg_val = Self::check_between_cpu_mem_refs(op[1..].parse::<usize>().unwrap());
            cpu_state.clone().cpu_memory[reg_val]
        } else {
            op.clone().parse::<i32>().unwrap()
        }
    }

    fn get_mem_val(op: String) -> usize {
        if op.clone().as_bytes()[0]=="r".as_bytes()[0] {
            op[1..].parse::<usize>().unwrap()
        } else  {
            panic!("Bad cpu memory value format: {}, should be rXX", op);
        }
    }

    fn check_between_cpu_mem_refs(dir: usize) -> usize {
        if dir >= 0 && dir <= 11 {
            dir
        } else {
            panic!("Bad cpu memory dir value: {} should be between [1..15]", dir);
        }
    }

    pub fn compute(self, cpu_state: CpuState) -> CpuState {
        match self {
            Operation::Add {op1, op2, op3}      => 
                cpu_state.clone().alloc(
                    Self::get_mem_val(op1), 
                    Self::get_val(op2, &cpu_state) + Self::get_val(op3, &cpu_state)
                ),
            Operation::Sub {op1, op2, op3}      => 
                cpu_state.clone().alloc(
                    Self::get_mem_val(op1), 
                    Self::get_val(op2, &cpu_state) - Self::get_val(op3, &cpu_state)
                ),
            Operation::Mul {op1, op2, op3}      => 
                cpu_state.clone().alloc(
                    Self::get_mem_val(op1),
                    Self::get_val(op2, &cpu_state) * Self::get_val(op3, &cpu_state)
                ),
            Operation::Div {op1, op2, op3}      => 
                cpu_state.clone().alloc(
                    Self::get_mem_val(op1), 
                    Self::get_val(op2, &cpu_state) / Self::get_val(op3, &cpu_state)
                ),
            Operation::Ld  {dir, at }      => 
                cpu_state.clone().move_to_cpu_memory(
                    Self::get_mem_val(dir), 
                    at.parse::<usize>().unwrap()
                ),
            Operation::Str {reg, to }      => 
                cpu_state.clone().move_to_main_memory(
                    Self::get_mem_val(reg),
                    to.parse::<usize>().unwrap()
                ),
            Operation::Cmp {this, eq_this} => {println!("Cmp not implemented yet!"); cpu_state},
            Operation::Jmp {dir}                   => {println!("Jmp not implemented yet!"); cpu_state},
            _ => panic!("Operation not recognised!!")
        }
    }

}
