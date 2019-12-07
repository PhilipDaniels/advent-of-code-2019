use std::str::FromStr;
use std::error::Error;
use std::fmt;

#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum Opcode {
    Add = 1,
    Multiply = 2,
    ReadInput = 3,
    WriteOutput = 4,
    Halt = 99,
}

/// The two different modes that an instruction parameter can have.
#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum ParameterMode {
    /// The parameter is to be interpreted as a position - if the parameter is 50,
    /// its value is the value stored at address 50 in memory.
    Position,
    /// The parameter is to be interpreted as a value - if the parameter is 50,
    /// its value is simply 50.
    Immediate,
}

/// Represents an instruction as decoded from the input.
#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Instruction {
    opcode: Opcode,
    parameter1: Option<ParameterMode>,
    parameter2: Option<ParameterMode>,
    parameter3: Option<ParameterMode>,
}

impl FromStr for Instruction {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s.is_empty() {
            return Err("Empty instruction".to_string());
        }

        // Instructions are always 1..5 characters in length.
        let mut padded: [u8; 5] = *b"00000";
        let s = s.as_bytes();
        //println!("s = {:?}", s);
        padded[5 - s.len()..].copy_from_slice(&s[..]);
        //println!("Padded = {:?}", padded);

        let opcode_slice = &padded[3..];
        let opcode = match opcode_slice {
             b"01" => Opcode::Add,
             b"02" => Opcode::Multiply,
             b"03" => Opcode::ReadInput,
             b"04" => Opcode::WriteOutput,
             b"99" => Opcode::Halt,
             _ => return Err(format!("Unknown opcode {:?}", opcode_slice)),
         };

        Ok(
            Instruction {
                opcode: opcode,
                parameter1: None,
                parameter2: None,
                parameter3: None,
            }
        )
    }
}

impl Instruction {
    pub fn ip_inc(&self) -> u8 {
        if self.opcode == Opcode::Halt {
            return 0;
        }

        if self.parameter3.is_some() {
            return 4;
        } else if self.parameter2.is_some() {
            return 3;
        } else if self.parameter1.is_some() {
            return 2;
        }

        1
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    pub fn from_str_empty() {
        let result = Instruction::from_str("");
        assert_eq!(result, Err("Empty instruction".into()));
    }

    #[test]
    pub fn from_str_halt() {
        let result = Instruction::from_str("99").unwrap();
        assert_eq!(result.opcode, Opcode::Halt);
        assert!(result.parameter1.is_none());
        assert!(result.parameter2.is_none());
        assert!(result.parameter3.is_none());
        assert_eq!(result.ip_inc(), 0);
    }
}

pub type ComputerResult<T> = Result<T, Box<dyn Error>>;

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum ComputerError {
    BadInstruction(&'static str),
}

impl ComputerError {
    // fn as_str(&self) -> &str {
    //     match *self {
    //         ComputerError::BadInstruction(e) => e.as_str(),
    //     }
    // }
}

impl Error for ComputerError {
    fn description(&self) -> &str {
        match *self {
            ComputerError::BadInstruction(ref e) => e,
        }
    }
}

impl fmt::Display for ComputerError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            ComputerError::BadInstruction(ref e) => e.fmt(f),
        }
    }
}
