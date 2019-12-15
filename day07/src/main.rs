use permutohedron::LexicalPermutation;
use computer::{Computer, ComputerIo};

fn get_phase_setting_permutations() -> Vec<Vec<i32>> {
    let mut phase_settings = vec![0, 1, 2, 3, 4];
    let mut permutations = Vec::new();
    loop {
        permutations.push(phase_settings.clone());
        if !phase_settings.next_permutation() {
            break;
        }
    }

    permutations
}

fn get_input(raw_input: &str) -> Vec<i32> {
    raw_input.split(',')
        .map(|s| s.parse::<i32>().expect("Input should be an integer"))
        .collect()
}

fn get_puzzle_input() -> Vec<i32> {
    let data = include_str!("input.txt");
    get_input(data)
}

fn main() {
    let permutations = get_phase_setting_permutations();
    println!("There are {} permutations.", permutations.len());

    // TODO: Run all these in parallel!
    let mut max_output_signal = 0;
    for permutation in permutations {
        let output_signal = calculate_output_signal(&permutation);
        if output_signal > max_output_signal {
            max_output_signal = output_signal;
            println!("New max_output_signal of {} found for phase settings {:?}",
                max_output_signal, permutation);
        }
    }

    assert_eq!(max_output_signal, 21760, "This is the answer");
}

fn calculate_output_signal(permutation: &[i32]) -> i32 {
    // The computer currently returns 'output' that is simply
    // the value left at address 0. This is UTTERLY IRRELEVANT
    // for this problem, where the output we want is the value
    // that is written to stdout.
    let mut amp_a = make_amplifier(permutation[0], 0);
    amp_a.run().expect("Program should produce a valid output");
    let stage_output = amp_a.io_system.value;

    let mut amp_b = make_amplifier(permutation[1], stage_output);
    amp_b.run().expect("Program should produce a valid output");
    let stage_output = amp_b.io_system.value;

    let mut amp_c = make_amplifier(permutation[2], stage_output);
    amp_c.run().expect("Program should produce a valid output");
    let stage_output = amp_c.io_system.value;

    let mut amp_d = make_amplifier(permutation[3], stage_output);
    amp_d.run().expect("Program should produce a valid output");
    let stage_output = amp_d.io_system.value;

    let mut amp_e = make_amplifier(permutation[4], stage_output);
    amp_e.run().expect("Program should produce a valid output");
    let stage_output = amp_e.io_system.value;

    stage_output
}

fn make_amplifier(phase_setting: i32, value: i32) -> Computer<AutoComputerIoSystem> {
    let program = get_puzzle_input();
    let io  = AutoComputerIoSystem::new(phase_setting, value);
    Computer::load_program(program, io)
}

pub struct AutoComputerIoSystem {
    num_reads: i32,
    phase_setting: i32,
    // This is both the input and the output value.
    // A bit nasty, but works for this problem.
    value: i32,
}

impl ComputerIo for AutoComputerIoSystem {
    fn read(&mut self, _message: &str) -> i32 {
        match self.num_reads {
            0 => {
                self.num_reads += 1;
                self.phase_setting
            },
            1 => {
                self.num_reads += 1;
                self.value
            },
            _ => panic!("Should not read more than twice.")

        }
    }

    fn write(&mut self, message: &str) {
        self.value = message.parse::<i32>().expect("Program should write a valid integer");
    }
}

impl AutoComputerIoSystem {
    fn new(phase_setting: i32, value: i32) -> Self {
        Self {
            num_reads: 0,
            phase_setting,
            value: value,
        }
    }
}