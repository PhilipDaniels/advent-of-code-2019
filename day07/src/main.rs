use permutohedron::LexicalPermutation;

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
        .map(|s| i32::parse(s).expect("Input should be an integer"))
        .collect()
}

fn get_puzzle_input() -> Vec<i32> {
    let data = include_str!("input.txt");
    get_input(data)
}

fn main() {
    let permutations = get_phase_setting_permutations();
    println!("There are {} permutations.", permutations.len());

    // First amplifier input is 0.
    // There are 5 amplifiers wired in series.

    // Phase setting is an integer 0..=4.
    // Each amp will ask for this setting.
    // Each is used only once (what are all the possible
    // permutations of 0..=4 ?)


}
