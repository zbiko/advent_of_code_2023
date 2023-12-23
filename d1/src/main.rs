use std::{fs, panic::resume_unwind, str::FromStr};

fn substitute_literal_numbers(s: &str) -> String {
    let mut out: String = String::from("");
    let mut pos = 0;
    let mut found: bool;
    while pos < s.len() {
        found = false;
        'forloop: for p in pos..s.len() {
            let word = "one";
            match s[pos..p + 1].find(word) {
                Some(i) => {
                    out = out + &s[pos..pos + i] + "1";
                    pos = pos + i + 1;
                    found = true;
                    break 'forloop;
                }
                _ => (),
            };
            let word = "two";
            match s[pos..p + 1].find(word) {
                Some(i) => {
                    out = out + &s[pos..pos + i] + "2";
                    pos = pos + i + 1;
                    found = true;
                    break 'forloop;
                }
                _ => (),
            };
            let word = "three";
            match s[pos..p + 1].find(word) {
                Some(i) => {
                    out = out + &s[pos..pos + i] + "3";
                    pos = pos + i + 1;
                    found = true;
                    break 'forloop;
                }
                _ => (),
            };
            let word = "four";
            match s[pos..p + 1].find(word) {
                Some(i) => {
                    out = out + &s[pos..pos + i] + "4";
                    pos = pos + i + 1;
                    found = true;
                    break 'forloop;
                }
                _ => (),
            };
            let word = "five";
            match s[pos..p + 1].find(word) {
                Some(i) => {
                    out = out + &s[pos..pos + i] + "5";
                    pos = pos + i + 1;
                    found = true;
                    break 'forloop;
                }
                _ => (),
            };
            let word = "six";
            match s[pos..p + 1].find(word) {
                Some(i) => {
                    out = out + &s[pos..pos + i] + "6";
                    pos = pos + i + 1;
                    found = true;
                    break 'forloop;
                }
                _ => (),
            };
            let word = "seven";
            match s[pos..p + 1].find(word) {
                Some(i) => {
                    out = out + &s[pos..pos + i] + "7";
                    pos = pos + i + 1;
                    found = true;
                    break 'forloop;
                }
                _ => (),
            };
            let word = "eight";
            match s[pos..p + 1].find(word) {
                Some(i) => {
                    out = out + &s[pos..pos + i] + "8";
                    pos = pos + i + 1;
                    found = true;
                    break 'forloop;
                }
                _ => (),
            };
            let word = "nine";
            match s[pos..p + 1].find(word) {
                Some(i) => {
                    out = out + &s[pos..pos + i] + "9";
                    pos = pos + i + 1;
                    found = true;
                    break 'forloop;
                }
                _ => (),
            };
        }
        if !found {
            out = out + &s[pos..];
            pos = s.len()
        }
    }
    out
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_substitute_literal_numbers() {
        assert_eq!(substitute_literal_numbers("onetwothree"), "1ne2wo3hree");
    }

    #[test]
    fn test_substitute_literal_numbers_2() {
        assert_eq!(
            substitute_literal_numbers("blablaonetwothreeblabla"),
            "blabla1ne2wo3hreeblabla"
        );
    }

    #[test]
    fn test_substitute_literal_numbers_3() {
        assert_eq!(
            substitute_literal_numbers("12eightwothreefiveight"),
            "128igh2wo3hree5iv8ight"
        );
    }
}

fn main() {
    let contents = fs::read_to_string("input_day_1.txt").expect("should read the file");

    let mut sum = 0;

    for l in contents.lines() {
        let mut modified_line: String = l.to_string();

        modified_line = substitute_literal_numbers(modified_line.as_str());

        let first_digit = modified_line
            .chars()
            .nth(modified_line.find(|c: char| c.is_ascii_digit()).unwrap())
            .unwrap()
            .to_digit(10)
            .unwrap();
        let last_digit = modified_line
            .chars()
            .nth(modified_line.rfind(|c: char| c.is_ascii_digit()).unwrap())
            .unwrap()
            .to_digit(10)
            .unwrap();
        // println!("{}{}", first_digit, last_digit);
        sum = sum + first_digit * 10 + last_digit;
    }
    println!("SUM is {}", sum);
}
