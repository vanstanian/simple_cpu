mod operation;

use operation::Operation;
use std::io::stdin;


#[derive(Clone)]
pub struct CpuState {
    cpu_memory: [i32; 12],
    main_memory: [i32; 48],
    program: Vec<Operation>,
}

impl CpuState {
    pub fn new(program: Vec<String>) -> CpuState {
        let cpu_memory: [i32; 12] = [0,0,0,0,0,0,0,0,0,0,0,0];
        let main_memory: [i32; 48] = [0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0];
        CpuState {
            cpu_memory,
            main_memory,
            program: Self::gen_program_vec(&program)
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

    pub fn show_cpu_memory(self){
        println!("Cpu Memory: {:?}", self.cpu_memory);
    }

    pub fn show_main_memory(self){
        println!("Main Memory: {:?}", self.main_memory);
    }

    pub fn show_program(self){
        for i in self.program {
            i.print_ln();
        }
    }

    pub fn read_program(self) -> Vec<Operation> {
        self.program
    }

    pub fn execute_program(self, batch_or_debug: String) -> CpuState {
        let mut cpu = self;
        for i in cpu.clone().read_program(){
            cpu = i.clone().compute(cpu.clone());
            if batch_or_debug == "debug" {
                i.print_ln();
                cpu.clone().show_cpu_memory();
                cpu.clone().show_main_memory();
                let mut input_str = String::new();
                
                stdin()
                .read_line(&mut input_str)
                .ok()
                .expect("Failed to read line");

            }
        };
        cpu
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