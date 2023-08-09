fn main() {
    // Might as well use a file we know exists
    let out = find_most_common_bit_for_each(&std::fs::read("./src/main.rs").unwrap());
    let out_inverted = out ^ u8::MAX;
    println!("Most common bits:\n{out:08b}\nInverted:\n{out_inverted:08b}");
}

// Given a list of bytes, find the most common bit for each of the 8 bits
// Example:
// Input:
// 11110000
// 10101010
// 00110011
// Output:
// 10110010
fn find_most_common_bit_for_each(bytes: &[u8]) -> u8 {
    // Positive counts mean there were that many more 1s than 0s
    // Negative counts mean there abs(that) many more 0s than 1s
    // 0 mean there were an equal number of 1s and 0s
    let mut counts = [0i64; 8];
    for byte in bytes {
        let mut byte = *byte;
        for index in (0..8).rev() {
            let least_significant_bit = byte % 2 == 1;
            counts[index] += if least_significant_bit { 1 } else { -1 };
            byte >>= 1;
        }
    }
    // If there's an equal number of 0s and 1s, we'll just say the most common bit is 0
    let bits = counts.map(|count| count > 0);
    let mut out = 0;
    for (index, bit) in bits.into_iter().enumerate() {
        let bit = bit as u8;
        let shifted_bit = bit << (7 - index);
        out |= shifted_bit;
    }
    out
}

#[cfg(test)]
mod tests {

    #[test]
    fn bool_cast_test() {
        assert_eq!(false as u8, 0);
        assert_eq!(true as u8, 1);
        assert_eq!(false as i64, 0);
        assert_eq!(true as i64, 1);
    }

    #[test]
    fn clever_test_name() {
        assert_eq!(
            super::find_most_common_bit_for_each(&[0b11110000, 0b10101010, 0b00110011]),
            0b10110010
        )
    }
}
