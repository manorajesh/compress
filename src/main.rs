use num::{BigInt, FromPrimitive, Zero};

fn pi_digit_at_position(position: u32) -> u32 {
    // Calculate pi using the BBP formula
    let mut pi = BigInt::zero();
    for k in 0..=position {
        let term1 = BigInt::from_u32(4).unwrap() / (BigInt::from_u32(8).unwrap() * k + BigInt::from_u32(1).unwrap());
        let term2 = BigInt::from_u32(2).unwrap() / (BigInt::from_u32(8).unwrap() * k + BigInt::from_u32(4).unwrap());
        let term = (term1 - term2) * BigInt::from_u32(16).unwrap().pow(position - k);
        pi += term;
    }
    
    // Return the digit at the specified position
    let pi_str = pi.to_string();
    pi_str.chars()
        .nth(position as usize)
        .unwrap()
        .to_digit(10)
        .unwrap()
}

fn compress(input_data: &Vec<u8>) -> Vec<u32> {
    let mut output_data: Vec<u32> = Vec::with_capacity(input_data.len());

    for byte in input_data {
        let mut byte = *byte;
        for _ in 0..8 {
            let digit = pi_digit_at_position(byte as u32);
            output_data.push(digit);
            byte >>= 1;
        }
    }

    output_data
}

fn decompress(input_data: Vec<u32>) -> Vec<u8> {
    let mut output_data: Vec<u8> = Vec::with_capacity(input_data.len() / 8);

    for i in 0..input_data.len() / 8 {
        let mut byte = 0;
        for j in 0..8 {
            let digit = input_data[i * 8 + j];
            byte |= digit << j;
        }
        output_data.push(byte as u8);
    }

    output_data
}


fn main() {
    let input_data = "hi he".as_bytes().to_vec();
    let output_data = compress(&input_data);

    println!("input: {:x?}", input_data);
    println!("compressed: {:x?}", output_data);
    
    let decompressed_data = decompress(output_data);

    println!("decompressed: {:x?}", decompressed_data);
}
