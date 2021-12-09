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

fn find_most_common_oxygen_value_in_column(input: Vec<Vec<u32>>) -> Vec<u32> {
    let input_length = input.len();
    let input_width = input[0].len();
    let mut most_common_digit = vec![0 as u32; input_width];
    let mut sum_columns = vec![0 as u32; input_width];
    for i in input {
        for (j, v) in i.iter().enumerate() {
            sum_columns[j] += *v as u32;
        };
    };
    
    for (k,v) in sum_columns.iter().enumerate() {
        println!("oxy_sums[{}]: {}", k, v);
        if (*v as u32) * 2 >= input_length as u32 {
            most_common_digit[k] = 1;
        }
    };
    println!("oxy_map: {:?}", most_common_digit);
    most_common_digit
}

fn filter_oxygen_input(input: Vec<Vec<u32>>, filter_mask: Vec<u32>) -> Vec<u32> {
    let filter_length = filter_mask.len();
    let mut oxygen_input = input;
    for i in 0..filter_length {
        println!("oxy i: {} len: {}", i, oxygen_input.len());
        if oxygen_input.len() == 1 { println!("{}", i); break; };
        oxygen_input = oxygen_input
            .into_iter()
            .filter(|v| v[i] == filter_mask[i])
            .collect::<Vec<Vec<u32>>>();
    }
    println!("{:?}", oxygen_input[0]);
    oxygen_input[0].clone()
}

fn find_most_common_co2_value_in_column(input: Vec<Vec<u32>>) -> Vec<u32> {
    let input_length = input.len();
    let input_width = input[0].len();
    let mut most_common_digit = vec![0 as u32; input_width];
    let mut sum_columns = vec![0 as u32; input_width];
    for i in input {
        for (j, v) in i.iter().enumerate() {
            sum_columns[j] += *v as u32;
        };
    };
    
    for (k,v) in sum_columns.iter().enumerate() {
        println!("co2_sums[{}]: {}", k, v);
        if (*v as u32) * 2 < input_length as u32 {
            most_common_digit[k] = 1;
        }
    };
    println!("co2_map: {:?}", most_common_digit);
    most_common_digit
}

fn filter_co2_input(input: Vec<Vec<u32>>, filter_mask: Vec<u32>) -> Vec<u32> {
    let filter_length = filter_mask.len();
    let mut co2_input = input;
    for i in 0..filter_length {
        println!("c02 i: {} len: {}", i, co2_input.len());
        if co2_input.len() == 1 { println!("{}", i); break; };
        co2_input = co2_input
            .into_iter()
            .filter(|v| v[i] == filter_mask[i])
            .collect::<Vec<Vec<u32>>>();
    }
    println!("{:?}", co2_input[0]);
    co2_input[0].clone()
}

fn convert_int_vector_binary_representation_to_decimal(bin_vector: Vec<u32>) -> u32 {
    let bin_string = bin_vector.iter()
        .map(|x| x.to_string())
        .collect::<String>();
    println!("{}", bin_string);
    let bin_int = u32::from_str_radix(&bin_string, 2).unwrap();
    bin_int
}

pub fn find_power_consumpation(input: String) -> u32 {
    let bin_vectors = split_input_into_int_vectors(input);
    let oxy_common_map = find_most_common_oxygen_value_in_column(bin_vectors.clone());
    let oxygen_bin_vec = filter_oxygen_input(bin_vectors.clone(), oxy_common_map);
    let oxygen_generator_rating = convert_int_vector_binary_representation_to_decimal(oxygen_bin_vec);
    let co2_common_map = find_most_common_co2_value_in_column(bin_vectors.clone());
    let co2_bin_vec = filter_co2_input(bin_vectors.clone(), co2_common_map);
    let co2_scrubber_rating = convert_int_vector_binary_representation_to_decimal(co2_bin_vec);
    println!("o_gen: {}\nco2_scrub: {}", oxygen_generator_rating, co2_scrubber_rating);
    oxygen_generator_rating * co2_scrubber_rating
}