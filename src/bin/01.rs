advent_of_code::solution!(1);

pub fn part_one(input: &str) -> Option<u32> {
    let mut sum: u32 = 0;

    for line in input.lines() {
        let numbers: Vec<u32> = line
            .chars()
            .filter_map(|a| a.to_digit(10))
            .collect();

        let first: &u32 = numbers.first().unwrap();
        let last: &u32 = numbers.last().unwrap();

        sum += (first * 10) + last;

    }

    Some(sum)
}

pub fn part_two(input: &str) -> Option<u32> {
    let mut sum: u32 = 0;

    for line in input.lines() {

        let mut first: u32 = 0;
        let mut last: u32 = 0;

        for (i, c) in line.chars().enumerate() {

            let current = {
                if c.is_digit(10) {
                    c.to_digit(10)
                } else {
                    match c {
                        'o' => {
                            if line.len() - i > 2 && &line[i..i+3] == "one" {
                                Some(1)
                            } else { None }
                        },
                        't' => {
                            if line.len() - i > 2 && &line[i..i+3] == "two" {
                                Some(2)
                            } else if line.len() - i > 4 && &line[i..i+5] == "three" {
                                Some(3)
                            } else {
                                None
                            }
                        },
                        'f' => {
                            if line.len() - i > 3 && &line[i..i+4] == "four" {
                                Some(4)
                            } else if line.len() - i > 3 && &line[i..i+4] == "five" {
                                Some(5)
                            } else {
                                None
                            }
                        },
                        's' => {
                            if line.len() - i > 2 && &line[i..i+3] == "six" {
                                Some(6)
                            } else if line.len() - i > 4 && &line[i..i+5] == "seven" {
                                Some(7)
                            } else {
                                None
                            }
                        },
                        'e' => {
                            if line.len() - i > 4 && &line[i..i+5] == "eight" {
                                Some(8)
                            } else {
                                None
                            }
                        },
                        'n' => {
                            if line.len() - i > 3 && &line[i..i+4] == "nine" {
                                Some(9)
                            } else {
                                None
                            }
                        },
                        _ => None,
                    }
                }
            };
            match current {
                Some(n) => {
                    first = if first == 0 { n } else { first };
                    last = n;
                }
                None => continue,
            }

        }

        sum += (first * 10) + last;

    }

    Some(sum)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(142));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(281));
    }
}
