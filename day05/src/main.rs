use computer::Instruction;




fn main() {
    let instr = Instruction::decode(789769);
    println!("{:?}", instr);
}

