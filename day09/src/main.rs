use computer::{Computer, ComputerIo, StandardComputerIoSystem, ExecutionState};

fn main() {
    let program = get_puzzle_input();
    let io  = StandardComputerIoSystem::new();
    let mut computer = Computer::load_program(program, io);
    computer.run();
}

fn get_input(raw_input: &str) -> Vec<i64> {
    raw_input.split(',')
        .map(|s| s.parse::<i64>().expect("Input should be an integer"))
        .collect()
}

fn get_puzzle_input() -> Vec<i64> {
    let data = include_str!("input.txt");
    get_input(data)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    pub fn test1() {
        let input = get_input("109,1,204,-1,1001,100,1,100,1008,100,16,101,1006,101,0,99");
    }

    #[test]
    pub fn test2() {
        let input = get_input("1102,34915192,34915192,7,4,7,99,0");
    }

    #[test]
    pub fn test3() {
        let input = get_input("104,1125899906842624,99");
    }
}
