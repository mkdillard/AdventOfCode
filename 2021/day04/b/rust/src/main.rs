mod puzzle;
mod util;

fn main() {
    let input = util::get_input("https://adventofcode.com/2021/day/4/input");
    let result = puzzle::find_power_consumpation(input);
    println!("Submarine Power Consumption is: {}", result);
}
