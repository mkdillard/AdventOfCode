fn split_input_into_int_vectors(s: String) -> Vec::<Vec<u32>> {
    let split_string = s.trim_end().split("\n");
    let vec = split_string
        .map(|i: &str| -> Vec<u32> {
            let bits = i.chars();
            let mut bit_vector = Vec::<u32>::new();
            for b in bits {
                bit_vector.push(b.to_digit(10).unwrap());
            };
            bit_vector
        })
        .collect::<Vec<Vec<u32>>>();
    vec
}

fn find_most_common_oxygen_value_in_column(input: Vec<Vec<u32>>, column: usize) -> u32 {
    let input_length = input.len();
    let mut result = 0;
    let mut sum_column = 0;
    for i in input {
        sum_column += i[column];
    };
    if sum_column * 2 >= input_length as u32 {
        result = 1;
    };
    result
}

fn filter_oxygen_input(input: Vec<Vec<u32>>) -> Vec<u32> {
    let filter_length = input[0].len();
    let mut oxygen_input = input;
    for i in 0..filter_length {
        if oxygen_input.len() == 1 { break; };
        let match_bit = find_most_common_oxygen_value_in_column(oxygen_input.clone(), i);
        oxygen_input = oxygen_input
            .into_iter()
            .filter(|v| v[i] == match_bit)
            .collect::<Vec<Vec<u32>>>();
    }
    oxygen_input[0].clone()
}

fn find_most_common_co2_value_in_column(input: Vec<Vec<u32>>, column: usize) -> u32 {
    let input_length = input.len();
    let mut result = 0;
    let mut sum_column = 0;
    for i in input {
        sum_column += i[column];
    };
    if sum_column * 2 < input_length as u32 {
        result = 1;
    };
    result
}

fn filter_co2_input(input: Vec<Vec<u32>>) -> Vec<u32> {
    let filter_length = input[0].len();
    let mut co2_input = input;
    for i in 0..filter_length {
        if co2_input.len() == 1 { break; };
        let match_bit = find_most_common_co2_value_in_column(co2_input.clone(), i);
        co2_input = co2_input
            .into_iter()
            .filter(|v| v[i] == match_bit)
            .collect::<Vec<Vec<u32>>>();
    }
    co2_input[0].clone()
}

fn convert_int_vector_binary_representation_to_decimal(bin_vector: Vec<u32>) -> u32 {
    let bin_string = bin_vector.iter()
        .map(|x| x.to_string())
        .collect::<String>();
    let bin_int = u32::from_str_radix(&bin_string, 2).unwrap();
    bin_int
}

pub fn find_power_consumpation(input: String) -> u32 {
    let bin_vectors = split_input_into_int_vectors(input);
    let oxygen_bin_vec = filter_oxygen_input(bin_vectors.clone());
    let oxygen_generator_rating = convert_int_vector_binary_representation_to_decimal(oxygen_bin_vec);
    let co2_bin_vec = filter_co2_input(bin_vectors.clone());
    let co2_scrubber_rating = convert_int_vector_binary_representation_to_decimal(co2_bin_vec);
    println!("o_gen: {}\nco2_scrub: {}", oxygen_generator_rating, co2_scrubber_rating);
    oxygen_generator_rating * co2_scrubber_rating
}