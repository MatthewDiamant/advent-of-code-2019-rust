fn main() {
    println!("Part 1: {}", mass());
}

fn parse_input() -> Vec<i32> {
    let file = include_str!("./input.txt");
    file.lines()
        .map(|x| i32::from_str_radix(x, 10).unwrap())
        .collect()
}

fn mass() -> i32 {
    parse_input().into_iter().fold(0, mass_reducer)
}

fn mass_reducer(acc: i32, module: i32) -> i32 {
    acc + (module / 3 - 2)
}
