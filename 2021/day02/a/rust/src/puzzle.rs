use std::slice::Iter;

fn split_string_on_newline<'a>(s: &'a String) -> Vec<&'a str> {
    let split_string = s.trim_end().split("\n");
    let vec = split_string.collect::<Vec<&str>>();
    vec
}

fn determine_horizontal_position(moves: Iter<&str>) -> i32 {
    let horizontal_position = moves.filter(|m| m.contains("forward"))
        .map(|m: &&str| -> i32 {
            let m_string = m.to_string();
            let split_m = m_string.split(" ").collect::<Vec<&str>>();
            split_m[1].parse::<i32>().unwrap()
        })
        .fold(0, | acc, m | {
            acc + m
        });
    horizontal_position
}

fn determine_depth_increase(moves: Iter<&str>) -> i32 { 
    let vertical_shift_down = moves.filter(|m| m.contains("down"))
        .map(|m: &&str| -> i32 {
            let m_string = m.to_string();
            let split_m = m_string.split(" ").collect::<Vec<&str>>();
            split_m[1].parse::<i32>().unwrap()
        })
        .fold(0, | acc, m | {
            acc + m
        });
    vertical_shift_down
}

fn determine_depth_decrease(moves: Iter<&str>) -> i32 {
    let vertical_shift_up = moves.filter(|m| m.contains("up"))
    .map(|m: &&str| -> i32 {
        let m_string = m.to_string();
        let split_m = m_string.split(" ").collect::<Vec<&str>>();
        split_m[1].parse::<i32>().unwrap()
    })
    .fold(0, | acc, m | {
        acc + m
    });
    vertical_shift_up
}

// fn determine_depth_position(moves: Iter<&str>) -> i32 {
//     let up_total = determine_depth_decrease(moves);
//     let down_total = determine_depth_increase(moves);
//     down_total - up_total
// }

pub fn determine_product_of_position(input: String) -> i32 {
    let moves = split_string_on_newline(&input);
    let h_forward = determine_horizontal_position(moves.iter());
    let d_up = determine_depth_decrease(moves.iter());
    let d_down = determine_depth_increase(moves.iter());
    println!("Forward: {}\nUp: {}\nDown: {}\nDepth: {}", h_forward, d_up, d_down, d_down - d_up);
    h_forward * (d_down - d_up)
}