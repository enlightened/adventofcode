use std::vec;
use std::collections::HashSet;

advent_of_code::solution!(3);

pub fn part_one(input: &str) -> Option<u32> {
    let mut part_num_sum: u32 = 0;

    let array: Vec<Vec<char>> = input.lines().map(|line| line.chars().collect()).collect();

    let row_count = array.len();
    let col_count = array[0].len();

    for (i, row) in array.iter().enumerate() {
        let mut current_num: Vec<char> = vec![];
        let mut current_num_part = false;
        for (j, ch) in row.iter().enumerate() {

            // println!("{ch}");
            if ch.is_digit(10) {
                current_num.push(*ch);

                // Top left
                if (i != 0 && j != 0) && (!array[i-1][j-1].is_digit(10) && array[i-1][j-1] != '.') {
                    current_num_part = true;
                }
                // Top centre
                if (i != 0) && (!array[i-1][j].is_digit(10) && array[i-1][j] != '.') {
                    current_num_part = true;
                }
                // Top right
                if (i != 0 && j + 1 < col_count) && (!array[i-1][j+1].is_digit(10) && array[i-1][j+1] != '.') {
                    current_num_part = true;
                }
                // Centre left
                if (j != 0) && (!array[i][j-1].is_digit(10) && array[i][j-1] != '.') {
                    current_num_part = true;
                }
                // Centre right
                if (j + 1 < col_count) && (!array[i][j+1].is_digit(10) && array[i][j+1] != '.') {
                    current_num_part = true;
                }
                // Bottom left
                if (i + 1 < row_count && j != 0) && (!array[i+1][j-1].is_digit(10) && array[i+1][j-1] != '.') {
                    current_num_part = true;
                }
                // Bottom centre
                if (i + 1 < row_count) && (!array[i+1][j].is_digit(10) && array[i+1][j] != '.') {
                    current_num_part = true;
                }
                // Bottom right
                if (i + 1 < row_count && j + 1 < col_count) && (!array[i+1][j+1].is_digit(10) && array[i+1][j+1] != '.') {
                    current_num_part = true;
                }

            }

            if !ch.is_digit(10) || j + 1 == col_count {

                if current_num_part {
                   let joined_num: String = current_num.iter().collect();
                   part_num_sum += joined_num.parse::<u32>().unwrap();
                }

                current_num_part = false;
                current_num = vec![];

            }

        }
    }

    Some(part_num_sum)
}

pub fn part_two(input: &str) -> Option<u32> {
    let mut gear_num_sum: u32 = 0;

    let array: Vec<Vec<char>> = input.lines().map(|line| line.chars().collect()).collect();

    let row_count = array.len();
    let col_count = array[0].len();

    let mut gear_adj: Vec<(u32, (usize, usize))> = vec![];


    for (i, row) in array.iter().enumerate() {
        let mut current_num: Vec<char> = vec![];
        let mut current_num_part = false;
        let mut current_gear_loc: HashSet<(usize, usize)> = HashSet::new();

        for (j, ch) in row.iter().enumerate() {

            if ch.is_digit(10) {
                current_num.push(*ch);

                // Top left
                if (i != 0 && j != 0) && (array[i-1][j-1] == '*') {
                    current_num_part = true;
                    current_gear_loc.insert((i-1, j-1));
                }
                // Top centre
                if (i != 0) && (array[i-1][j] == '*') {
                    current_num_part = true;
                    current_gear_loc.insert((i-1, j));
                }
                // Top right
                if (i != 0 && j + 1 < col_count) && (array[i-1][j+1] == '*') {
                    current_num_part = true;
                    current_gear_loc.insert((i-1, j+1));
                }
                // Centre left
                if (j != 0) && (array[i][j-1] == '*') {
                    current_num_part = true;
                    current_gear_loc.insert((i, j-1));
                }
                // Centre right
                if (j + 1 < col_count) && (array[i][j+1] == '*') {
                    current_num_part = true;
                    current_gear_loc.insert((i, j+1));
                }
                // Bottom left
                if (i + 1 < row_count && j != 0) && (array[i+1][j-1] == '*') {
                    current_num_part = true;
                    current_gear_loc.insert((i+1, j-1));
                }
                // Bottom centre
                if (i + 1 < row_count) && (array[i+1][j] == '*') {
                    current_num_part = true;
                    current_gear_loc.insert((i+1, j));
                }
                // Bottom right
                if (i + 1 < row_count && j + 1 < col_count) && (array[i+1][j+1] == '*') {
                    current_num_part = true;
                    current_gear_loc.insert((i+1, j+1));
                }

            }

            if !ch.is_digit(10) || j + 1 == col_count {

                if current_num_part {
                   let joined_num: String = current_num.iter().collect();

                   for x in current_gear_loc {
                       gear_adj.push((joined_num.parse::<u32>().unwrap(), x));
                   }
                }

                current_num_part = false;
                current_num = vec![];
                current_gear_loc = HashSet::new()

            }

        }
    }

    while let Some(x) = gear_adj.pop() {
        let matching: Vec<_> = gear_adj.iter().filter(|a| a.1 == x.1).collect();
        if matching.len() == 1 {
            gear_num_sum += matching[0].0 * x.0;

        }
        gear_adj.retain(|a| a.1 != x.1)
    }


    Some(gear_num_sum)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(4361));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(467835));
    }
}
