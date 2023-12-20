advent_of_code::solution!(6);

pub fn part_one(input: &str) -> Option<u32> {
    let input: Vec<Vec<u32>> = input.split("\n")
        .map(|line| line.split(": ")
            .nth(1)
            .unwrap()
            .split_whitespace()
            .map(|ch| ch.parse::<u32>().unwrap())
                .collect())
        .collect();

    let num_races = &input[0].len();

    let mut win_counts: Vec<u32> = vec![];

    for i in 0..*num_races {
        let (time, distance) = (&input[0][i], &input[1][i]);

        let mut win_count: u32 = 0;
        // sec == time button being held at start == mm/ms
        for sec in 0..*time {
            if sec * (time - sec) > *distance {
                win_count += 1;
            }
        }

        win_counts.push(win_count);

    }

    Some(win_counts.iter().fold(1, |acc, a| acc * *a ))
}

pub fn part_two(input: &str) -> Option<u64> {
    let input: Vec<u64> = input.split("\n")
        .map(|line| line.split(": ")
            .nth(1)
            .unwrap()
            .replace(" ", "").parse::<u64>().unwrap()).collect();

    let (time, distance) = (&input[0], &input[1]);

    let mut win_count: u64 = 0;
    for sec in 0..*time {
        if sec * (time - sec) > *distance {
            win_count += 1;
        }
    }

    Some(win_count)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(288));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(71503));
    }
}
