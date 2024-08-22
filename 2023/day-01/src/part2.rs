use std::collections::BTreeMap;

use nom::AsChar;

use crate::custom_error::AocError;

#[tracing::instrument]
pub fn process(input: &str) -> miette::Result<String, AocError> {
    let lines: Vec<u32> = input.lines().map(parse_line).collect();
    Ok(lines.iter().sum::<u32>().to_string())
}

fn parse_line(line: &str) -> u32 {
    let mut line_numbers: Vec<u32> = Vec::new();
    let number_map = BTreeMap::from([
        ("one", 1),
        ("two", 2),
        ("three", 3),
        ("four", 4),
        ("five", 5),
        ("six", 6),
        ("seven", 7),
        ("eight", 8),
        ("nine", 9),
    ]);
    let number_keys: Vec<&str> = number_map.keys().clone().copied().collect();
    for x in 0..line.len() {
        let line_chunk = &line[x..];
        for key in &number_keys {
            if line_chunk.starts_with(*key) {
                line_numbers.push(*number_map.get(*key).unwrap());
            }
        }
        let next = line_chunk.chars().next().unwrap();
        if next.is_dec_digit() {
            line_numbers.push(next.to_digit(10).unwrap())
        }
    }

    let first = line_numbers.first().expect("There should be at least 1");
    let last = line_numbers
        .last()
        .or(Some(first))
        .expect("First is defined");
    format!("{first}{last}").parse().expect("Number")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process() -> miette::Result<()> {
        let input = "two1nine
eightwothree
abcone2threexyz
xtwone3four
4nineeightseven2
zoneight234
7pqrstsixteen";
        assert_eq!("281", process(input)?);
        Ok(())
    }
}
