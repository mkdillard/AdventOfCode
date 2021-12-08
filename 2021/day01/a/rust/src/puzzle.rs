fn split_string_on_newline_to_integer_vector(s: String) -> Vec<i32> {
    let split_string = s.trim_end().split("\n");
    let vec = split_string
        .map(|i: &str| -> i32 {
            i.parse::<i32>().unwrap()
        })
        .collect::<Vec<i32>>();
    vec
}

pub fn count_increases(input: String) -> i32 {
    let levels = split_string_on_newline_to_integer_vector(input);
    let mut increases_count: i32 = 0;
    levels.iter().fold(levels[0], | acc, x | {
        if x > &acc {
            increases_count += 1;
        };
        *x
    });
    increases_count
}