use crate::Day;
use std::process;

const MAIN_FUNC: &str = r#"fn main() {
    let input = include_str!("input.txt");
    println!("Part A: \x1b[1m{}\x1b[0m", part_a(input).unwrap());
    println!("Part B: \x1b[1m{}\x1b[0m", part_b(input).unwrap());
}"#;

pub fn handle(day: Day) {
    let path = format!("src/bin/{}.rs", day);
    let src = match std::fs::read_to_string(&path) {
        Err(e) => {
            eprintln!("Failed to read {}: {}", path, e);
            process::exit(1)
        }
        Ok(content) => content,
    };

    let mut import_split = src.split("advent_of_code::solution!");
    let imports = import_split.next().unwrap();

    let front_split = import_split.next().unwrap().split_once(";\n\n").unwrap();

    let mut back_split = front_split.1.split("\n#[cfg(test)]");
    let solution = back_split.next().unwrap().to_string();

    let executable = format!("{}{}\n\n{}", imports, MAIN_FUNC, solution);

    print!("{}", executable);
}
