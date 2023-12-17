advent_of_code::solution!(2);

use regex::Regex;

pub fn part_one(input: &str) -> Option<u32> {
    let mut sum_ids = 0;

    for (i, line) in input.lines().enumerate() {

        let game_id = i as u32 + 1;

        let mut games: Vec<(u32, &str)> = vec![];

        let mut flag = false;

        for (_, game) in line.split(";").enumerate() {

            let games_re = Regex::new(r"(\d+) (\w+)").unwrap();

            for (_, [value, colour]) in games_re.captures_iter(game).map(|c| c.extract()){

                let value = value.parse::<u32>().unwrap();
                games.push((value, colour));

                match colour {
                    "blue" => {
                        if value > 14 {
                            flag = true
                        }
                    },
                    "red" => {
                        if value > 12 {
                            flag = true
                        }
                    },
                    "green" => {
                        if value > 13 {
                            flag = true
                        }
                    },
                    _ => continue,
                }
            }
        }

        if !flag {
            sum_ids += game_id;
        }

    }

    Some(sum_ids)
}

pub fn part_two(input: &str) -> Option<u32> {

    let mut sum = 0;

    for (_, line) in input.lines().enumerate() {

        let mut games: Vec<(u32, &str)> = vec![];

        let (mut red, mut green, mut blue) = (0, 0, 0);

        for (_, game) in line.split(";").enumerate() {

            let games_re = Regex::new(r"(\d+) (\w+)").unwrap();

            for (_, [value, colour]) in games_re.captures_iter(game).map(|c| c.extract()){

                let value = value.parse::<u32>().unwrap();
                games.push((value, colour));

                match colour {
                    "blue" => {
                        if value > blue { blue = value };
                    },
                    "red" => {
                        if value > red { red = value };
                    },
                    "green" => {
                        if value > green { green = value };
                    },
                    _ => continue,
                }
            }
        }

        sum += red * green * blue;

    }

    Some(sum)

}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(8));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(2286));
    }
}
