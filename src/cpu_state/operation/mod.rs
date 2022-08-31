use core::panic;

use crate::cpu_state::{Cpu, CpuState};
#[derive(Clone)]
pub enum Operation {
    Add {op1:      String, op2:        String, op3: String},
    Sub {op1:      String, op2:        String, op3: String},
    Mul {op1:      String, op2:        String, op3: String},
    Div {op1:      String, op2:        String, op3: String},
    Ld  {dir:      String, at:         String},
    Str {reg:      String, to:         String},
    Mov {from_reg: String, to_reg:     String},
    Cmp {this:     String, eq_this:    String},
    Jmp {reg_cond: String, pc_to_jump: String},
}

pub trait OperationLogic {
    fn parse_op(line: String) -> Self;
    fn to_string(self) -> String;
    fn print_ln(self);
    fn compute(self, cpu_state: CpuState) -> CpuState;
}

mod private {
    use crate::CpuState;    

    pub fn get_particles(line: &String) -> Vec<&str> {
        let splitted = line.split(' ');
        let particles: Vec<&str> = splitted.collect();

        if particles.len() != 4 
            && *particles.get(0).unwrap() == "add"
            && *particles.get(0).unwrap() == "sub"
            && *particles.get(0).unwrap() == "mul"
            && *particles.get(0).unwrap() == "div" {
            panic!("Line {} is not of 4 elements!", line);
        } else if particles.len() != 3 
            && *particles.get(0).unwrap() == "ld"
            && *particles.get(0).unwrap() == "str"
            && *particles.get(0).unwrap() == "cmp" 
            && *particles.get(0).unwrap() == "jmp"
            && *particles.get(0).unwrap() == "mov"
        {
            panic!("Line {} is not of 3 elements!", line);
        };
        particles
    }

    pub fn check_between_cpu_mem_refs(dir: usize, cpu_state: &CpuState) -> usize {
        if dir <= 11 {
            dir
        } else {
            panic!(
                "Bad cpu memory dir value: {} should be between [1..15] at instruction {}", 
                dir, 
                cpu_state.program_counter + 1
            );
        }
    }

    pub fn get_val(op: String, cpu_state: &CpuState) -> i32 {
        if op.as_bytes()[0]=="r".as_bytes()[0] {
            let reg_val = check_between_cpu_mem_refs(op[1..].parse::<usize>().unwrap(), cpu_state);
            cpu_state.cpu_memory[reg_val]
        } else {
            op.parse::<i32>().unwrap()
        }
    }

    pub fn get_mem_val(op: String, cpu_state: &CpuState) -> usize {
        if op.as_bytes()[0]=="r".as_bytes()[0] {
            op[1..].parse::<usize>().unwrap()
        } else  {
            panic!(
                "Bad cpu memory value format: {}, should be rXX at instruction {}", 
                op,
                cpu_state.program_counter + 1
            );
        }
    }

}

impl OperationLogic for Operation {

    fn parse_op(line: String) -> Operation{

        let particles: Vec<&str> = private::get_particles(&line);

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
            "add" => Operation::Add {op1, op2, op3},
            "sub" => Operation::Sub {op1, op2, op3},
            "mul" => Operation::Mul {op1, op2, op3},
            "div" => Operation::Div {op1, op2, op3},
            "ld"  => Operation::Ld  {dir: op1, at:  op2},
            "str" => Operation::Str {reg: op1, to:  op2},
            "mov" => Operation::Mov { from_reg: op1, to_reg: op2 },
            "cmp" => Operation::Cmp {this: op1, eq_this: op2},
            "jmp" => Operation::Jmp {reg_cond: op1, pc_to_jump: op2},
            &_ => panic!("Not recognised op!: {}", line)
        }
    }

    fn to_string(self) -> String {
        match self {
            Operation::Add {op1, op2, op3}      => format!("add {} {} {}", op1, op2, op3),
            Operation::Sub {op1, op2, op3}      => format!("sub {} {} {}", op1, op2, op3),
            Operation::Mul {op1, op2, op3}      => format!("mul {} {} {}", op1, op2, op3),
            Operation::Div {op1, op2, op3}      => format!("div {} {} {}", op1, op2, op3),
            Operation::Ld  {dir, at }                   => format!("ld  {} {}", dir, at),
            Operation::Str {reg, to }                   => format!("str {} {}", reg, to),
            Operation::Mov {from_reg, to_reg}           => format!("mov {} {}", from_reg, to_reg),
            Operation::Cmp {this, eq_this}              => format!("cmp {} {}", this, eq_this),
            Operation::Jmp {reg_cond, pc_to_jump}       => format!("jmp {} {}", reg_cond, pc_to_jump)
        }
    }

    fn print_ln(self) {
        println!("{}", self.to_string());
    }

    fn compute(self, cpu_state: CpuState) -> CpuState {
        match self {
            Operation::Add {op1, op2, op3}      => 
                cpu_state.clone().alloc(
                    private::get_mem_val(op1, &cpu_state), 
                    private::get_val(op2, &cpu_state) + private::get_val(op3, &cpu_state)
                ),
            Operation::Sub {op1, op2, op3}      => 
                cpu_state.clone().alloc(
                    private::get_mem_val(op1, &cpu_state), 
                    private::get_val(op2, &cpu_state) - private::get_val(op3, &cpu_state)
                ),
            Operation::Mul {op1, op2, op3}      => 
                cpu_state.clone().alloc(
                    private::get_mem_val(op1, &cpu_state),
                    private::get_val(op2, &cpu_state) * private::get_val(op3, &cpu_state)
                ),
            Operation::Div {op1, op2, op3}      => 
                cpu_state.clone().alloc(
                    private::get_mem_val(op1, &cpu_state), 
                    private::get_val(op2, &cpu_state) / private::get_val(op3, &cpu_state)
                ),
            Operation::Ld  {dir, at }      => 
                cpu_state.clone().move_to_cpu_memory(
                    private::get_mem_val(dir, &cpu_state), 
                    at.parse::<usize>().unwrap()
                ),
            Operation::Str {reg, to }      => 
                cpu_state.clone().move_to_main_memory(
                    private::get_mem_val(reg, &cpu_state),
                    to.parse::<usize>().unwrap()
                ),
            Operation::Mov {from_reg, to_reg}      => 
                cpu_state.clone().move_on_cpu_memory(
                    private::get_mem_val(from_reg, &cpu_state),
                    private::get_mem_val(to_reg, &cpu_state)
                ),
            Operation::Cmp {this, eq_this} => {
                let cmp = 
                    if cpu_state.cpu_memory[private::get_mem_val(this.clone(), &cpu_state)] 
                        != eq_this.parse::<i32>().unwrap() 
                    { 0 } else { 1 };
                cpu_state.clone().alloc(
                    private::get_mem_val(this, &cpu_state),
                    cmp
                )
            },
            Operation::Jmp {reg_cond, pc_to_jump}                   => 
                {
                    if cpu_state.cpu_memory[private::get_mem_val(reg_cond, &cpu_state)] == 0 {
                        cpu_state.jump_program_counter(pc_to_jump.parse::<usize>().unwrap())
                    } else {
                        cpu_state.next_program_counter()
                    }
                }
        }
    }

}
