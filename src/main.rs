mod reader;
mod cpu_state;

use std::env;
use reader::{Read, Reader};
use cpu_state::{Cpu, CpuState};

fn main() {

    let args: Vec<String> = env::args().collect();

    let batch_or_debug = 
        match args.clone().pop() {
            Some(ba_or_deb) => ba_or_deb,
            None => "batch".to_string()
        };

    let reader: Reader = Reader::new(Reader::read_file_name(args));

    let program: Vec<String> = reader.read_file_line_splitted();

    let mut cpu: CpuState = CpuState::new(program);

    cpu = cpu.execute_program(batch_or_debug);

    cpu.clone().show_program();

    cpu.clone().show_cpu_memory();

    cpu.show_main_memory();

}