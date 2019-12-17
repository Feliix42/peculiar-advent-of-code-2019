use instr::{Instruction, ParameterMode};
use std::fs::File;
use std::io::{self, Read};
use std::path::Path;

mod instr;

#[derive(Clone, Debug)]
pub struct IntProgram {
    data: Vec<i32>,
    initial: Option<(i32, i32)>,
}

impl IntProgram {
    /// Runs an intcode program.
    pub fn run(self) -> i32 {
        let mut data = self.data;

        if let Some((word, noun)) = self.initial {
            data[1] = word;
            data[2] = noun;
        }

        let stdin = io::stdin();

        let mut pos = 0;
        loop {
            // parse the instruction
            let parsed = Instruction::parse(data[pos]);

            match parsed.opcode {
                1 => {
                    // addition
                    let target = data[pos + 3] as usize;
                    let s1 = get_value(
                        &data,
                        pos + 1,
                        parsed.parameter_modes.get(0).unwrap_or_default(),
                    );
                    let s2 = get_value(
                        &data,
                        pos + 2,
                        parsed.parameter_modes.get(1).unwrap_or_default(),
                    );
                    data[target] = s1 + s2;
                    pos += 4;
                }
                2 => {
                    // multiplication
                    let target = data[pos + 3] as usize;
                    let s1 = get_value(
                        &data,
                        pos + 1,
                        parsed.parameter_modes.get(0).unwrap_or_default(),
                    );
                    let s2 = get_value(
                        &data,
                        pos + 2,
                        parsed.parameter_modes.get(1).unwrap_or_default(),
                    );
                    data[target] = s1 * s2;
                    pos += 4;
                }
                3 => {
                    // input
                    let target = data[pos + 1] as usize;
                    let mut inp = String::new();
                    // read a line from stdin
                    stdin.read_line(&mut inp).unwrap();
                    data[target] = inp.trim().parse().unwrap();
                    pos += 2;
                }
                4 => {
                    // output
                    let val = get_value(
                        &data,
                        pos + 1,
                        parsed.parameter_modes.get(pos + 1).unwrap_or_default(),
                    );
                    println!("{}", val);
                    pos += 2;
                }
                5 => {
                    // jump-if-true
                    let cond = get_value(
                        &data,
                        pos+1,
                        parsed.parameter_modes.get(0).unwrap_or_default(),
                    );

                    if cond != 0 {
                        let address = get_value(
                            &data,
                            pos+2,
                            parsed.parameter_modes.get(1).unwrap_or_default(),
                        );

                        pos = address as usize;
                    } else {
                        pos += 3
                    }

                }
                6 => {
                    // jump-if-false
                    let cond = get_value(
                        &data,
                        pos+1,
                        parsed.parameter_modes.get(0).unwrap_or_default(),
                    );

                    if cond == 0 {
                        let address = get_value(
                            &data,
                            pos+2,
                            parsed.parameter_modes.get(1).unwrap_or_default(),
                        );

                        pos = address as usize;
                    } else {
                        pos += 3
                    }
                }
                7 => {
                    // less than
                    let target = data[pos + 3] as usize;
                    let s1 = get_value(
                        &data,
                        pos + 1,
                        parsed.parameter_modes.get(0).unwrap_or_default(),
                    );
                    let s2 = get_value(
                        &data,
                        pos + 2,
                        parsed.parameter_modes.get(1).unwrap_or_default(),
                    );
                    data[target] = if s1 < s2 { 1 } else { 0 };
                    pos += 4;
                }
                8 => {
                    // equals
                    let target = data[pos + 3] as usize;
                    let s1 = get_value(
                        &data,
                        pos + 1,
                        parsed.parameter_modes.get(0).unwrap_or_default(),
                    );
                    let s2 = get_value(
                        &data,
                        pos + 2,
                        parsed.parameter_modes.get(1).unwrap_or_default(),
                    );
                    data[target] = if s1 == s2 { 1 } else { 0 };
                    pos += 4;
                }
                99 => break,
                _ => panic!("Unknown opcode"),
            }
        }

        println!("-------- SYSTEM HALTED --------");

        data[0]
    }

    /// Loads an intcode program from a file.
    pub fn from_file<P: AsRef<Path>>(path: P) -> Self {
        let mut fil = File::open(path).unwrap();

        let mut content = String::new();
        fil.read_to_string(&mut content).unwrap();

        let mut data = Vec::new();
        for item in content.trim().split(",") {
            data.push(item.parse().expect("Could not parse input"));
        }

        Self {
            data,
            initial: None,
        }
    }
}

fn get_value(data: &Vec<i32>, position: usize, mode: &ParameterMode) -> i32 {
    match *mode {
        ParameterMode::Position => data[data[position] as usize],
        ParameterMode::Immediate => data[position],
    }
}
