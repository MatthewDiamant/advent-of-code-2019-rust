static INPUT: &str = include_str!("./input.txt");
const ADD: usize = 1;
const MULTIPLY: usize = 2;
const END_PROGRAM: usize = 99;

struct Instruction {
    opcode: usize,
    input_1: usize,
    input_2: usize,
    output_index: usize,
}

fn main() {
    println!("Part 1: {}", calculate_gravity_assist());
}

fn calculate_gravity_assist() -> usize {
    let mut index = 0;
    let mut program = reinitialize_state(parse_input());
    while program[index] != END_PROGRAM {
        let instruction: Instruction = create_instruction(&program, index);
        program[instruction.output_index] = calculate_instruction(&instruction);
        index += 4;
    }
    return program[0];
}

fn parse_input() -> Vec<usize> {
    INPUT
        .trim()
        .split(',')
        .map(|x| usize::from_str_radix(x, 10).unwrap())
        .collect()
}

fn reinitialize_state(mut program: Vec<usize>) -> Vec<usize> {
    program[1] = 12;
    program[2] = 2;
    return program;
}

fn create_instruction(program: &Vec<usize>, index: usize) -> Instruction {
    return Instruction {
        opcode: program[index],
        input_1: program[program[index + 1]],
        input_2: program[program[index + 2]],
        output_index: program[index + 3],
    };
}

fn calculate_instruction(instruction: &Instruction) -> usize {
    if instruction.opcode == ADD {
        return instruction.input_1 + instruction.input_2;
    } else if instruction.opcode == MULTIPLY {
        return instruction.input_1 * instruction.input_2;
    } else {
        return 0;
    }
}
