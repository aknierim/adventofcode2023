use aho_corasick::AhoCorasick;
use std::time::Instant;

#[path = "../input/mod.rs"]
mod input;
use input::input::get_input;

fn part_1() -> i32 {
    let input = get_input(1, 2023).unwrap();

    let lns = input.lines();

    let mut vec = vec![];
    for ln in lns {
        let digits: String = ln.chars().filter(|char| char.is_digit(10)).collect();
        let mut dgt = String::from(digits.chars().next().unwrap());
        dgt.push(digits.chars().last().unwrap());

        vec.push(dgt.parse::<i32>().unwrap());
    }

    return vec.iter().sum::<i32>();
}

fn part_2() -> i32 {
    let input = get_input(1, 2023).unwrap();

    let lns = input.lines();

    let patterns = &[
        "one", "two", "three", "four", "five", "six", "seven", "eight", "nine",
    ];
    // I do not like this, but at this point
    // I also do not care anymore :D
    let patterns_inv = &[
        "eno", "owt", "eerht", "ruof", "evif", "xis", "neves", "thgie", "enin",
    ];
    let replace_pattern = &["1", "2", "3", "4", "5", "6", "7", "8", "9"];
    let mut vec = vec![];

    for ln in lns {
        let ln_coll = ln.chars().rev().collect::<String>();
        let ln_inv: &str = &ln_coll[..];

        let ac = AhoCorasick::new(patterns);
        let ac_inv = AhoCorasick::new(patterns_inv);

        let tmp = ac.expect("failed").replace_all(ln, replace_pattern);
        let tmp_inv = ac_inv.expect("failed").replace_all(ln_inv, replace_pattern);

        let digits: String = tmp.chars().filter(|char| char.is_digit(10)).collect();
        let digits_inv: String = tmp_inv.chars().filter(|char| char.is_digit(10)).collect();

        let mut dgt = String::from(digits.chars().next().unwrap());
        let dgt_inv = digits_inv.chars().next().unwrap();

        dgt.push(dgt_inv);

        vec.push(dgt.parse::<i32>().unwrap());
    }

    return vec.iter().sum::<i32>();
}

fn main() {
    let now = Instant::now();

    let ans_1: i32 = part_1();

    let elapsed_time = now.elapsed();
    println!(
        "Running part_1() took {} seconds. Answer: {}",
        elapsed_time.as_secs(),
        ans_1
    );

    let now = Instant::now();

    let ans_2: i32 = part_2();

    let elapsed_time = now.elapsed();
    println!(
        "Running part_2() took {} seconds. Answer: {}",
        elapsed_time.as_secs(),
        ans_2
    );
}
