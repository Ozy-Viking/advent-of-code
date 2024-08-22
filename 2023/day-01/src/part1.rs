use nom::AsChar;

use crate::custom_error::AocError;

#[tracing::instrument]
pub fn process(input: &str) -> miette::Result<String, AocError> {
    let lines: Vec<u32> = input
        .lines()
        .map(|line| {
            let mut it = line.chars().filter(|c| c.is_dec_digit());
            let first = it.next().expect("There should be at least 1");
            let last = it.last().or(Some(first)).expect("First is defined");
            format!("{first}{last}").parse().expect("Number")
        })
        .collect();
    Ok(lines.iter().sum::<u32>().to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process() -> miette::Result<()> {
        let input = "1abc2
pqr3stu8vwx
a1b2c3d4e5f
treb7uchet";
        assert_eq!("142", process(input)?);
        Ok(())
    }
}
