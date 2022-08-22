use std::fs::File;
use std::io::{BufRead};
use std::io::Result;
use std::io::Lines;
use std::io::BufReader;
use std::path::Path;

pub struct Reader{
    file_name: String,
}

impl Reader {
    pub fn new(file_name: String) -> Reader{
        Reader {
            file_name: file_name,
        }
    }

    pub fn read_file_name(args: Vec<String>) -> String {
        let val_file_name =
            match args.get(1){
                Some(name) => name.to_string(),
                None => panic!("File not introduced")
            };
        val_file_name
    }

    fn read_lines<P>(file_name:P) -> Result<Lines<BufReader<File>>>
        where P: AsRef<Path>, {
        let file = File::open(file_name)?;
        Ok(BufReader::new(file).lines())
    }

    pub fn read_file_line_splitted(self) -> Vec<String>{
        let mut lines_read: Vec<String> = Vec::<String>::new();
        if let Ok(lines) = Reader::read_lines(self.file_name){
            for line in lines {
                lines_read.push(line.unwrap());
            }
        }
        lines_read
    }

}