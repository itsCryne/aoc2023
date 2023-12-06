advent_of_code::solution!(6);

pub fn part_a(input: &str) -> Option<u32> {
    let (times, distances) = input.split_once('\n').unwrap();
    let times: Vec<u32> = times
        .split(':')
        .last()
        .unwrap()
        .split_whitespace()
        .map(|t| t.parse().unwrap())
        .collect();
    let distances: Vec<u32> = distances
        .split(':')
        .last()
        .unwrap()
        .split_whitespace()
        .map(|t| t.parse().unwrap())
        .collect();

    let mut margin = 1;
    for (time, distance) in times.into_iter().zip(distances) {
        let mut race_sum = 0;
        for my_time in 0..=time {
            if (time - my_time) * my_time > distance {
                race_sum += 1;
            }
        }
        margin *= race_sum;
    }

    Some(margin)
}

pub fn part_b(input: &str) -> Option<u64> {
    let (time, distance) = input.split_once('\n').unwrap();
    let time: u64 = time
        .split(':')
        .last()
        .unwrap()
        .split_whitespace()
        .collect::<Vec<&str>>()
        .join("")
        .parse()
        .unwrap();
    let distance: u64 = distance
        .split(':')
        .last()
        .unwrap()
        .split_whitespace()
        .collect::<Vec<&str>>()
        .join("")
        .parse()
        .unwrap();

    let mut race_sum = 0;
    for my_time in 0..=time {
        if (time - my_time) * my_time > distance {
            race_sum += 1;
        }
    }
    Some(race_sum)
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
        assert_eq!(result, Some(288));
    }

    #[test]
    fn test_part_two() {
        let result = part_b(&advent_of_code::template::read_file(
            "examples",
            DAY,
            Some(Part('b')),
        ));
        assert_eq!(result, Some(71503));
    }
}
