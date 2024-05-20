use std::{fs, result};

fn get_instructions() -> Vec<i32> {
    fs::read_to_string("input-2.txt")
        .expect("File does not exist")
        .split(",")
        .map(str::parse::<i32>)
        .filter(result::Result::is_ok)
        .map(result::Result::unwrap)
        .collect()
}

fn handle_operation(opcode: i32, operand_1: i32, operand_2: i32) -> i32 {
    match opcode {
        1 => operand_1 + operand_2,
        2 => operand_1 * operand_2,
        _ => panic!("Unsupported Opcode!"),
    }
}

fn main() {
    let mut instructions = get_instructions();
    instructions[1] = 12;
    instructions[2] = 2;

    for idx in (0..(instructions.len() - 1)).step_by(4) {
        let opcode = instructions[idx];

        if opcode == 99 {
            break;
        }

        let first_operand_idx = instructions[idx + 1];
        let second_operand_idx = instructions[idx + 2];
        let dest_idx = instructions[idx + 3];

        let result = handle_operation(
            opcode,
            instructions[first_operand_idx as usize],
            instructions[second_operand_idx as usize],
        );
        instructions[dest_idx as usize] = result;
    }

    println!("The result at pos 0 is: {}", instructions[0]);
}
