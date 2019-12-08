/// The two different modes that an instruction parameter can have.
#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum ParameterMode {
    /// The parameter is to be interpreted as a position - if the parameter is 50,
    /// its value is the value stored at address 50 in memory.
    Position = 0,
    /// The parameter is to be interpreted as a value - if the parameter is 50,
    /// its value is simply 50.
    Immediate = 1,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
/// A machine instruction.
pub enum Instruction {
    // An Add instruction. Valid modes are Either, Either, Position.
    // Adds its first two operands and stores the result in the third.
    Add(ParameterMode, ParameterMode, ParameterMode),
    // A Multiply instruction. Valid modes are Either, Either, Position.
    // Multiplies its first two operands and stores the result in the third.
    Multiply(ParameterMode, ParameterMode, ParameterMode),
    // A Read instruction. Valid mode is Position.
    // Reads a value from stdin and stores it in the address
    // pointed to by its parameter.
    Read(ParameterMode),
    // A Write instruction. Valid mode is Either.
    // Writes the value of its parameter to stdout.
    Write(ParameterMode),
    // A Halt instruction. Stops the computer. No valid parameters.
    Halt
}

impl Instruction {
    /// Returns the increment to be applied to the instruction pointer
    /// after processing this instruction.
    pub fn instruction_pointer_increment(&self) -> u8 {
        match *self {
            Instruction::Add(..) => 3,
            Instruction::Multiply(..) => 3,
            Instruction::Read(..) => 2,
            Instruction::Write(..) => 2,
            Instruction::Halt => panic!("Do not call instruction_pointer_increment() for Halt instructions"),
        }
    }

    /// Decodes an instruction from a raw integer.
    pub fn decode(inst: i32) -> Result<Instruction, String> {
        use Instruction::*;

        // Include this as a sanity check so we don't start allowing
        // things like -3 to be valid instructions.
        if inst < 1 || inst > 99_999 {
            return Err(format!("Bad instruction {}, out of range", inst));
        }

        // The opcode is in the rightmost two digits, which we can extract
        // by using the remainder operator.
        let opcode = match inst % 100 {
            n @ 1..=4 => n,
            99 => 99,
            _ => return Err(format!("Bad instruction {}, opcode not valid", inst))
        };

        // Now pull out the parameter modes, being careful to *only* accept
        // valid instructions (by looking at the whole input). This way we will
        // get an early warning if our program goes wrong and starts to
        // write junk into the wrong addresses.
        let instruction = match opcode {
            1 => {
                let p1 = decode_parameter_mode(inst, ParameterNumber::One, AllowedParameterMode::Either)?;
                let p2 = decode_parameter_mode(inst, ParameterNumber::Two, AllowedParameterMode::Either)?;
                let p3 = decode_parameter_mode(inst, ParameterNumber::Three, AllowedParameterMode::Position)?;
                if inst / 10000 > 0 {
                    return Err(format!("Invalid instruction {}, superfluous digits", inst));
                }
                Add(p1, p2, p3)
            },

            2 => {
                let p1 = decode_parameter_mode(inst, ParameterNumber::One, AllowedParameterMode::Either)?;
                let p2 = decode_parameter_mode(inst, ParameterNumber::Two, AllowedParameterMode::Either)?;
                let p3 = decode_parameter_mode(inst, ParameterNumber::Three, AllowedParameterMode::Position)?;
                if inst / 10000 > 0 {
                    return Err(format!("Invalid instruction {}, superfluous digits", inst));
                }
                Multiply(p1, p2, p3)
            },

            3 => {
                let p1 = decode_parameter_mode(inst, ParameterNumber::One, AllowedParameterMode::Position)?;
                if inst / 100 > 0 {
                    return Err(format!("Invalid instruction {}, superfluous digits", inst));
                }
                Read(p1)
            },

            4 => {
                let p1 = decode_parameter_mode(inst, ParameterNumber::One, AllowedParameterMode::Either)?;
                if inst / 100 > 1 {
                    return Err(format!("Invalid instruction {}, superfluous digits", inst));
                }
                Write(p1)
            },

            99 => {
                if inst / 100 > 0 {
                    return Err(format!("Invalid instruction {}, superfluous digits", inst));
                }
                Halt
            },
            _ => unreachable!("Bad opcode should be returned as error in previous match")
        };

        Ok(instruction)
    }

}

#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
enum AllowedParameterMode {
    Position,
    Immediate,
    Either,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
enum ParameterNumber {
    One,
    Two,
    Three
}

/// Helper function to pull out a single parameter mode.
fn decode_parameter_mode(inst: i32, prm_num: ParameterNumber, allowed: AllowedParameterMode) -> Result<ParameterMode, String> {
    use self::ParameterMode::*;

    let i = inst / match prm_num {
        ParameterNumber::One => 100,
        ParameterNumber::Two => 1000,
        ParameterNumber::Three => 10000,
    };

    let mode = i % 10;

    //println!("inst = {}, i = {}, mode = {}, prm_num = {:?}, allowed = {:?}", inst, i, mode, prm_num, allowed);

    match mode {
        0 => {
            if allowed == AllowedParameterMode::Position || allowed == AllowedParameterMode::Either {
                Ok(Position)
            } else {
                Err(format!(
                    "In instruction {}, found parameter mode {}, which does not comply with the allowed mode {:?}",
                    inst, mode, allowed))
            }
        },
        1 => {
            if allowed == AllowedParameterMode::Immediate || allowed == AllowedParameterMode::Either {
                Ok(Immediate)
            } else {
                Err(format!(
                    "In instruction {}, found parameter mode {}, which does not comply with the allowed mode {:?}",
                    inst, mode, allowed))
            }
        },
        _ => Err(format!("In instruction {}, found invalid parameter mode {}", inst, mode))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use super::ParameterMode::*;
    use super::Instruction::*;

    #[test]
    fn decode_integers_too_small() {
        assert!(Instruction::decode(0).is_err());
        assert!(Instruction::decode(-1).is_err());
    }

    #[test]
    pub fn decode_unknown_opcode() {
        assert!(Instruction::decode(5).is_err());
        assert!(Instruction::decode(98).is_err());
    }

    #[test]
    pub fn decode_halt() {
        assert_eq!(Instruction::decode(99), Ok(Halt));
        assert!(Instruction::decode(199).is_err());
        assert!(Instruction::decode(1099).is_err());
        assert!(Instruction::decode(10099).is_err());
    }

    #[test]
    pub fn decode_add() {
        assert_eq!(Instruction::decode(1).unwrap(), Add(Position, Position, Position));
        assert_eq!(Instruction::decode(101).unwrap(), Add(Immediate, Position, Position));
        assert_eq!(Instruction::decode(1001).unwrap(), Add(Position, Immediate, Position));
        assert!(Instruction::decode(10001).is_err(), "Write prm must be Position mode");
        // Bad position modes, not repeated for other instructions.
        assert!(Instruction::decode(201).is_err());
        assert!(Instruction::decode(2001).is_err());
        assert!(Instruction::decode(20001).is_err());
    }

    #[test]
    pub fn decode_multiply() {
        assert_eq!(Instruction::decode(2).unwrap(), Multiply(Position, Position, Position));
        assert_eq!(Instruction::decode(102).unwrap(), Multiply(Immediate, Position, Position));
        assert_eq!(Instruction::decode(1002).unwrap(), Multiply(Position, Immediate, Position));
        assert!(Instruction::decode(10002).is_err(), "Write prm must be Position mode");
    }

    #[test]
    pub fn decode_read() {
        assert_eq!(Instruction::decode(3).unwrap(), Read(Position));
        assert!(Instruction::decode(103).is_err(), "Write prm must be Position mode");
    }

    #[test]
    pub fn decode_write() {
        assert_eq!(Instruction::decode(4).unwrap(), Write(Position));
        assert_eq!(Instruction::decode(104).unwrap(), Write(Immediate));
        assert!(Instruction::decode(1004).is_err(), "Too many digits");
    }
}
