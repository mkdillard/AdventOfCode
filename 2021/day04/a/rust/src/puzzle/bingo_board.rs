#[derive(Debug)]
pub struct BingoBoard {
    numbers: Vec<Vec<i32>>,
    marked:  Vec<Vec<bool>>,
}

fn validate_input_board(ib: Vec<Vec<i32>>) -> (bool, i32) {
    let mut right_size = true;
    let mut bad_row_size: i32 = 0;
    if ib.len() != 5 {
        right_size = false;
    } else {
        for r in ib {
            if r.len() != 5 {
                right_size = false;
                bad_row_size = r.len() as i32;
                break;
            };
        };
    };
    (right_size, bad_row_size)
}

pub fn new_empty_board() -> BingoBoard {
    let numbers = vec![vec![0;5]; 5];
    let marked = vec![vec![false;5]; 5];
    BingoBoard { numbers: numbers, marked: marked}
}

impl BingoBoard {


    pub fn new(input: Vec<Vec<i32>>) -> BingoBoard {
        let mut bb = new_empty_board();
        let (right_size, bad_row_length) = validate_input_board(input.clone());
        if !right_size {
            eprintln!("BingoBoard expects 5 rows of 5 values each but received {} rows at least one of which had {} values, returning an empty board", input.len(), bad_row_length);
        } else {
            bb.numbers = input;
        }
        bb
    }



    pub fn new_from_int_vector(input: Vec<i32>) -> BingoBoard {
        let mut bb = new_empty_board();
        if input.len() != 25 {
            eprintln!("BingoBoard expects 25 input values, {} were provided, returning an empty board", input.len());
        } else {
            let mut row_index = 0;
            for (i, num) in input.iter().enumerate() {
                let col_index = i % 5;
                if col_index == 0 {
                    row_index += 1;
                };
                bb.numbers[row_index - 1][col_index] = *num;
            };
        };
        bb
    }

    pub fn print_board(&self) {
        println!("Board Numbers:");
        for r in &self.numbers {
            for (i, c) in r.iter().enumerate() {
                if c.to_string().len() == 1 {
                    print!(" ");
                }
                print!("{:?}", c);
                if i < 4 {
                    print!(" ");
                }
                
            }
            print!("\n");
        }
        println!("");
        
    }

    pub fn print_marked(&self) {
        println!("Marked Numbers:");
        for r in &self.marked {
            for (i, c) in r.iter().enumerate() {
                if c.to_string().len() == 1 {
                    print!(" ");
                }
                print!("{:?}", c);
                if i < 4 {
                    print!(" ");
                }
            }
            print!("\n");
        }
        println!("");
    }

    pub fn print(&self) {
        self.print_board();
        self.print_marked();
    }
}




