use std::{env, fs};
use reqwest::blocking::Client;

const INPUT_DIR: &str = "input_data";
const BASE_URL: &str = "https://adventofcode.com";


pub fn get_input(day: u8, year: u16) -> Result<String, reqwest::Error> {
    let input_file = format!("{}/{}.txt", INPUT_DIR, day);

    // check if file already contains data
    // and return input if true
    if let Ok(input) = fs::read_to_string(input_file) {
        return Ok(input)
    }

    let url: String = format!("{}/{}/day/{}/input", BASE_URL, year, day);

    let cookie = env::var("AOC_SESSION").expect(
        "Could not find AOC_SESSION env variable! Please set your AOC \
        cookie as the AOC_SESSION env variable"
    );

    let input = Client::new()
        .get(url)
        .header("Cookie", format!("session={}", cookie))
        .send()?
        .text();

    fs::create_dir_all(INPUT_DIR).expect(
        &format!("Could not create directory '{}'", INPUT_DIR)
    );
    fs::write(format!("{}/day_{}", INPUT_DIR, day), input.as_ref().unwrap());

    input
}

