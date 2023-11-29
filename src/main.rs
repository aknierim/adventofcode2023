pub mod input;
pub use crate::input::input::get_input;

fn main() -> std::io::Result<()> {
    let input = get_input(1, 2022).expect("Failed to fetch input data!");
    println!("{}", input);

    Ok(())
}
