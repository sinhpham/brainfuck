use std::str::FromStr;
use std::io::Read;
use std::collections::HashMap;

fn main() {
    let ret = run(
">++[<+++++++++++++>-]<[[>+>+<<-]>[<+>-]++++++++
[>++++++++<-]>.[-]<<>++++++++++[>++++++++++[>++
++++++++[>++++++++++[>++++++++++[>++++++++++[>+
+++++++++[-]<-]<-]<-]<-]<-]<-]<-]++++++++++.");

    println!("Program finished, ret = {:?}", ret);
}

#[derive(Debug)]
#[derive(PartialEq)]
enum BfCommand {
    IncreaseDataPointer,
    DecreaseDataPointer,
    IncreaseData,
    DecreaseData,
    Output,
    Input,
    BeginLoop{end_ptr: usize},
    EndLoop{begin_ptr: usize}
}

#[derive(Debug)]
struct BfProgram {
    commands: BfCommands,
    data: Vec<u8>,
    data_ptr: usize,
}

#[derive(Debug)]
struct BfCommands (
    Vec<BfCommand>
);

impl FromStr for BfCommands {
    type Err = String;
    fn from_str(s: &str) -> Result<Self, String> {
        
        let mut open_locations = Vec::<usize>::new();
        let mut brackets = HashMap::<usize, usize>::new();
        let mut idx_offset = 0; // Ignore all non-command characters.
        for (idx, ch) in s.chars().enumerate() {
            match ch {
                '>' | '<' | '+' | '-' | '.' | ',' => (),
                '[' => {
                    open_locations.push(idx - idx_offset);
                },
                ']' => {
                    let open = try!(open_locations.pop().ok_or("Missing opening bracket"));
                    brackets.insert(open, idx - idx_offset);
                    brackets.insert(idx - idx_offset, open);
                },
                _ => idx_offset += 1
            }
        }
        
        if !open_locations.is_empty() {
            return Err("Missing closing bracket".to_string());
        }
        
        idx_offset = 0;
        let commands = s.chars().enumerate().filter_map(|(idx, ch)|
            match ch {
                '>' => Some(BfCommand::IncreaseDataPointer),
                '<' => Some(BfCommand::DecreaseDataPointer),
                '+' => Some(BfCommand::IncreaseData),
                '-' => Some(BfCommand::DecreaseData),
                '.' => Some(BfCommand::Output),
                ',' => Some(BfCommand::Input),
                '[' => Some(BfCommand::BeginLoop{end_ptr: *brackets.get(&(idx - idx_offset)).unwrap()}),
                ']' => Some(BfCommand::EndLoop{begin_ptr: *brackets.get(&(idx - idx_offset)).unwrap()}),
                _ => {
                    idx_offset += 1;
                    None
                }
            }
        ).collect();
        
        return Ok(BfCommands(commands));
    }
}

impl BfProgram {
    fn run(& mut self) {
        let BfCommands(ref commands) = self.commands;
        let mut ins_ptr = 0;
        
        while ins_ptr < commands.len() {
            let curr_command = &commands[ins_ptr];
            // Ensure data.
            while self.data.len() <= self.data_ptr {
                self.data.push(0);
            }
            match *curr_command {
                BfCommand::IncreaseDataPointer => self.data_ptr += 1,
                BfCommand::DecreaseDataPointer => self.data_ptr -= 1,
                BfCommand::IncreaseData => self.data[self.data_ptr] += 1,
                BfCommand::DecreaseData => self.data[self.data_ptr] -= 1,
                BfCommand::Output => {
                    let ch = self.data[self.data_ptr] as char;
                    print!("{}", ch);
                },
                BfCommand::Input => {
                    let input = std::io::stdin()
                        .bytes() 
                        .next()
                        .and_then(|result| result.ok())
                        .map(|byte| byte as u8);
                        
                    self.data[self.data_ptr] = input.unwrap();
                },
                BfCommand::BeginLoop{end_ptr} => {
                    if self.data[self.data_ptr] == 0  {
                        ins_ptr = end_ptr;
                    }
                },
                BfCommand::EndLoop{begin_ptr} => {
                    if self.data[self.data_ptr] != 0  {
                        ins_ptr = begin_ptr;
                    }
                },
            }
            ins_ptr += 1;
        }
    }
}

fn run(input: &str) -> Result<(), String> {
    let commands = try!(BfCommands::from_str(input));
    
    let mut program = BfProgram {
        commands: commands,
        data: Vec::new(),
        data_ptr: 0 };
    
    program.run();
    Ok(())
}