mod operation;

use operation::Operation;
use std::io::stdin;


#[derive(Clone)]
pub struct CpuState {
    cpu_memory: [i32; 12],
    main_memory: [i32; 48],
    program: Vec<Operation>,
    program_counter: usize
}

impl CpuState {
    pub fn new(program: Vec<String>) -> CpuState {
        let cpu_memory: [i32; 12] = [0,0,0,0,0,0,0,0,0,0,0,0];
        let main_memory: [i32; 48] = [0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0];
        let program_counter: usize = 0;
        CpuState {
            cpu_memory,
            main_memory,
            program: Self::gen_program_vec(&program),
            program_counter
        }
    }

    fn new_interal(
        cpu_memory: [i32; 12],
        main_memory: [i32; 48],
        program: Vec<Operation>,
        program_counter: usize) 
    -> CpuState {
        CpuState {
            cpu_memory,
            main_memory,
            program,
            program_counter
        }
    }

    pub fn alloc(mut self, dir: usize, val: i32) -> CpuState{
        self.cpu_memory[dir] = val;
        self
    }

    pub fn move_to_main_memory(mut self, reg: usize, to: usize) -> CpuState {
        self.main_memory[to] = self.clone().cpu_memory[reg];
        self
    }

    pub fn move_to_cpu_memory(mut self, dir: usize, at: usize) -> CpuState {
        self.cpu_memory[at] = self.clone().main_memory[dir];
        self
    }

    pub fn move_on_cpu_memory(mut self, from: usize, to: usize) -> CpuState {
        self.cpu_memory[to] = self.clone().cpu_memory[from];
        self
    }

    pub fn show_cpu_memory(self){
        println!("Cpu Memory: {:?}", self.cpu_memory);
    }

    pub fn show_main_memory(self){
        println!("Main Memory: {:?}", self.main_memory);
    }

    pub fn show_program(self){
        let mut ins_num = 0;
        for i in self.program {
            println!("{}: {}", ins_num, i.to_string());
            ins_num = ins_num + 1;
        }
    }

    pub fn execute_program(self, batch_or_debug: String) -> CpuState {
        let mut cpu = self.clone();
        
        while cpu.clone().program_counter.clone() < cpu.clone().program.len() {
            
            let instruction = 
                cpu.clone()
                .program.clone()
                .get(cpu.clone().program_counter.clone()).unwrap().to_owned();
            cpu = instruction.clone().compute(cpu.clone());
            
            match instruction.clone() {
                Operation::Jmp { reg_cond: _, pc_to_jump: _} => {},
                _ => {cpu = Self::next_program_counter(cpu.clone());}
            };

            if batch_or_debug == "debug" {
            
                instruction.clone().print_ln();
                cpu.clone().show_cpu_memory();
                cpu.clone().show_main_memory();
                if cpu.clone().program_counter < cpu.clone().program.clone().len(){
                    println!("Next program counter: {}", cpu.clone().program_counter);
                } else {
                    println!("Ended!!");
                }
                let mut input_str = String::new();
                cpu.clone().show_program();
                
                stdin()
                .read_line(&mut input_str)
                .ok()
                .expect("Failed to read line");

            };

        };
        cpu
    }

    pub fn next_program_counter (self) -> CpuState {
        let cpu_program_counter = self.clone().program_counter + 1;
        CpuState::new_interal(self.clone().cpu_memory, self.clone().main_memory, self.clone().program, cpu_program_counter)
    }

    pub fn jump_program_counter (self, new_program_counter: usize) -> CpuState {
        CpuState::new_interal(self.clone().cpu_memory, self.clone().main_memory, self.clone().program, new_program_counter)
    }

    fn gen_program_vec(reading: &Vec<String>) -> Vec<Operation>{
        let mut vector = Vec::<Operation>::new();
        for i in reading {
            let operation: Operation = Operation::parse_op(i.to_string());
            vector.push(operation);
        };
        vector
    }

}