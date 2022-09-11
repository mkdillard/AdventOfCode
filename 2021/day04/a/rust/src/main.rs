mod puzzle;
mod util;

fn main() {
    let input = util::get_input("https://adventofcode.com/2021/day/4/input");
    let result = puzzle::find_final_board_score(input);
    println!("Winning Bingo board score is: {}", result);
}
