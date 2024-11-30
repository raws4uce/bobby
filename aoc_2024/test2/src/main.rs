use std::u128;

use anyhow::Result;
use regex::Regex;
use util_aoc::aoc_io;
fn main() -> Result<()> {
    aoc_io::lickballs();
    let re = Regex::new(r"(?i)(one|two|three|four|five|six|seven|eight|nine)").unwrap();
    //aoc_io::process_file("test.txt".to_string(), |line| {});
    let mut ans: u128 = 0;
    let _ = aoc_io::process_file("test.txt".to_string(), |line| {
        let mut temp = String::new();
        let mut temp_str = String::new();
        for (i, _) in line.chars().into_iter().enumerate() {
            if let Some(char) = line.chars().nth(i) {
                if char.is_ascii_digit() {
                    //fine jere                    println!("E");
                    temp.push(char);
                    temp_str.clear();
                } else {
                    //fine here      HIVWIUF = HIVWIUF + 1;
                    temp_str.push(char);
                    if temp_str.len() > 2 {
                        if let Some(nom) = is_nom_but_like_a_word_instead_of_a_digit_mk2(
                            temp_str.to_lowercase().trim(),
                            re.clone(),
                        ) {
                            temp.push(nom);
                            temp_str.clear();
                        }
                    }
                }
            }
        }
        println!("{}", temp);
        if temp.len() == 1 {
            temp.push(temp.chars().next().unwrap());
        } else {
            println!("343434 fffre {}", temp);

            let n1 = temp.chars().next().unwrap();
            let n2 = temp.chars().last().unwrap();
            temp = format!("{n1}{n2}");
        }
        match temp.trim().parse::<u128>() {
            Ok(x) => {
                ans = ans + x;
            }
            Err(_) => {}
        };
    });
    println!("ans {}", ans);
    Ok(())
}

//i aint gonna lie, this here chat gpt'd, Regex is too much for me rn, one day it would be learnt,
//but till that day... llm our lord an saviour, llm's we trust, everything else was me tho, let
//that be known, I duno if you could tel ;>
fn is_nom_but_like_a_word_instead_of_a_digit_mk2(s: &str, re: Regex) -> Option<char> {
    if let Some(captures) = re.captures(s) {
        match &captures[1].to_lowercase()[..] {
            "one" => Some('1'),
            "two" => Some('2'),
            "three" => Some('3'),
            "four" => Some('4'),
            "five" => Some('5'),
            "six" => Some('6'),
            "seven" => Some('7'),
            "eight" => Some('8'),
            "nine" => Some('9'),
            _ => None,
        }
    } else {
        None
    }
}

//not in use
//fn is_nom_but_like_a_word_instead_of_a_digit(s: &str) -> Option<char> {
//    let one = Regex::new(r"(?i)\bone\b").unwrap();
//    let two = Regex::new(r"(?i)\btwo\b").unwrap();
//    let three = Regex::new(r"(?i)\bthree\b").unwrap();
//    let four = Regex::new(r"(?i)\bfour\b").unwrap();
//    let five = Regex::new(r"(?i)\bfive\b").unwrap();
//    let six = Regex::new(r"(?i)\bsix\b").unwrap();
//    let seven = Regex::new(r"(?i)\bseven\b").unwrap();
//    let eight = Regex::new(r"(?i)\beight\b").unwrap();
//    let nine = Regex::new(r"(?i)\bnine\b").unwrap();
//
//    //try shave?
//
//    match s {
//        _ if one.is_match(s) => Some('1'),
//        _ if two.is_match(s) => Some('2'),
//        _ if three.is_match(s) => Some('3'),
//        _ if four.is_match(s) => Some('4'),
//        _ if five.is_match(s) => Some('5'),
//        _ if six.is_match(s) => Some('6'),
//        _ if seven.is_match(s) => Some('7'),
//        _ if eight.is_match(s) => Some('8'),
//        _ if nine.is_match(s) => Some('9'),
//        _ => None,
//    }
//}
