use regex::Regex;
use thiserror::Error;

#[derive(Error, Debug, PartialEq, Eq, Clone)]
pub enum AddError {
    #[error("consecutive separators detected")]
    ConsecutiveSeparators,
    #[error("invalid input")]
    InvalidInput,
    #[error("not a number")]
    NaN,
    #[error("input has negative number {0:?}")]
    HasNegative(Vec<i64>),
}

fn check_negatives(ns: Vec<i64>) -> Result<(), AddError> {
    let negatives = ns
        .into_iter()
        .filter(|n| n.is_negative())
        .collect::<Vec<i64>>();

    if !negatives.is_empty() {
        return Err(AddError::HasNegative(negatives));
    }
    return Ok(());
}

fn check_consecutive_separators(input: &str, seps: Vec<char>) -> Result<(), AddError> {
    let mut is_separator_detected = false;
    for c in input.chars() {
        // Skip the whitespace
        if c == ' ' {
            continue;
        }
        if seps.contains(&c) {
            if is_separator_detected {
                return Err(AddError::ConsecutiveSeparators);
            }
            is_separator_detected = true;
            continue;
        }
        is_separator_detected = false;
    }
    return Ok(());
}

fn normalize_numbers(numbers: Vec<i64>) -> Vec<i64> {
    numbers
        .into_iter()
        .map(|n| if n <= 1000 { n } else { 0 })
        .collect()
}

fn tokenizer(input: &str, sep: &str) -> Result<Vec<i64>, AddError> {
    if input.is_empty() {
        return Ok(vec![]);
    }
    let re = Regex::new(&format!(r"[{sep}]")).unwrap();

    let res: Result<Vec<i64>, AddError> = re
        .split(input)
        .map(|s| s.trim())
        .map(|c| c.parse::<i64>().map_err(|_| AddError::NaN))
        .collect();

    let Ok(numbers) = res.clone() else {
        return res;
    };
    check_negatives(numbers.clone())?;

    Ok(normalize_numbers(numbers))
}

fn handle_custom_separator(input: &str) -> Result<i64, AddError> {
    let sep = input.chars().nth(2).ok_or(AddError::InvalidInput)?;
    // Skip 4 characters which is "//[delimiter]\n
    let s = input.chars().skip(4).collect::<String>();
    // If it has empty so it has consecutive separators.
    check_consecutive_separators(&s, vec![sep])?;
    let tokens = tokenizer(&s, &sep.to_string())?;
    return Ok(tokens.into_iter().sum());
}

fn handle_default_seperator(input: &str) -> Result<i64, AddError> {
    check_consecutive_separators(input, vec![',', '\n'])?;

    let tokens = tokenizer(input, ",\n")?;

    return Ok(tokens.into_iter().sum());
}

pub fn add(input: &str) -> Result<i64, AddError> {
    // There is custom delimiter
    if input.starts_with("//") {
        return handle_custom_separator(input);
    }
    return handle_default_seperator(input);
}

#[cfg(test)]
mod tests {
    use super::*;
    use proptest::prelude::*;

    #[test]
    fn empty_string() {
        assert_eq!(Ok(0), add(""));
    }

    #[test]
    fn one_number_string() {
        assert_eq!(Ok(1), add("1"));
        assert_eq!(Ok(0), add("0"));
    }

    #[test]
    fn more_number_string() {
        assert_eq!(Ok(1), add("0, 1"));
        assert_eq!(Ok(3), add("0, 1, 2"));
        assert_eq!(Ok(6), add("0, 1, 2, 3"));
    }

    #[test]
    fn newline_seprator() {
        assert_eq!(Ok(6), add("1\n2,3"));
    }

    #[test]
    fn no_consecutive_delimiter() {
        assert_eq!(Err(AddError::ConsecutiveSeparators), add("1,\n"));
        assert_eq!(Err(AddError::ConsecutiveSeparators), add("1,\n2"));
        assert_eq!(Err(AddError::ConsecutiveSeparators), add("1\n,2"));
    }

    #[test]
    fn invalid_input() {
        assert_eq!(Err(AddError::InvalidInput), add("//"));
    }

    #[test]
    fn custom_delimiter() {
        assert_eq!(Ok(3), add("//;\n1;2"));
        assert_eq!(Ok(4), add("//h\n1h3"));
        assert_eq!(Ok(5), add("//\n\n1\n4"));
        assert_eq!(Err(AddError::ConsecutiveSeparators), add("//\n\n1\n\n2"));
    }

    #[test]
    fn negatives() {
        assert_eq!(Err(AddError::HasNegative(vec![-3])), add("//;\n1;2;-3"));
        assert_eq!(Err(AddError::HasNegative(vec![-3])), add("0, 1, 2, -3"));

        assert_eq!(
            Err(AddError::HasNegative(vec![-2, -3])),
            add("0, 1, -2, -3")
        );
    }

    #[test]
    fn bigger_than_1000() {
        assert_eq!(Ok(1), add("//;\n1;2000"));
        assert_eq!(Ok(1), add("0, 1, 2000, 1001"));
    }
    proptest! {
        #[test]
        fn doesnt_crash_on_random_input(s: String) {
            let _ = add(&s);
        }

        #[test]
        fn sum_is_commutative(a in 0i64..1000, b in 0i64..1000) {
            let input1 = format!("{},{}", a, b);
            let input2 = format!("{},{}", b, a);
            assert_eq!(add(&input1), add(&input2));
        }
    }
}
