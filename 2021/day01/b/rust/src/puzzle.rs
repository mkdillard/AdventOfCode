fn split_string_on_newline_to_integer_vector(s: String) -> Vec<i32> {
    let split_string = s.trim_end().split("\n");
    let vec = split_string
        .map(|i: &str| -> i32 {
            i.parse::<i32>().unwrap()
        })
        .collect::<Vec<i32>>();
    vec
}

fn calculate_three_measurement_sums(input: Vec<i32>) -> Vec<i32> {
    let index_max = input.len() - 2;
    let mut sum_vector = Vec::<i32>::new();
    for (i, v) in input.iter().enumerate() {
        if i < index_max {
            sum_vector.push(v + input[i+1] + input[i+2]);
        }
    };
    sum_vector
}

pub fn count_increases(input: String) -> i32 {
    let levels = split_string_on_newline_to_integer_vector(input);
    let sum_levels = calculate_three_measurement_sums(levels);
    let mut increases_count: i32 = 0;
    sum_levels.iter().fold(sum_levels[0], | acc, x | {
        if x > &acc {
            increases_count += 1;
        };
        *x
    });
    increases_count
}

//  len == 10
// 0, 1, 2, 3, 4, 5, 6, 7, 8, 9