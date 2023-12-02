use std::cmp;
use std::collections::HashMap;

#[path = "../input/mod.rs"]
mod input;
use aoc23::input::input::get_input;

fn day_2() -> (u32, u32) {
    let input = get_input(2, 2023).unwrap();

    let lookup = HashMap::from([
        ("red", 12),
        ("green", 13),
        ("blue", 14)
    ]);

    let lines = input.lines();

    let mut ans_1: u32 = 0;
    let mut ans_2: u32 = 0;

    for line in lines {
        let mut is_valid = true;
        let mut score: u32 = 1;
        let mut hm: HashMap<&str, u32> = HashMap::from([
            ("red", 0),
            ("green", 0),
            ("blue", 0)
        ]);

        let (game, line) = line.
            trim()
            .split_once(":")
            .unwrap();

        for event in line.split(";") {
            for cube in event.split(",") {
                let (count, color) = cube.
                    trim().
                    split_once(" ").
                    unwrap();

                let count = count
                    .parse::<u32>()
                    .unwrap();

                hm.insert(color, cmp::max(*hm.get(color).unwrap(), count));

                if count > *lookup.get(color).unwrap() {
                    is_valid = false;
                }
            }
        }

        let (_, id) = game.split_once(" ").unwrap();

        if is_valid {
            ans_1 += id.parse::<u32>().unwrap();
        }

        for val in hm.values() {
            score *= val;
        }
        ans_2 += score;
    }
    return (ans_1, ans_2);
}

fn main() {
    let (ans_1, ans_2): (u32, u32) = day_2();
    println!("{}, {}", ans_1, ans_2);
}
