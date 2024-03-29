use computer::{Computer, StandardComputerIoSystem, ExecutionState};

fn day2_program() -> Vec<i64> {
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
    let mut computer = Computer::load_program(program, StandardComputerIoSystem::new());
    match computer.run() {
        ExecutionState::Halted(result) => println!("Success, result = {}", result),
        e @ _ => println!("Error: {:?}", e),
    }

    // Now iterate for part 2. Should print noun = 95, verb = 7, 100 * noun + verb = 9507.
    'done: for noun in 0..=99 {
        for verb in 0..=99 {
            let mut program = day2_program();
            program[1] = noun;
            program[2] = verb;

            let mut computer = Computer::load_program(program, StandardComputerIoSystem::new());
            match computer.run() {
                ExecutionState::Halted(result) => {
                    if result == 19690720 {
                        println!("noun = {}, verb = {}, 100 * noun + verb = {}",
                            noun,
                            verb,
                            100 * noun + verb
                        );
                        break 'done;
                    }
                },
                e @ _ => println!("Error: {:?}", e),
            }
        }
    }
}

fn day5_program() -> Vec<i64> {
    vec![
        3,225,1,225,6,6,1100,1,238,225,104,0,1101,90,64,225,1101,15,56,225,1,14,153,
        224,101,-147,224,224,4,224,1002,223,8,223,1001,224,3,224,1,224,223,223,2,162,
        188,224,101,-2014,224,224,4,224,1002,223,8,223,101,6,224,224,1,223,224,223,
        1001,18,81,224,1001,224,-137,224,4,224,1002,223,8,223,1001,224,3,224,1,223,
        224,223,1102,16,16,224,101,-256,224,224,4,224,1002,223,8,223,1001,224,6,224,
        1,223,224,223,101,48,217,224,1001,224,-125,224,4,224,1002,223,8,223,1001,224,
        3,224,1,224,223,223,1002,158,22,224,1001,224,-1540,224,4,224,1002,223,8,223,
        101,2,224,224,1,223,224,223,1101,83,31,225,1101,56,70,225,1101,13,38,225,102,
        36,192,224,1001,224,-3312,224,4,224,1002,223,8,223,1001,224,4,224,1,224,223,
        223,1102,75,53,225,1101,14,92,225,1101,7,66,224,101,-73,224,224,4,224,102,8,
        223,223,101,3,224,224,1,224,223,223,1101,77,60,225,4,223,99,0,0,0,677,0,0,0,0,
        0,0,0,0,0,0,0,1105,0,99999,1105,227,247,1105,1,99999,1005,227,99999,1005,0,256,
        1105,1,99999,1106,227,99999,1106,0,265,1105,1,99999,1006,0,99999,1006,227,274,
        1105,1,99999,1105,1,280,1105,1,99999,1,225,225,225,1101,294,0,0,105,1,0,1105,1,
        99999,1106,0,300,1105,1,99999,1,225,225,225,1101,314,0,0,106,0,0,1105,1,99999,
        7,226,677,224,1002,223,2,223,1005,224,329,1001,223,1,223,1007,226,677,224,1002,
        223,2,223,1005,224,344,101,1,223,223,108,226,226,224,1002,223,2,223,1006,224,
        359,101,1,223,223,7,226,226,224,102,2,223,223,1005,224,374,101,1,223,223,8,677,
        677,224,1002,223,2,223,1005,224,389,1001,223,1,223,107,677,677,224,102,2,223,223,
        1006,224,404,101,1,223,223,1107,677,226,224,102,2,223,223,1006,224,419,1001,223,
        1,223,1008,226,226,224,1002,223,2,223,1005,224,434,1001,223,1,223,7,677,226,224,
        102,2,223,223,1006,224,449,1001,223,1,223,1107,226,226,224,1002,223,2,223,1005,
        224,464,101,1,223,223,1108,226,677,224,102,2,223,223,1005,224,479,101,1,223,223,
        1007,677,677,224,102,2,223,223,1006,224,494,1001,223,1,223,1107,226,677,224,1002,
        223,2,223,1005,224,509,101,1,223,223,1007,226,226,224,1002,223,2,223,1006,224,
        524,101,1,223,223,107,226,226,224,1002,223,2,223,1005,224,539,1001,223,1,223,1108,
        677,677,224,1002,223,2,223,1005,224,554,101,1,223,223,1008,677,226,224,102,2,223,
        223,1006,224,569,1001,223,1,223,8,226,677,224,102,2,223,223,1005,224,584,1001,223,
        1,223,1008,677,677,224,1002,223,2,223,1006,224,599,1001,223,1,223,108,677,677,224,
        102,2,223,223,1006,224,614,1001,223,1,223,108,226,677,224,102,2,223,223,1005,224,
        629,101,1,223,223,8,677,226,224,102,2,223,223,1005,224,644,101,1,223,223,107,677,
        226,224,1002,223,2,223,1005,224,659,101,1,223,223,1108,677,226,224,102,2,223,223,
        1005,224,674,1001,223,1,223,4,223,99,226
    ]
}

fn main() {
    //validate_day2_using_library_interpreter();

    /* Worked first time! Results:

    Enter number: 1
    0
    0
    0
    0
    0
    0
    0
    0
    0
    7988899

    */
    let program = day5_program();
    let mut computer = Computer::load_program(program, StandardComputerIoSystem::new());
    println!("{:?}", computer.run());
}
