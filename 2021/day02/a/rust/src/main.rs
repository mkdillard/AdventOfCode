mod puzzle;
mod util;

fn main() {
    let input = util::get_input("https://adventofcode.com/2021/day/2/input");
    let result = puzzle::determine_product_of_position(input);
    println!("Product of Forward and Depth: {}", result);
}
