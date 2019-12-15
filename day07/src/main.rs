use permutohedron::LexicalPermutation;
use computer::{Computer, StandardComputerIoSystem, ComputerIo};

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

    // First amplifier input is 0.
    // There are 5 amplifiers wired in series.

    // Phase setting is an integer 0..=4.
    // Each amp will ask for this setting.
    // Each is used only once (what are all the possible
    // permutations of 0..=4 ?)
}

fn calculate_output_signal(permutation: &[i32]) -> i32 {
    let mut amp_a = make_amplifier(permutation[0]);

    // The computer currently returns 'output' that is simply
    // the value left at address 0. This is UTTERLY IRRELEVANT
    // for this problem, where the output we want is the value
    // that is written to stdout.
    amp_a.run().expect("Program should produce a valid output");

    0
}

fn make_amplifier(phase_setting: i32) -> Computer<AutoComputerIoSystem> {
    let program = get_puzzle_input();

    let io = AutoComputerIoSystem::new(phase_setting);
    Computer::load_program(program, io)
}

pub struct AutoComputerIoSystem {
    phase_setting: i32,
    input: Option<i32>,
    output: Option<i32>,
}

impl ComputerIo for AutoComputerIoSystem {
    fn read(&self, message: &str) -> i32 {
        0
    }

    fn write(&self, message: &str) {
        self.output = Some(
            message.parse::<i32>().expect("Program should write a valid integer")
        );
    }
}

impl AutoComputerIoSystem {
    fn new(phase_setting: i32) -> Self {
        Self {
            phase_setting,
            input: None,
            output: None,
        }
    }
}