use std::{fs, io::Read, process::Command};

type CellSize = i16;

pub fn interpret(code: &str) {
    let mut current_memory_address = 0; // The data pointer
    let mut current_instruction_pointer = 0; // The instruction pointer
    let mut jump_point: Option<usize> = None;

    let mut memory: Vec<CellSize> = vec![0; 30000];

    let instructions = code.chars().into_iter().map(move |c| { c }).collect::<Vec<char>>();
    let end_of_instructions = instructions.len();

    while current_instruction_pointer < end_of_instructions {

        match instructions[current_instruction_pointer] {
            '>' => {
                if current_memory_address == memory.len()-1 {
                    current_memory_address = 0;
                } else {
                    current_memory_address += 1;
                }                
            }
            '<' => { 
                if current_memory_address == 0 {
                    current_memory_address = memory.len()-1;
                } else {
                    current_memory_address -= 1;
                }
            }
            '+' => { memory[current_memory_address] += 1; }
            '-' => { memory[current_memory_address] -= 1; }
            '.' => {
                print!("{}", memory[current_memory_address] as u8 as char); // Unicode Scalar Value
            }
            ',' => { memory[current_memory_address] = std::io::stdin().bytes().next().unwrap().unwrap() as CellSize; }
            '[' => {
                if memory[current_memory_address] != 0 {
                    jump_point = Some(current_instruction_pointer);
                } else {
                    // We have to jump to the matching ]
                    let mut depth = 1;
                    while depth > 0 {
                        current_instruction_pointer += 1;
                        if code.chars().nth(current_instruction_pointer as usize).unwrap() == ']' {
                            depth -= 1;
                        } else if code.chars().nth(current_instruction_pointer as usize).unwrap() == '[' {
                            depth += 1;
                        }
                    }
                }

            },
            ']' => {
                if memory[current_memory_address] != 0 {
                    current_instruction_pointer = jump_point.unwrap();
                }
            }

            'b' => {
                println!("\nDUMPING MEMORY\n{memory:?}\nCurrent Address Pointer: {current_memory_address}\nCurrent Instruction Pointer: {current_instruction_pointer}\nCurrently Pointing to {}",memory[current_memory_address]);
            }

            _ => { /* Ignore the token */ }
        }

        current_instruction_pointer += 1;
    }
}

pub fn complie(code: &str, max_memory_size: usize, cell_size: &str) -> String {
    let mut start = format!("type CellSize = {cell_size};use std::io::Read;
    fn main() {{
        let mut current_memory_address = 0;
        let mut memory: Vec<CellSize> = vec![0; {max_memory_size}];    
    ");

    for (_, token) in code.chars().enumerate() {
        match token {
            '>' => {
                start += "if current_memory_address == memory.len()-1 {
                    current_memory_address = 0;
                } else {
                    current_memory_address += 1;
                }"
            },

            '<' => {
                start += "if current_memory_address == 0 {
                    current_memory_address = memory.len()-1;
                } else {
                    current_memory_address -= 1;
                }";
            },

            '+' => { start += "memory[current_memory_address] += 1;"; },
            '-' => { start += "memory[current_memory_address] -= 1;"; },
            '.' => { start += "print!(\"{}\", memory[current_memory_address] as u8 as char);"; },
            ',' => { start += "memory[current_memory_address] = std::io::stdin().bytes().next().unwrap().unwrap() as CellSize;"; },
            '[' => {
                start += "while memory[current_memory_address] != 0 {";

            },
            ']' => {
                start += "}";
            }            

            _ => { /* Ignore the token */ }
        }
    }

    let finial_code = format!("{start} }}");
    finial_code

}



#[test]
fn hello_world() {

    interpret("
    ++++++++++[>++++++++++<-]>++++. // h - 104
    >++++++++++[>++++++++++<-]>+. // e - 101
    >++++++++++[>++++++++++<-]>++++++++.. // ll - 108
    >++++++++++[>++++++++++<-]>+++++++++++. // o - 111
    >++++++++++++++++[>++<-]>. // (SPACE) - 32
    >++++++++++[>++++++++++<-] // 100 is in the next cell
    +++++++++[>++<-]+ // 19 is in this cell
    > // Enter the cell with 100
    [<+>-] // Add 100 and 19
    <. // Print w - 119
    <<<+. // Print o - 111
    >>>> // Goto the end
    
    // Print r - 114
    ++++++++++[>++++++++++<-]>++++++++++ // Fill the address with 100
    ++++++++++++++. // Fill the address with 14 
    >++++++++++[>++++++++++<-]>++++++++. // l - 108
    > ++++++++++[>++++++++++<-]>. // d - 100
    >++++++++++++++++[>++<-]>+.-. // ! - 33
    ");

}

#[test]
fn complie_helloworld() {
    let code = complie("
    ++++++++++[>++++++++++<-]>++++. // h - 104
    >++++++++++[>++++++++++<-]>+. // e - 101
    >++++++++++[>++++++++++<-]>++++++++.. // ll - 108
    >++++++++++[>++++++++++<-]>+++++++++++. // o - 111
    >++++++++++++++++[>++<-]>. // (SPACE) - 32
    >++++++++++[>++++++++++<-] // 100 is in the next cell
    +++++++++[>++<-]+ // 19 is in this cell
    > // Enter the cell with 100
    [<+>-] // Add 100 and 19
    <. // Print w - 119
    <<<+. // Print o - 111
    >>>> // Goto the end
    
    // Print r - 114
    ++++++++++[>++++++++++<-]>++++++++++ // Fill the address with 100
    ++++++++++++++. // Fill the address with 14 
    >++++++++++[>++++++++++<-]>++++++++. // l - 108
    > ++++++++++[>++++++++++<-]>. // d - 100
    >++++++++++++++++[>++<-]>+.-. // ! - 33
    ", 100, "i16");

    fs::write("./tmp.rs", code).unwrap();
    Command::new("rustc").args(["./tmp.rs"]).output().unwrap();
}