use std::collections::{HashMap, HashSet};
advent_of_code::solution!(4);

pub fn part_a(input: &str) -> Option<u32> {
    let lines = input.lines();
    let mut points = 0;
    for line in lines {
        let mut card_split = line.split(": ");
        card_split.next().unwrap();
        let (winning_numbers, card_numbers) = card_split.next().unwrap().split_once(" | ").unwrap();
        let winning_numbers: HashSet<&str> = winning_numbers.split(' ').collect();
        let card_numbers: HashSet<&str> = card_numbers.split_ascii_whitespace().collect();
        let win_count = winning_numbers.intersection(&card_numbers).count();
        if win_count > 0 {
            points += 2_u32.pow(win_count as u32 - 1);
        }
    }
    Some(points)
}

pub fn part_b(input: &str) -> Option<u32> {
    let lines = input.lines();
    let mut cards = HashMap::new();
    for (card_id, line) in lines.enumerate() {
        let card_id = card_id + 1;
        cards.entry(card_id).and_modify(|v| *v += 1).or_insert(1);
        let mut card_split = line.split(": ");
        card_split.next().unwrap();
        let (winning_numbers, card_numbers) = card_split.next().unwrap().split_once(" | ").unwrap();
        let winning_numbers: HashSet<&str> = winning_numbers.split(' ').collect();
        let card_numbers: HashSet<&str> = card_numbers.split_ascii_whitespace().collect();
        let win_count = winning_numbers.intersection(&card_numbers).count();
        for i in (card_id + 1)..(card_id + win_count + 1) {
            let current_card_count = *cards.get(&card_id).unwrap();
            cards
                .entry(i)
                .and_modify(|v| *v += current_card_count)
                .or_insert(current_card_count);
        }
    }
    Some(cards.values().sum())
}

#[cfg(test)]
mod tests {
    use super::*;
    use advent_of_code::Part;

    #[test]
    fn test_part_one() {
        let result = part_a(&advent_of_code::template::read_file(
            "examples",
            DAY,
            Some(Part('a')),
        ));
        assert_eq!(result, Some(13));
    }

    #[test]
    fn test_part_two() {
        let result = part_b(&advent_of_code::template::read_file(
            "examples",
            DAY,
            Some(Part('b')),
        ));
        assert_eq!(result, Some(30));
    }
}
