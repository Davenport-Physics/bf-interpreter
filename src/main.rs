
use clap::Parser;
use std::{self, fs};
use std::io::Read;

#[derive(Parser)]
#[command(version, about, long_about = None)]
struct Args {
    #[clap(short, long)]
    file_name: String
}

fn main() {

    let args = Args::parse();
    let mut interpreter = Interpreter::new(args.file_name);
    interpreter.run();

}

struct Interpreter {
    code: String,
    memory: [u8; 30000],
    pointer: usize,
}

impl Interpreter {

    pub fn new(file_name: String) -> Interpreter {

        let code: String = fs::read_to_string(file_name)
            .expect("Something went wrong reading the file")
            .split("\n")
            .into_iter()
            .map(|x| x.chars())
            .flatten()
            .collect();

        Interpreter {
            code: code,
            memory: [0; 30000],
            pointer: 0
        }

    }

    pub fn run(&mut self) {

        let chars = self.code.chars().collect::<Vec<char>>();
        self.run_loop(chars.as_slice());

        if cfg!(debug_assertions) {
            println!("\n{:?}", self.memory[0..100].to_vec());
        }

    }

    pub fn run_loop(&mut self, chars: &[char]) {

        let mut i = 0;
        let mut input_buffer: [u8; 3] = [0; 3];

        while i < chars.len() {

            match chars[i] {
                '>' => {
                    self.pointer += 1;
                    if self.pointer == 30000 {
                        panic!("Pointer exceeded memory bounds.");
                    }
                },
                '<' => {
                    if let Some(val) = self.pointer.checked_sub(1) {
                        self.pointer = val;
                    } else {
                        panic!("Pointer overflowed on subtraction.");
                    }
                },
                '+' => {
                    if let Some(val) = self.memory[self.pointer].checked_add(1) {
                        self.memory[self.pointer] = val;
                    } else {
                        panic!("Memory overflowed on addition.");
                    }
                },
                '-' => {
                    if let Some(val) = self.memory[self.pointer].checked_sub(1) {
                        self.memory[self.pointer] = val;
                    } else {
                        panic!("Memory overflowed on subtraction.");
                    }
                },
                '.' => print!("{}", self.memory[self.pointer] as char),
                ',' => self.memory[self.pointer] = std::io::stdin().read(&mut input_buffer).unwrap() as u8,
                '[' => {

                    let mut open_brace_count = 1;
                    let mut idx = 0;
                    for j in i+1..chars.len() {
                        
                        if chars[j] == '[' {
                            open_brace_count += 1;
                        } else if chars[j] == ']' {
                            open_brace_count -= 1;
                        }

                        if open_brace_count == 0 {
                            idx = j;
                            break;
                        }

                    }

                    if self.memory[self.pointer] != 0 {
                        self.run_loop(&chars[i+1..=idx]);
                    }

                    i = idx+1;

                },
                ']' => {

                    if self.memory[self.pointer] != 0 {
                        self.run_loop(&chars);
                    } else {
                        break;
                    }

                },
                _ => {}
            }
            i += 1;

        }

    }

}