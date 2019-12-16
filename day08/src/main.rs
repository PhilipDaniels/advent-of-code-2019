fn get_input(raw_input: &str) -> Vec<u32> {
    raw_input.chars()
        .map(|c| c.to_digit(10).expect("Input should be an integer"))
        .collect()
}

fn get_puzzle_input() -> Vec<u32> {
    let data = include_str!("input.txt");
    get_input(data)
}

#[derive(Debug, Default)]
struct Counts {
    zero: u32,
    one: u32,
    two: u32,
}

fn main() {
    const IMAGE_WIDTH: usize = 25;
    const IMAGE_HEIGHT: usize = 6;

    let input = get_puzzle_input();
    let layers = input.chunks(IMAGE_WIDTH * IMAGE_HEIGHT)
        .map(|layer| {
            let mut counts = Counts::default();
            for i in layer {
                if *i == 0 {
                    counts.zero += 1;
                } else if *i == 1 {
                    counts.one += 1;
                } else if *i == 2 {
                    counts.two += 1;
                }
            }
            (layer, counts)
        });

    let layer_with_fewest_zeros = layers
        .min_by_key(|(_, counts)| counts.zero)
        .unwrap();

    println!("L = {:?}", layer_with_fewest_zeros);
    let answer = layer_with_fewest_zeros.1.one * layer_with_fewest_zeros.1.two;
    assert_eq!(answer, 1548);
    println!("Answer = {}", answer);
}
