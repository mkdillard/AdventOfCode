fn split_string_on_newline<'a>(s: &'a String) -> Vec<&'a str> {
    let split_string = s.trim_end().split("\n");
    let vec = split_string.collect::<Vec<&str>>();
    vec
}

fn determine_final_position_with_aim(moves: Vec<&str>) -> Vec<i32> {
    let mut aim = 0;
    let mut h_position = 0;
    let mut depth = 0;
    for m in &moves {
        let m_string = m.to_string();
        let split_m = m_string.split(" ").collect::<Vec<&str>>();
        let m_int = split_m[1].parse::<i32>().unwrap();
        match split_m[0] {
            "forward" => {
                h_position += m_int;
                depth += aim*m_int;
            },
            "up" => {
                aim -= m_int;
            },
            "down" => {
                aim += m_int;
            },
            _ => ()
        }
    };
    vec![h_position, depth]
}

pub fn determine_product_of_position(input: String) -> i32 {
    let moves = split_string_on_newline(&input);
    let position = determine_final_position_with_aim(moves);
    position[0]*position[1]
}