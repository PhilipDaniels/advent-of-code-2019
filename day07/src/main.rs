use permutohedron::LexicalPermutation;
use computer::{Computer, ComputerIo, ExecutionState};

fn get_phase_setting_permutations(mut phase_settings: Vec<i64>) -> Vec<Vec<i64>> {
    let mut permutations = Vec::new();
    loop {
        permutations.push(phase_settings.clone());
        if !phase_settings.next_permutation() {
            break;
        }
    }

    permutations
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

fn main() {
    let permutations = get_phase_setting_permutations(vec![0, 1, 2, 3, 4]);
    println!("There are {} permutations.", permutations.len());

    let program = get_puzzle_input();

    // TODO: Run all these in parallel!
    let mut max_output_signal = 0;
    for permutation in permutations {
        let output_signal = calculate_output_signal(program.clone(), &permutation);
        if output_signal > max_output_signal {
            max_output_signal = output_signal;
            println!("New max_output_signal of {} found for phase settings {:?}",
                max_output_signal, permutation);
        }
    }

    assert_eq!(max_output_signal, 21760);
    println!("The answer for part 1 is {}", max_output_signal);

    // Part 2.
    let permutations = get_phase_setting_permutations(vec![5, 6, 7, 8, 9]);
    max_output_signal = 0;
    for permutation in permutations {
        let output_signal = calculate_output_signal_with_feedback(program.clone(), &permutation);
        if output_signal > max_output_signal {
            max_output_signal = output_signal;
            println!("New max_output_signal of {} found for phase settings {:?}",
                max_output_signal, permutation);
        }
    }

    assert_eq!(max_output_signal, 69816958);
    println!("The answer for part 2 is {}", max_output_signal);
}


fn calculate_output_signal_with_feedback(program: Vec<i64>, permutation: &[i64]) -> i64 {
    let mut amp_a = make_amplifier(program.clone(), permutation[0], 0);
    let mut amp_b = make_amplifier(program.clone(), permutation[1], 0);
    let mut amp_c = make_amplifier(program.clone(), permutation[2], 0);
    let mut amp_d = make_amplifier(program.clone(), permutation[3], 0);
    let mut amp_e = make_amplifier(program.clone(), permutation[4], 0);

    loop {
        amp_a.run();

        amp_b.io_system.value = amp_a.io_system.value.take();
        amp_b.run();

        amp_c.io_system.value = amp_b.io_system.value.take();
        amp_c.run();

        amp_d.io_system.value = amp_c.io_system.value.take();
        amp_d.run();

        amp_e.io_system.value = amp_d.io_system.value.take();
        amp_e.run();

        if let ExecutionState::Halted(_) = amp_e.execution_state {
            return amp_e.io_system.value.unwrap();
        } else {
            amp_a.io_system.value = amp_e.io_system.value.take();
        }
    }
}

fn calculate_output_signal(program: Vec<i64>, permutation: &[i64]) -> i64 {
    let mut amp_a = make_amplifier(program.clone(), permutation[0], 0);
    amp_a.run();
    let stage_output = amp_a.io_system.value.unwrap();

    let mut amp_b = make_amplifier(program.clone(), permutation[1], stage_output);
    amp_b.run();
    let stage_output = amp_b.io_system.value.unwrap();

    let mut amp_c = make_amplifier(program.clone(), permutation[2], stage_output);
    amp_c.run();
    let stage_output = amp_c.io_system.value.unwrap();

    let mut amp_d = make_amplifier(program.clone(), permutation[3], stage_output);
    amp_d.run();
    let stage_output = amp_d.io_system.value.unwrap();

    let mut amp_e = make_amplifier(program.clone(), permutation[4], stage_output);
    amp_e.run();
    let stage_output = amp_e.io_system.value.unwrap();

    stage_output
}

fn make_amplifier(
    program: Vec<i64>,
    phase_setting: i64,
    value: i64) -> Computer<AutoComputerIoSystem>
{
    let io  = AutoComputerIoSystem::new(phase_setting, value);
    Computer::load_program(program, io)
}

pub struct AutoComputerIoSystem {
    num_reads: i32,
    phase_setting: i64,
    // This is both the input and the output value.
    // A bit nasty, but works for this problem.
    value: Option<i64>,
}

impl ComputerIo for AutoComputerIoSystem {
    fn try_read(&mut self, _message: &str) -> Option<i64> {
        if self.num_reads == 0 {
            self.num_reads += 1;
            Some(self.phase_setting)
        } else {
            self.num_reads += 1;
            self.value.take()
        }
    }

    fn write(&mut self, value: i64) {
        self.value = Some(value);
    }
}

impl AutoComputerIoSystem {
    fn new(phase_setting: i64, value: i64) -> Self {
        Self {
            num_reads: 0,
            phase_setting,
            value: Some(value),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    pub fn test1() {
        let permutations = get_phase_setting_permutations(vec![5, 6, 7, 8, 9]);

        let program = get_input("3,26,1001,26,-4,26,3,27,1002,27,2,27,1,27,26,\
            27,4,27,1001,28,-1,28,1005,28,6,99,0,0,5");

        let mut max_output_signal = 0;
        for permutation in permutations {
            let output_signal = calculate_output_signal_with_feedback(program.clone(), &permutation);
            if output_signal > max_output_signal {
                max_output_signal = output_signal;
                println!("New max_output_signal of {} found for phase settings {:?}",
                    max_output_signal, permutation);
            }
        }

        assert_eq!(max_output_signal, 139629729);
    }
}
