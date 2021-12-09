fn split_input_into_int_vectors(s: String) -> Vec::<Vec<u8>> {
    let split_string = s.trim_end().split("\n");
    let vec = split_string
        .map(|i: &str| -> Vec<u8> {
            let bits = i.chars();
            let mut bit_vector = Vec::<u8>::new();
            for b in bits {
                bit_vector.push(b.to_digit(10).unwrap() as u8);
            };
            bit_vector
        })
        .collect::<Vec<Vec<u8>>>();
    vec
}

fn find_gamma_and_epsilon_vectors(input: Vec<Vec<u8>>) -> (Vec<u8>, Vec<u8>) {
    let input_length = input.len();
    let input_width = input[0].len();
    let mut sum_columns = vec![0 as u32; input_width];
    let mut g = vec![0;input_width];
    let mut e = vec![0;input_width];
    for i in input {
        for (j, v) in i.iter().enumerate() {
            sum_columns[j] += *v as u32;
        };
    };
    
    for (k,v) in sum_columns.iter().enumerate() {
        if (*v as u32) * 2 > input_length as u32 {
            g[k] = 1 as u8;
        } else {
            e[k] = 1 as u8;
        };
    };
    (g, e)
}

fn convert_int_vector_binary_representation_to_decimal(bin_vector: Vec<u8>) -> u32 {
    let bin_string = bin_vector.iter()
        .map(|x| x.to_string())
        .collect::<String>();
    let bin_int = u32::from_str_radix(&bin_string, 2).unwrap();
    bin_int
}

pub fn find_power_consumpation(input: String) -> u32 {
    let bin_vectors = split_input_into_int_vectors(input);
    let (gamma, epsilon) = find_gamma_and_epsilon_vectors(bin_vectors);
    let g_decimal = convert_int_vector_binary_representation_to_decimal(gamma);
    let e_decimal = convert_int_vector_binary_representation_to_decimal(epsilon);
    g_decimal * e_decimal
}