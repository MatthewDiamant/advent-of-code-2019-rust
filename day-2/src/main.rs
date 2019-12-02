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
    println!("Part 1: {}", part_1());
    println!("Part 2: {}", part_2());
}

fn part_1() -> usize {
    let program = initialize_state(parse_input(), 12, 2);
    return calculate_gravity_assist(program);
}

fn part_2() -> usize {
    let input = parse_input();
    for verb in 0..100 {
        for noun in 0..100 {
            let program = initialize_state(input.clone(), noun, verb);
            let result = calculate_gravity_assist(program);
            if result == 19690720 {
                return 100 * noun + verb;
            }
        }
    }
    0
}

fn calculate_gravity_assist(mut program: Vec<usize>) -> usize {
    let mut index = 0;
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

fn initialize_state(mut program: Vec<usize>, noun: usize, verb: usize) -> Vec<usize> {
    program[1] = noun;
    program[2] = verb;
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
