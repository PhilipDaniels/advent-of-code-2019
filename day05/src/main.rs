use computer::{Computer};

fn day2_program() -> Vec<i32> {
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

/// Run day 2's problems using our new librarified interpreter and verify
/// that we get the same results.
fn validate_day2_using_library_interpreter() {
    // This is the run for part 1. Should print 2692315.
    let program = day2_program();
    let mut computer = Computer::load_program(program);
    computer.run_and_print_result();

    // Now iterate for part 2. Should print noun = 95, verb = 7, 100 * noun + verb = 9507.
    'done: for noun in 0..=99 {
        for verb in 0..=99 {
            let mut program = day2_program();
            program[1] = noun;
            program[2] = verb;

            let mut computer = Computer::load_program(program);
            let result = computer.run().unwrap();

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

fn main() {
    validate_day2_using_library_interpreter();

}
