mod puzzle;
mod util;

fn main() {
    let input = util::get_input("https://adventofcode.com/2021/day/1/input");
    let result = puzzle::count_increases(input);
    println!("Number of increases: {}", result);
}
