use std::default::Default;

pub enum ParameterMode {
    Position,
    Immediate,
}

impl Default for &ParameterMode {
    fn default() -> Self {
        &ParameterMode::Position
    }
}

/// An intcode computer instruction.
pub struct Instruction {
    pub opcode: usize,
    pub parameter_modes: Vec<ParameterMode>,
}

// TODO: Impl default für ParameterMode?

// TODO: Error class?

impl Instruction {
    /// Function to parse an instruction.
    pub fn parse(instruction: i32) -> Self {
        let mut stringified = instruction.to_string();
        let instr_len = stringified.len();
        if instr_len <= 2 {
            return Self {
                opcode: instruction as usize,
                parameter_modes: Vec::new(),
            };
        }

        let op = stringified.split_off(instr_len - 2).parse().unwrap();
        // parse parameter modes

        let mut params = Vec::new();

        for ch in stringified.chars().rev() {
            match &ch {
                '0' => params.push(ParameterMode::Position),
                '1' => params.push(ParameterMode::Immediate),
                m => panic!(
                    "Invalid parameter mode {} (instruction: {})",
                    m, instruction
                ),
            }
        }

        Self {
            opcode: op,
            parameter_modes: params,
        }
    }
}
