use std::collections::HashMap;

const WIRE1_INPUT: &str = "R1000,U573,L25,U468,L833,D867,R515,D941,L513,D1,L380,U335,L661,D725,L506,U365,L103,\
D987,L425,U756,R129,D153,R326,U297,L456,D632,L142,U666,R864,D255,R85,D661,L566,D125,R445,\
U293,R295,D14,R181,D772,R376,U151,L146,D344,L947,D519,L455,D232,L873,U617,R143,D600,R654,\
D14,R813,U176,L443,U712,R230,U629,L554,U886,L931,D591,R716,U904,R605,D176,R801,U911,L746,\
D316,R30,U240,R975,D929,L879,U295,L56,U662,R429,U117,R282,D716,R57,D445,L7,D486,R147,D991,\
R750,D252,R134,U43,L410,D757,R252,U595,R986,U978,L883,D664,R267,D718,R28,U727,R926,U395,\
L81,D70,L67,D92,R209,D633,L253,D798,R820,U816,R754,U646,R846,D863,L868,U911,L678,D893,\
R686,D466,L153,D884,L589,U960,L924,U603,R93,D518,L291,D324,L67,D40,R722,U384,R195,D916,\
R64,D666,R896,D860,R388,D833,L662,D192,R567,U551,L558,U11,L674,U19,L669,U110,R681,D882,\
L997,U535,R683,U313,L904,U674,L476,D969,L464,D342,R574,D981,R405,D352,R431,U429,L329,D160,\
L573,U978,R930,U683,R592,D877,L88,D512,R676,U436,R708,U187,L664,U614,L734,D480,L242,U489,\
R732,U876,L416,D524,R181,U846,L396,D974,L620,D282,L124,D206,R119,U179,L171,D528,R469,U516,\
L708,D599,R913,U63,R922,D300,L856,U700,L396,D185,R933,D453,L234,D385,R426,D189,L25,U599,\
L715,U355,L574,D857,R662,D504,R746,U386,R389,U751,R85,U499,R255,D150,R998,U804,L832,D642,\
R102,U202,R972,U312,L265,D484,R314,D591,L250,U791,L120,D536,L808,D972,L808,D46,L626,D284,\
R60,D155,L849,D501,L206,U445,L765,U770,L67,U780,R876,D409,R603,U713,L459,D81,L294,D471,\
R656,U603,R55,D650,L211,D333,L44,D168,L187,D52,R60,D574,R54";

const WIRE2_INPUT: &str = "L1004,U110,R738,D383,R606,U840,L123,D756,L234,D585,R475,U429,L585,\
D615,L859,D669,L812,U672,L415,D114,L538,D899,R444,D379,L886,D276,R268,D90,R200,D247,L704,D802,\
L10,U313,R437,D854,R899,U21,L553,D352,L736,U604,R162,D504,R509,D471,R501,D472,L117,U796,L828,\
U906,R450,U697,R831,D302,R879,U730,R381,U788,L654,U927,R971,D355,L712,D959,L104,D169,L297,U898,\
R82,D673,R21,D608,L813,U754,L554,U239,L1,U834,R456,D671,L692,D855,L784,U664,R832,U446,L673,D898,\
R146,U507,L934,D569,R249,D755,L212,D475,R970,U122,R418,U820,L754,U313,L843,D608,R165,D881,L293,\
U628,R492,D37,L120,U659,L471,D275,R790,U372,L736,U318,L353,U439,L669,U18,R683,U768,R518,U300,\
L478,U601,R14,U233,L33,U765,L910,U591,R304,D528,R637,D376,L704,U27,L226,U384,R870,U318,L975,U876,\
R576,U500,R880,D108,L670,U171,R561,U873,L391,U717,L455,D909,L34,U211,R919,U376,L228,D632,L91,U408,\
R354,U454,L81,D547,L624,U464,R480,D630,L596,D57,L206,U736,R255,U185,L236,U705,L221,D511,L461,U718,\
R351,D59,L142,U236,R623,D124,R736,D758,L368,D605,L417,U990,R228,D207,L792,U150,L353,U612,R269,D459,\
L855,U808,L852,U168,R838,D794,R478,U281,L453,D134,L643,D862,L299,D590,L570,D782,L294,U935,R835,U849,\
R842,U997,R890,U20,L370,D157,R89,U203,L243,U71,R987,D812,R595,U664,L926,D359,L915,D382,R190,D443,\
R360,U253,R230,D879,L606,D755,R859,U232,R771,U465,R858,D823,R405,D499,L737,U846,R241,D976,R415,U541,\
L746,D569,L563,D410,L409,D39,R117,U638,R824,D215,R232,U578,R790,U535,R873,D477,R805,U94,L313,U570,\
L500,U783,L556,U663,L335,U152,L524,D583,L462,U710,R741,U641,L135";

/// We model the 'board' as a hashset of (x,y) coordinates.
/// It turns out that we only ever need to store the coordinates
/// for the first wire, when we are processing the second wire
/// we can just check for a crossing by probing the hashset, we
/// don't need to store wire 2's path at all.
/// This means we do not have to make a growable matrix-type structure.
type Board = HashMap<(i32, i32), u32>;

#[derive(Debug, Clone, Copy)]
enum Direction {
    Left,
    Right,
    Down,
    Up
}

/// Represents a single step from the input.
struct Instruction {
    direction: Direction,
    number: i32
}

#[derive(PartialEq)]
enum Wire {
    Wire1,
    Wire2
}

fn main() {
    let wire1_input = parse_input(WIRE1_INPUT);
    let wire2_input = parse_input(WIRE2_INPUT);

    let mut board = Board::new();
    plot_wire(&mut board, wire1_input, Wire::Wire1);
    let (manhattan_distance, combined_steps) = plot_wire(&mut board, wire2_input, Wire::Wire2);
    println!("The smallest Manhattan distance is {}", manhattan_distance);
    println!("The minimum combined steps is {}", combined_steps);
}

/// Plot the path of a wire by adding to the board all the positions
/// mentioned by the wire's path, starting at (0,0). If doing wire 2,
/// look for crossings while we do it.
fn plot_wire(board: &mut Board, wire_input: Vec<Instruction>, wire: Wire) -> (i32, u32) {
    // The two things we need to find (min_manhattan_distance for part 1 and
    // min_combined_steps for part 2). We can do both with the same code.
    let mut min_manhattan_distance = 0;
    let mut min_combined_steps = 0;

    let mut x = 0;
    let mut y = 0;
    let mut num_steps = 0;
    for Instruction { direction, mut number } in wire_input {
        while number > 0 {
            // The minimum value we will store in the map is 1.
            num_steps += 1;

            // Calculate the next position. By doing this before we store in
            // the board, we ensure we don't plot a point at (0,0), avoiding
            // having to filter it out as a special case.
            match direction {
                Direction::Left => x -= 1,
                Direction::Right => x += 1,
                Direction::Up => y += 1,
                Direction::Down => y -= 1,
            };

            if wire == Wire::Wire1 {
                board.insert((x, y), num_steps);
            } else {
                // We don't actually need to write Wire2 into the board,
                // we just need to check if there is a Wire1 at that position.
                if let Some(stored_steps) = board.get(&(x, y)) {
                    // This is a crossing point.
                    let this_manhattan_distance = x.abs() + y.abs();

                    if this_manhattan_distance < min_manhattan_distance || min_manhattan_distance == 0 {
                        min_manhattan_distance = this_manhattan_distance;
                    }

                    let this_combined_steps = stored_steps + num_steps;
                    if this_combined_steps < min_combined_steps || min_combined_steps == 0 {
                        min_combined_steps = this_combined_steps;
                    }
                }
            }

            // Proceed with moving N steps for this instruction.
            number -= 1;
        }
    }

    (min_manhattan_distance, min_combined_steps)
}

fn parse_input(input: &str) -> Vec<Instruction> {
    input.split(',')
        .map(|s| parse_instruction(s))
        .collect()
}

fn parse_instruction(instruction: &str) -> Instruction {
    let direction = match &instruction[0..1] {
        "L" => Direction::Left,
        "R" => Direction::Right,
        "D" => Direction::Down,
        "U" => Direction::Up,
        _ => panic!("Unknown direction in instruction {}", instruction)
    };

    let number = instruction[1..].parse::<i32>().unwrap();

    Instruction {
        direction,
        number
    }
}
