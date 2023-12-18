advent_of_code::solution!(4);

use std::collections::HashSet;

pub fn part_one(input: &str) -> Option<u32> {

    let mut sum: u32 = 0;

    for line in input.lines() {

        let mut card_value: u32 = 0;

        let sides: Vec<&str> = line.split("|").collect();

        let winning_numbers: HashSet<u32> = sides[0]
            .split(": ")
            .collect::<Vec<&str>>()[1]
            .split(' ')
            .map(|a| a.trim())
            .filter(|a| !a.is_empty())
            .map(|a| a.parse::<u32>().unwrap())
            .collect::<HashSet<u32>>();

        let game_numbers: HashSet<u32> = sides[1]
            .split(' ')
            .map(|a| a.trim())
            .filter(|a| !a.is_empty())
            .map(|a| a.parse::<u32>().unwrap())
            .collect::<HashSet<u32>>();


        let intersection = game_numbers.intersection(&winning_numbers);

        for _ in intersection {
            card_value = if card_value == 0 { 1 } else { card_value * 2 }; // Apparently this could be replaced by fold, but have yet to learn that
        }

        sum += card_value;

    }

    Some(sum)
}

pub fn part_two(input: &str) -> Option<u32> {

    let count = input.lines().count();

    let mut number = vec![1; count];

    for (i, line) in input.lines().enumerate() {

        let sides: Vec<&str> = line.split("|").collect();

        let winning_numbers: HashSet<u32> = sides[0]
            .split(": ")
            .collect::<Vec<&str>>()[1]
            .split(' ')
            .map(|a| a.trim())
            .filter(|a| !a.is_empty())
            .map(|a| a.parse::<u32>().unwrap())
            .collect::<HashSet<u32>>();

        let game_numbers: HashSet<u32> = sides[1]
            .split(' ')
            .map(|a| a.trim())
            .filter(|a| !a.is_empty())
            .map(|a| a.parse::<u32>().unwrap())
            .collect::<HashSet<u32>>();


        let intersection = game_numbers.intersection(&winning_numbers);

        let win_count: u32 = intersection.count() as u32;

        for j in (1..=win_count).into_iter() {
            match number.get(i + j as usize) {
                None => continue,
                Some(_) => {
                    number[i + j as usize] += number[i]
                },
            }
        }

    }

    Some(number.iter().sum())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(13));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(30));
    }
}
