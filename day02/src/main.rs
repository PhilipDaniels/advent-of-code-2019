fn get_input() -> Vec<usize> {
    let mut input = vec![
        1,0,0,3,1,1,2,3,1,3,4,3,1,5,0,3,2,1,6,19,1,19,5,23,2,13,23,27,1,10,
        27,31,2,6,31,35,1,9,35,39,2,10,39,43,1,43,9,47,1,47,9,51,2,10,51,
        55,1,55,9,59,1,59,5,63,1,63,6,67,2,6,67,71,2,10,71,75,1,75,5,79,1,
        9,79,83,2,83,10,87,1,87,6,91,1,13,91,95,2,10,95,99,1,99,6,103,2,13,
        103,107,1,107,2,111,1,111,9,0,99,2,14,0,0
    ];

    input[1] = 12;
    input[2] = 2;
    input
}

fn run_program(mut program: Vec<usize>) -> usize {
    let mut current_pos = 0;
    loop {
        let opcode = program[current_pos];
        if opcode == 99 {
            break;
        } else if ! (opcode == 1 || opcode == 2) {
            panic!("Bad opcode: {}", opcode);
        }

        let operand1_idx = program[current_pos + 1];
        let operand2_idx = program[current_pos + 2];

        let result = if opcode == 1 {
            program[operand1_idx] + program[operand2_idx]
        } else {
            program[operand1_idx] * program[operand2_idx]
        };

        let destination_idx = program[current_pos + 3];
        program[destination_idx] = result;
        current_pos += 4;
    }

    program[0]
}

fn main() {
    // This is the run for part 1.
    let input = get_input();
    let result = run_program(input);
    println!("Value at position 0 = {}", result);

    // Now iterate for part 2.
    'done: for noun in 0..=99 {
        for verb in 0..=99 {
            let mut input = get_input();
            input[1] = noun;
            input[2] = verb;
            let result = run_program(input);
            if result == 19690720 {
                println!("noun = {}, verb = {}, 100 * noun + verb = {}",
                    noun,
                    verb,
                    100 * noun + verb
                );
                break 'done;
            }
        }
    }
}
