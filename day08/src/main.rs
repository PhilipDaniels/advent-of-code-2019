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

    let answer = layer_with_fewest_zeros.1.one * layer_with_fewest_zeros.1.two;
    assert_eq!(answer, 1548);
    println!("Answer for part 1 = {}", answer);


    // Part 2.
    let layers = input.chunks(IMAGE_WIDTH * IMAGE_HEIGHT).collect::<Vec<_>>();
    println!("There are {} layers", layers.len());  // 100 layers.

    let mut final_image = Vec::<u32>::new();

    for pixel_index in 0..IMAGE_WIDTH * IMAGE_HEIGHT {
        for layer_index in 0..layers.len() {
            if layers[layer_index][pixel_index] != 2 {
                final_image.push(layers[layer_index][pixel_index]);
                break;
            }
        }
    }

    // Print the final image (prints "CEKUA").
    println!();
    let mut n = 0;
    for pixel in final_image {
        print!("{}", if pixel == 0 { '.' } else { '#'});
        if n == IMAGE_WIDTH - 1{
            println!();
            n = 0;
        } else {
            n += 1;
        }
    }
}
