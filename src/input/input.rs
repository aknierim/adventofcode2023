use std::{env, fs, path::Path, error::Error};
use reqwest::blocking::Client;

const INPUT_DIR: &str = "input_data";
const BASE_URL: &str = "https://adventofcode.com";


/// Fetches and returns the input data of a given day
///
/// # Arguments
///
/// * `day` - An integer that sets the day to fetch the input for.
/// * `year` - An integer that sets the year of the AoC.
///
/// # Examples
///
/// ```
/// pub use input::get_input;
/// let input = get_input(1, 2023)
/// ```
pub fn get_input(
    day: u8,
    year: u16
) -> Result<String, Box<dyn Error>> {
    let input_file = format!("{}/day_{}.txt", INPUT_DIR, day);

    // check if file already contains data
    // and return input if true
    if let Ok(input) = fs::read_to_string(input_file.clone()) {
        println!("Data found! Reading from file...");
        return Ok(input)
    }

    // Set URL and get session cookie
    let url: String = format!("{}/{}/day/{}/input", BASE_URL, year, day);
    let cookie: String = env::var("AOC_SESSION").expect(
        "Could not find AOC_SESSION env variable! Please set your AOC \
        cookie as the AOC_SESSION env variable"
    );

    // fetch input of the day
    println!("Fetching input data for day {}...", day);
    let input = Client::new()
        .get(url)
        .header("Cookie", format!("session={}", cookie))
        .send()?
        .text();

    // check if input dir exists
    if !!!Path::new(INPUT_DIR).is_dir() {
        println!("Creating directory '{}'...\n", INPUT_DIR);
        fs::create_dir_all(INPUT_DIR).expect(
            &format!("Could not create directory '{}'", INPUT_DIR)
        );
    }

    // write input to file
    let _ = fs::write(input_file, input.as_ref().unwrap());

    Ok(input?)
}

