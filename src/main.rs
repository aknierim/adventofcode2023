pub mod input;
pub use crate::input::input::get_input;

fn main() {
    let input = get_input(1, 2022).expect("failed");
    println!("{}", input);
}
