use regex::Regex;
use std::path::Path;
use std::{char, env, fs, u128};

fn main() {
    day_04();
}

fn day_04() {
    let content = read_from_file("day4.txt");
    let mut count = 0;

    let mut matrix: Vec<Vec<char>> = Vec::new();

    for line in content.lines() {
        matrix.push(line.chars().collect());
    }

    for i in 0..matrix.len() {
        for j in 0..matrix.get(i).unwrap().len() {
            if matrix[i][j] == 'X' || matrix[i][j] == 'S' {
                count += bfs(i, j, &mut matrix);
            }
        }
    }

    println!("{}", count / 2)
}

fn bfs(i: usize, j: usize, matrix: &mut Vec<Vec<char>>) -> usize {
    let n = matrix.len();
    let m = matrix.get(0).unwrap().len();
    let mut count = 0;

    let directions = [
        (-1, 0),
        (1, 0),
        (0, -1),
        (0, 1),
        (-1, -1),
        (-1, 1),
        (1, -1),
        (1, 1),
    ];

    for direction in directions.iter() {
        let mut new_row = i as isize;
        let mut new_col = j as isize;

        let mut word = Vec::new();
        word.push(matrix[i][j]);

        for _ in 1..=4 {
            new_row += direction.0;
            new_col += direction.1;

            if new_row < 0 || new_row >= n as isize || new_col < 0 || new_col >= m as isize {
                break;
            }

            word.push(matrix[new_row as usize][new_col as usize]);

            if word.len() > 4 {
                break;
            }

            if word == ['X', 'M', 'A', 'S'] || word == ['S', 'A', 'M', 'X'] {
                count += 1;
            }
        }
    }

    count
}

fn day_03() {
    let content = read_from_file("day3.txt");

    let re = Regex::new(r"mul\((\d{1,3}),(\d{1,3})\)").unwrap();

    let multipliers: Vec<&str> = re.find_iter(&content).map(|m| m.as_str()).collect();

    let mut total = 0;

    for multiplier in multipliers {
        if let Some(captures) = re.captures(multiplier) {
            let x = captures.get(1).unwrap().as_str().parse::<i32>().unwrap();
            let y = captures.get(2).unwrap().as_str().parse::<i32>().unwrap();

            total += x * y;
        }
    }

    println!("total = {}", total)
}

fn day_02() {
    let content = read_from_file("day2.txt");

    let mut safe_levels = 0;

    for line in content.lines() {
        let vector: Vec<i32> = line
            .split(" ")
            .into_iter()
            .map(|elem| elem.parse().expect("could not parse integer"))
            .collect();

        if is_level_safe(vector) {
            safe_levels += 1;
        }
    }

    println!("{}", safe_levels)
}

fn is_level_safe(level: Vec<i32>) -> bool {
    let is_ascending = level[0] < level[1];

    for i in 0..level.len() - 1 {
        if is_ascending && !(level[i + 1] >= level[i] + 1 && level[i + 1] <= level[i] + 3) {
            return false;
        } else if !is_ascending && !(level[i] >= level[i + 1] + 1 && level[i] <= level[i + 1] + 3) {
            return false;
        }
    }

    true
}

fn day_01() {
    let content = read_from_file("day1.txt");

    let mut list1 = Vec::new();
    let mut list2 = Vec::new();

    for line in content.lines() {
        let vector: Vec<&str> = line.split(" ").collect();

        let first: u128 = vector[0].parse().expect("failed to parse first integer");
        let second: u128 = vector[1].parse().expect("failed to parse second integer");

        list1.push(first);
        list2.push(second);
    }

    list1.sort();
    list2.sort();

    let mut sum = 0;
    let n = list1.len();

    for i in 0..n {
        sum += list1[i].abs_diff(list2[i]);
    }

    println!("{}", sum);
}

fn read_from_file(file: &str) -> String {
    let input_path = Path::new(env!("CARGO_MANIFEST_DIR")).join("src/".to_string() + file);

    fs::read_to_string(&input_path).expect("could not open file")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn day2_test() {
        assert_eq!(true, is_level_safe(vec![7, 6, 4, 2, 1]));
        assert_eq!(false, is_level_safe(vec![1, 2, 7, 8, 9]));
        assert_eq!(false, is_level_safe(vec![9, 7, 6, 2, 1]));
        assert_eq!(false, is_level_safe(vec![1, 3, 2, 4, 5]));
        assert_eq!(false, is_level_safe(vec![8, 6, 4, 4, 1]));
        assert_eq!(true, is_level_safe(vec![1, 3, 6, 7, 9]));
    }
}
