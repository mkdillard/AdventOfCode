mod bingo_board::BingoBoard;

fn split_input_into_boards(s: String) -> (Vec<i32>, Vec::<BingoBoard>) {
    //Vec<i32>, Vec::<Vec::<Vec<i32>>>
    let mut boards = Vec::<BingoBoard>::new();
    let mut split_string = s.trim_end().split("\n");
    let called_numbers = split_string.next()
        .unwrap()
        .trim_end()
        .split(",")
        .map(|s| s.parse::<i32>().unwrap())
        .collect::<Vec<i32>>();

    let mut board_input = Vec::<Vec::<i32>>::new();
    boards = split_string.fold(boards, |mut acc, s| {
        let s_trim = s.trim();
        if s_trim.len() > 0 {
            let new_row = s_trim
            .split(" ")
            .filter(|num_string| num_string.len() > 0)
            .map(|num_string| {
                num_string.parse::<i32>().unwrap()
            })
            .collect::<Vec<i32>>();

            board_input.push(new_row);
        }
        if board_input.len() == 5 {
            let new_board = BingoBoard::new(board_input.clone());
            acc.push(new_board);
            board_input.clear();
        };
        acc
    });
    println!("{:?}", called_numbers);
    for b in boards {
        b.print();
    }
    (called_numbers, boards)
}

fn determine_winning_board(cn: Vec<i32>, boards: Vec<BingoBoard>) {
    for n in cn {
        //loop over boards
        //mark board return true if on board or false if not
        //check board for win
        //if winner return board index
    }
}

pub fn find_final_board_score(input: String) -> i32 {
    let (called_numbers, boards) = split_input_into_boards(input);
    0
}