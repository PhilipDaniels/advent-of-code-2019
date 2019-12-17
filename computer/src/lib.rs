use bitflags::bitflags;

/// The two different modes that an instruction parameter can have.
#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum ParameterMode {
    /// The parameter is to be interpreted as a position - if the parameter is 50,
    /// its value is the value stored at address 50 in memory.
    Position = 0,
    /// The parameter is to be interpreted as a value - if the parameter is 50,
    /// its value is simply 50.
    Immediate = 1,
    /// The parameter is to be interpreted as a position relative to the
    /// current value of the computer's Relative Base Offset register.
    Relative = 2,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
/// A machine instruction.
pub enum Instruction {
    /// An Add instruction. Valid modes are Either, Either, Position.
    /// Adds its first two operands and stores the result in the third.
    Add(ParameterMode, ParameterMode, ParameterMode),
    /// A Multiply instruction. Valid modes are Either, Either, Position.
    /// Multiplies its first two operands and stores the result in the third.
    Multiply(ParameterMode, ParameterMode, ParameterMode),
    /// A Read instruction. Valid mode is Position.
    /// Reads a value from stdin and stores it in the address
    /// pointed to by its parameter.
    Read(ParameterMode),
    /// A Write instruction. Valid mode is Either.
    /// Writes the value of its parameter to stdout.
    Write(ParameterMode),
    /// Jump if true instruction. Valid modes are Either, Either.
    /// If the first parameter is non-zero, it sets the instruction pointer
    /// to the value from the second parameter. Otherwise, it does nothing.
    JumpIfTrue(ParameterMode, ParameterMode),
    /// Jump if false instruction. Valid modes are Either, Either.
    /// If the first parameter is zero, it sets the instruction pointer to the
    /// value from the second parameter. Otherwise, it does nothing.
    JumpIfFalse(ParameterMode, ParameterMode),
    /// Less Than instruction. Valid modes are Either, Either, Position.
    /// If the first parameter is less than the second parameter, it stores 1
    /// in the position given by the third parameter. Otherwise, it stores 0.
    LessThan(ParameterMode, ParameterMode, ParameterMode),
    /// Equals instruction. Valid modes are Either, Either, Position.
    /// If the first parameter is equal to the second parameter, it stores 1
    /// in the position given by the third parameter. Otherwise, it stores 0.
    Equals(ParameterMode, ParameterMode, ParameterMode),
    /// Relative Base Offset instruction. Valid mode is Immediate (I think).
    /// Adds the value of the parameter to the computer's current Relative Base
    /// Offset register.
    RelativeBaseOffset(ParameterMode),
    /// A Halt instruction. Stops the computer. No valid parameters.
    Halt
}

impl Instruction {
    /// Returns the increment to be applied to the instruction pointer
    /// after processing this instruction.
    pub fn instruction_pointer_increment(&self) -> usize {
        match *self {
            Instruction::Add(..) => 4,
            Instruction::Multiply(..) => 4,
            Instruction::Read(..) => 2,
            Instruction::Write(..) => 2,
            Instruction::JumpIfTrue(..) => 3,   // In the case where it does nothing.
            Instruction::JumpIfFalse(..) => 3,  // In the case where it does nothing.
            Instruction::LessThan(..) => 4,
            Instruction::Equals(..) => 4,
            Instruction::RelativeBaseOffset(..) => 2,
            Instruction::Halt => panic!("Do not call instruction_pointer_increment() for Halt instructions"),
        }
    }

    /// Decodes an instruction from a raw integer.
    pub fn decode(inst: i64) -> Result<Instruction, String> {
        use Instruction::*;

        // Include this as a sanity check so we don't start allowing
        // things like -3 to be valid instructions.
        if inst < 1 || inst > 99_999 {
            return Err(format!("Bad instruction {}, out of range", inst));
        }

        // The opcode is in the rightmost two digits, which we can extract
        // by using the remainder operator.
        let opcode = match inst % 100 {
            n @ 1..=9 => n,
            99 => 99,
            _ => return Err(format!("Bad instruction {}, opcode not valid", inst))
        };

        // Now pull out the parameter modes, being careful to *only* accept
        // valid instructions (by looking at the whole input). This way we will
        // get an early warning if our program goes wrong and starts to
        // write junk into the wrong addresses.
        let instruction = match opcode {
            1 => {
                let p1 = decode_parameter_mode2(inst, ParameterNumber::One, ModeFlags::ANY)?;
                let p2 = decode_parameter_mode2(inst, ParameterNumber::Two, ModeFlags::ANY)?;
                let p3 = decode_parameter_mode2(inst, ParameterNumber::Three, ModeFlags::POS_OR_REL)?;
                if inst / 10000 > 0 {
                    return Err(format!("Invalid instruction {}, superfluous digits", inst));
                }
                Add(p1, p2, p3)
            },

            2 => {
                let p1 = decode_parameter_mode2(inst, ParameterNumber::One, ModeFlags::ANY)?;
                let p2 = decode_parameter_mode2(inst, ParameterNumber::Two, ModeFlags::ANY)?;
                let p3 = decode_parameter_mode2(inst, ParameterNumber::Three, ModeFlags::POS_OR_REL)?;
                if inst / 10000 > 0 {
                    return Err(format!("Invalid instruction {}, superfluous digits", inst));
                }
                Multiply(p1, p2, p3)
            },

            3 => {
                let p1 = decode_parameter_mode2(inst, ParameterNumber::One, ModeFlags::POS_OR_REL)?;
                if inst / 100 > 0 {
                    return Err(format!("Invalid instruction {}, superfluous digits", inst));
                }
                Read(p1)
            },

            4 => {
                let p1 = decode_parameter_mode2(inst, ParameterNumber::One, ModeFlags::ANY)?;
                if inst / 100 > 2 {
                    return Err(format!("Invalid instruction {}, superfluous digits", inst));
                }
                Write(p1)
            },

            5 => {
                let p1 = decode_parameter_mode2(inst, ParameterNumber::One, ModeFlags::ANY)?;
                let p2 = decode_parameter_mode2(inst, ParameterNumber::Two, ModeFlags::ANY)?;
                if inst / 10000 > 0 {
                    return Err(format!("Invalid instruction {}, superfluous digits", inst));
                }
                JumpIfTrue(p1, p2)
            },

            6 => {
                let p1 = decode_parameter_mode2(inst, ParameterNumber::One, ModeFlags::ANY)?;
                let p2 = decode_parameter_mode2(inst, ParameterNumber::Two, ModeFlags::ANY)?;
                if inst / 10000 > 0 {
                    return Err(format!("Invalid instruction {}, superfluous digits", inst));
                }
                JumpIfFalse(p1, p2)
            },

            7 => {
                let p1 = decode_parameter_mode2(inst, ParameterNumber::One, ModeFlags::ANY)?;
                let p2 = decode_parameter_mode2(inst, ParameterNumber::Two, ModeFlags::ANY)?;
                let p3 = decode_parameter_mode2(inst, ParameterNumber::Three, ModeFlags::POS_OR_REL)?;
                if inst / 10000 > 0 {
                    return Err(format!("Invalid instruction {}, superfluous digits", inst));
                }
                LessThan(p1, p2, p3)
            },

            8 => {
                let p1 = decode_parameter_mode2(inst, ParameterNumber::One, ModeFlags::ANY)?;
                let p2 = decode_parameter_mode2(inst, ParameterNumber::Two, ModeFlags::ANY)?;
                let p3 = decode_parameter_mode2(inst, ParameterNumber::Three, ModeFlags::POS_OR_REL)?;
                if inst / 10000 > 0 {
                    return Err(format!("Invalid instruction {}, superfluous digits", inst));
                }
                Equals(p1, p2, p3)
            },

            9 => {
                let p1 = decode_parameter_mode2(inst, ParameterNumber::One, ModeFlags::ANY)?;
                if inst / 100 > 2 {
                    return Err(format!("Invalid instruction {}, superfluous digits", inst));
                }
                RelativeBaseOffset(p1)
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

bitflags! {
    struct ModeFlags: u32 {
        const POSITION = 0b00000001;
        const IMMEDIATE = 0b00000010;
        const RELATIVE = 0b00000100;
        const ANY = Self::POSITION.bits | Self::IMMEDIATE.bits | Self::RELATIVE.bits;
        const POS_OR_REL = Self::POSITION.bits | Self::RELATIVE.bits;
    }
}

/// The possible parameters to an instruction. We use an enum to constrain
/// the range of values that instances of this type can take (as opposed to
/// using say, an i32). The numeric values represent the offset relative
/// to the instruction pointer.
#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
enum ParameterNumber {
    One = 1,
    Two = 2,
    Three = 3,
}

impl ParameterNumber {
    fn offset(&self) -> usize {
        *self as usize
    }
}

fn decode_parameter_mode2(inst: i64, prm_num: ParameterNumber, allowed: ModeFlags) -> Result<ParameterMode, String> {
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
            if allowed.contains(ModeFlags::POSITION) {
                Ok(Position)
            } else {
                Err(format!(
                    "In instruction {}, found parameter mode {}, which does not comply with the allowed mode {:?}",
                    inst, mode, allowed))
            }
        },
        1 => {
            if allowed.contains(ModeFlags::IMMEDIATE) {
                Ok(Immediate)
            } else {
                Err(format!(
                    "In instruction {}, found parameter mode {}, which does not comply with the allowed mode {:?}",
                    inst, mode, allowed))
            }
        },
        2 => {
            if allowed.contains(ModeFlags::RELATIVE) {
                Ok(Relative)
            } else {
                Err(format!(
                    "In instruction {}, found parameter mode {}, which does not comply with the allowed mode {:?}",
                    inst, mode, allowed))
            }
        },
        _ => Err(format!("In instruction {}, found invalid parameter mode {}", inst, mode))
    }
}


/// Represents the IO that the computer is capable of.
pub trait ComputerIo {
    fn try_read(&mut self, message: &str) -> Option<i64>;
    fn write(&mut self, value: i64);
}

/// The default implementation of `ComputerIo` reads from stdin and
/// writes to stdout.
pub struct StandardComputerIoSystem { }

impl StandardComputerIoSystem {
    pub fn new() -> Self {
        Self {}
    }
}

impl ComputerIo for StandardComputerIoSystem {
    fn try_read(&mut self, message: &str) -> Option<i64> {
        use std::io::Write;
        use std::io::{stdout, stdin};

        loop {
            print!("{}", message);
            stdout().flush().unwrap();
            let mut ret = String::new();
            stdin().read_line(&mut ret).expect("Failed to read from stdin");

            match ret.trim().parse::<i64>() {
                Ok(value) => return Some(value),
                Err(_) => {
                    println!("\nNOT A VALID INTEGER. Try again.");
                }
            }
        }
    }

    fn write(&mut self, value: i64) {
        println!("{}", value);
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum ExecutionState {
    Running,
    Halted(i64),
    WaitingOnInput,
}

/// Represents the virtual machine we are executing the program on.
pub struct Computer<I> {
    instruction_pointer: usize,
    program: Vec<i64>,
    relative_base: i64,
    pub io_system: I,
    pub execution_state: ExecutionState,
}

impl<I> Computer<I>
    where I: ComputerIo
{
    pub fn load_program(program: Vec<i64>, io_system: I) -> Self {
        Computer {
            instruction_pointer: 0,
            program: program,
            relative_base: 0,
            io_system: io_system,
            execution_state: ExecutionState::Running,
        }
    }

    /// Executes the given program until the computer halts or has to suspend.
    /// Returns the execution state that it reaches. If the computer halts, the
    /// value stored in address 0 is returned, as several problems require this
    /// as the answer.
    ///
    /// If there are any problems, such as with decoding rogue instructions,
    /// the computer panics.
    pub fn run(&mut self) -> ExecutionState {
        loop {
            self.execution_state = ExecutionState::Running;

            let inst = self.next_instruction().expect("Cannot decode instruction");

            match inst {
                Instruction::Add(p1, p2, p3) => {
                    let p1_value = self.fetch_operand(ParameterNumber::One, p1);
                    let p2_value = self.fetch_operand(ParameterNumber::Two, p2);
                    let result = p1_value + p2_value;
                    self.write_operand(ParameterNumber::Three, p3, result);
                    self.instruction_pointer += inst.instruction_pointer_increment();
                },

                Instruction::Multiply(p1, p2, p3) => {
                    let p1_value = self.fetch_operand(ParameterNumber::One, p1);
                    let p2_value = self.fetch_operand(ParameterNumber::Two, p2);
                    let result = p1_value * p2_value;
                    self.write_operand(ParameterNumber::Three, p3, result);
                    self.instruction_pointer += inst.instruction_pointer_increment();
                },

                Instruction::Read(p1) => {
                    match self.io_system.try_read("Enter number: ") {
                        Some(input) => {
                            self.write_operand(ParameterNumber::One, p1, input);
                            self.instruction_pointer += inst.instruction_pointer_increment();
                        },
                        None => {
                            self.execution_state = ExecutionState::WaitingOnInput;
                            self.instruction_pointer += inst.instruction_pointer_increment();
                            break;
                        }
                    }
                },
                Instruction::Write(p1) => {
                    let value = self.fetch_operand(ParameterNumber::One, p1);
                    self.io_system.write(value);
                    self.instruction_pointer += inst.instruction_pointer_increment();
                    break;
                },

                Instruction::JumpIfTrue(p1, p2) => {
                    let p1_value = self.fetch_operand(ParameterNumber::One, p1);
                    if p1_value != 0 {
                        let new_ip = self.fetch_operand(ParameterNumber::Two, p2) as usize;
                        self.instruction_pointer = new_ip;
                        continue;
                    }
                    self.instruction_pointer += inst.instruction_pointer_increment();
                },

                Instruction::JumpIfFalse(p1, p2) => {
                    let p1_value = self.fetch_operand(ParameterNumber::One, p1);
                    if p1_value == 0 {
                        let new_ip = self.fetch_operand(ParameterNumber::Two, p2) as usize;
                        self.instruction_pointer = new_ip;
                        continue;
                    }
                    self.instruction_pointer += inst.instruction_pointer_increment();
                },

                Instruction::LessThan(p1, p2, p3) => {
                    let p1_value = self.fetch_operand(ParameterNumber::One, p1);
                    let p2_value = self.fetch_operand(ParameterNumber::Two, p2);
                    let result = if p1_value < p2_value { 1 } else { 0 };
                    self.write_operand(ParameterNumber::Three, p3, result);
                    self.instruction_pointer += inst.instruction_pointer_increment();
                },

                Instruction::Equals(p1, p2, p3) => {
                    let p1_value = self.fetch_operand(ParameterNumber::One, p1);
                    let p2_value = self.fetch_operand(ParameterNumber::Two, p2);
                    let result = if p1_value == p2_value { 1 } else { 0 };
                    self.write_operand(ParameterNumber::Three, p3, result);
                    self.instruction_pointer += inst.instruction_pointer_increment();
                },

                Instruction::RelativeBaseOffset(p1) => {
                    let p1_value = self.fetch_operand(ParameterNumber::One, p1);
                    self.relative_base += p1_value;
                    self.instruction_pointer += inst.instruction_pointer_increment();
                },

                Instruction::Halt => {
                    self.execution_state = ExecutionState::Halted(self.program[0]);
                    break;
                },
            }
        }

        self.execution_state
    }

    fn next_instruction(&self) -> Result<Instruction, String> {
        Instruction::decode(self.program[self.instruction_pointer])
    }

    fn fetch_operand(&mut self, operand_number: ParameterNumber, mode: ParameterMode) -> i64 {
        let offset = operand_number.offset();
        let operand_index = self.instruction_pointer + offset;
        self.grow_memory_if_needed(operand_index);

        match mode {
            ParameterMode::Position => {
                let address = self.program[operand_index];
                if address < 0 {
                    panic!("SIGSEGV: address = {}", address);
                }
                self.grow_memory_if_needed(address as usize);
                self.program[address as usize]
            },
            ParameterMode::Immediate => self.program[operand_index],
            ParameterMode::Relative => {
                unimplemented!();
            },
        }
    }

    fn write_operand(&mut self, operand_number: ParameterNumber, mode: ParameterMode, value: i64) {
        let offset = operand_number.offset();
        let operand_index = self.instruction_pointer + offset;
        self.grow_memory_if_needed(operand_index);

        match mode {
            ParameterMode::Position => {
                let address = self.program[operand_index];
                if address < 0 {
                    panic!("SIGSEGV: address = {}", address);
                }
                self.grow_memory_if_needed(address as usize);
                self.program[address as usize] = value;

            },
            ParameterMode::Immediate => panic!("FAULT: Cannot write to Immediate mode parameter"),
            ParameterMode::Relative => {
                unimplemented!();
            },
        }
    }

    fn grow_memory_if_needed(&mut self, address: usize) {
        if address >= self.program.len() {
            self.program.resize(address + 1, 0);
        }
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
        assert!(Instruction::decode(301).is_err());
        assert!(Instruction::decode(3001).is_err());
        assert!(Instruction::decode(30001).is_err());
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

    #[test]
    pub fn decode_jump_if_true() {
        assert_eq!(Instruction::decode(5).unwrap(), JumpIfTrue(Position, Position));
        assert_eq!(Instruction::decode(105).unwrap(), JumpIfTrue(Immediate, Position));
        assert_eq!(Instruction::decode(1005).unwrap(), JumpIfTrue(Position, Immediate));
        assert!(Instruction::decode(10005).is_err(), "Too many digits");
    }

    #[test]
    pub fn decode_jump_if_false() {
        assert_eq!(Instruction::decode(6).unwrap(), JumpIfFalse(Position, Position));
        assert_eq!(Instruction::decode(106).unwrap(), JumpIfFalse(Immediate, Position));
        assert_eq!(Instruction::decode(1006).unwrap(), JumpIfFalse(Position, Immediate));
        assert!(Instruction::decode(10006).is_err(), "Too many digits");
    }

    #[test]
    pub fn decode_less_than() {
        assert_eq!(Instruction::decode(7).unwrap(), LessThan(Position, Position, Position));
        assert_eq!(Instruction::decode(107).unwrap(), LessThan(Immediate, Position, Position));
        assert_eq!(Instruction::decode(1007).unwrap(), LessThan(Position, Immediate, Position));
        assert!(Instruction::decode(10007).is_err(), "Write prm must be Position mode");
    }

    #[test]
    pub fn decode_equals() {
        assert_eq!(Instruction::decode(8).unwrap(), Equals(Position, Position, Position));
        assert_eq!(Instruction::decode(108).unwrap(), Equals(Immediate, Position, Position));
        assert_eq!(Instruction::decode(1008).unwrap(), Equals(Position, Immediate, Position));
        assert!(Instruction::decode(10008).is_err(), "Write prm must be Position mode");
    }
}
