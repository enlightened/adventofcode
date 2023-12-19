advent_of_code::solution!(5);

use std::collections::HashSet;

fn parse_section(part: &&str) -> Vec<Vec<u64>> {
    part.split(":\n")
        .nth(1)
        .unwrap()
        .split("\n")
        .map(|a| a.split_whitespace()
            .map(|b| b.parse::<u64>().unwrap())
            .collect())
        .collect()
}

pub fn part_one(input: &str) -> Option<u64> {

    let input: Vec<&str> = input.split("\n\n").collect::<Vec<&str>>();

    let seeds: Vec<u64> = input[0].split(": ")
        .nth(1)
        .unwrap()
        .split_whitespace()
        .map(|a| a.parse::<u64>().unwrap()).map(|mut a| {
            for sec in &input[1..] {
                'inner: for line in parse_section(sec) {
                    let (dest, source, length) = (line[0], line[1], line[2]);

                    if a <= source + length && a >= source {
                        a = dest + (a - source);
                        break 'inner;
                    }
                }
            }
            a
        }).collect();

    Some(*seeds.iter().min().unwrap())
}

pub fn part_two(input: &str) -> Option<u64> {

    // This is going to be disgustingly unoptimized (takes >12GB of RAM and >80% of CPU on a M1 Pro)

    let input: Vec<&str> = input.split("\n\n").collect::<Vec<&str>>();

    let seeds: Vec<u64> = input[0].split(": ")
        .nth(1)
        .unwrap()
        .split_whitespace()
        .map(|a| a.parse::<u64>()
        .unwrap())
        .collect();

    let mut i = 0;
    let mut seeds_2: HashSet<u64> = HashSet::new();

    while i < seeds.len() {

        let start = seeds[i];
        let length = seeds[i+1];

        seeds_2.extend((start..start + length).collect::<Vec<u64>>());


        i += 2;
    }

    let seeds_2: Vec<u64> = seeds_2.into_iter().map(|mut a| {
        for sec in &input[1..] {
            'inner: for line in parse_section(sec) {
                let (dest, source, length) = (line[0], line[1], line[2]);

                if a <= source + length && a >= source {
                    a = dest + (a - source);
                    break 'inner;
                }
            }
        }
        a
    }).collect();

    Some(*seeds_2.iter().min().unwrap())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(35));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(46));
    }
}
