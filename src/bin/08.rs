use std::collections::HashMap;
use num::integer::lcm;

advent_of_code::solution!(8);

pub fn part_a(input: &str) -> Option<u32> {
    let (instructions, nodes) = input.split_once("\n\n").unwrap();
    let mut node_map = HashMap::new();
    for node in nodes.lines() {
        node_map.insert(node[0..3].to_string(), [node[7..10].to_string(), node[12..15].to_string()]);
    }
    let mut instructions = instructions.chars().map(|c| if c == 'L' {0} else if c == 'R' {1} else {unreachable!()}).cycle();
    let mut current_node = &String::from("AAA");
    let mut steps = 0;
    while current_node != &String::from("ZZZ") {
        steps += 1;
        current_node = node_map.get(current_node).unwrap().get(instructions.next().unwrap()).unwrap();
    }

    Some(steps)
}

pub fn part_b(input: &str) -> Option<u64> {
    let (instructions, nodes) = input.split_once("\n\n").unwrap();
    let mut node_map = HashMap::new();
    for node in nodes.lines() {
        node_map.insert(node[0..3].to_string(), [node[7..10].to_string(), node[12..15].to_string()]);
    }
    let instructions = instructions.chars().map(|c| if c == 'L' {0} else if c == 'R' {1} else {unreachable!()}).cycle();

    let mut step_list = vec![];
    for start_node in node_map.keys().filter(|k| k.ends_with("A")) {
        let mut steps = 0;
        let mut new_instructions = instructions.clone();
        let mut current_node = start_node;
        while !current_node.ends_with("Z") {
            steps += 1;
            let next_node = node_map.get(current_node).unwrap().get(new_instructions.next().unwrap()).unwrap();
            current_node = next_node;
        }
        step_list.push(steps);
    }

    step_list.into_iter().reduce(lcm)
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs::read_to_string;
    use advent_of_code::Part;

    #[test]
    fn test_day_008_part_a() {
        let result = part_a(&advent_of_code::template::read_file(
            "examples",
            DAY,
            Some(Part('a')),
        ));
        assert_eq!(result, Some(6));
    }

    #[test]
    fn test_day_008_part_b() {
        let result = part_b(&advent_of_code::template::read_file(
            "examples",
            DAY,
            Some(Part('b')),
        ));
        assert_eq!(result, Some(6));
    }
}