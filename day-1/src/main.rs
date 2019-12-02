fn main() {
    println!("Part 1: {}", mass());
    println!("Part 2: {}", total_mass());
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

fn total_mass() -> i32 {
    return parse_input().into_iter().fold(0, total_mass_reducer);
}

fn total_mass_reducer(acc: i32, module: i32) -> i32 {
    let mut mass: i32 = module / 3 - 2;
    let mut total_mass: i32 = acc;
    while mass > 0 {
        total_mass += mass;
        mass = mass / 3 - 2;
    }
    total_mass
}
