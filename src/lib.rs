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

    struct Test {
        input: String,
        expect: Result<i64, AddError>,
    }

    #[test]
    fn test_add() {
        let tests = vec![
            Test {
                input: "".to_string(),
                expect: Ok(0),
            },
            Test {
                input: "0".to_string(),
                expect: Ok(0),
            },
            Test {
                input: "1".to_string(),
                expect: Ok(1),
            },
            Test {
                input: "0, 1".to_string(),
                expect: Ok(1),
            },
            Test {
                input: "0, 1,  2".to_string(),
                expect: Ok(3),
            },
            Test {
                input: "0, 1,  2, 3".to_string(),
                expect: Ok(6),
            },
            Test {
                input: "1\n2,3".to_string(),
                expect: Ok(6),
            },
            Test {
                input: "1,\n".to_string(),
                expect: Err(AddError::ConsecutiveSeparators),
            },
            Test {
                input: "1\n,2".to_string(),
                expect: Err(AddError::ConsecutiveSeparators),
            },
            Test {
                input: "//".to_string(),
                expect: Err(AddError::InvalidInput),
            },
            Test {
                input: "//;\n1;2".to_string(),
                expect: Ok(3),
            },
            Test {
                input: "//h\n1h3".to_string(),
                expect: Ok(4),
            },
            Test {
                input: "//\n\n1\n4".to_string(),
                expect: Ok(5),
            },
            Test {
                input: "//\n\n1\n\n2".to_string(),
                expect: Err(AddError::ConsecutiveSeparators),
            },
            Test {
                input: "//;\n1;2;-3".to_string(),
                expect: Err(AddError::HasNegative(vec![-3])),
            },
            Test {
                input: "0, 1, 2, -3".to_string(),
                expect: Err(AddError::HasNegative(vec![-3])),
            },
            Test {
                input: "0, 1, -2, -3".to_string(),
                expect: Err(AddError::HasNegative(vec![-2, -3])),
            },
            Test {
                input: "//;\n1;2000".to_string(),
                expect: Ok(1),
            },
            Test {
                input: "0, 1, 2000, 1001".to_string(),
                expect: Ok(1),
            },
        ];

        for test in tests {
            assert_eq!(test.expect, add(&test.input))
        }
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
