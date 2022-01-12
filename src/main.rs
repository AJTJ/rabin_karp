fn main() {
    fn find_matches(pattern: &str, text: &str) -> Vec<u32> {
        fn gen_hash(pattern: &str, prime: u32) -> u32 {
            let mut hash = 0;
            // let mut pow_val = (pattern.len() as u32) - 1;
            for c in pattern.chars() {
                // This uses polynomial hashing
                // hash = (hash + (c as u32 - 96) * (10_u32.checked_pow(pow_val as u32).unwrap())) % prime;
                // using Horner's method, we can simplify and reduce the chance of overflow
                hash = (hash * 10 + c as u32 - 96) % prime;
                // pow_val -= 1;
            }
            hash
        }

        fn roll_hash(
            first_char: char,
            pattern: &str,
            prev_hash_passed: u32,
            next_char: char,
            prime: u32,
        ) -> u32 {
            println!("prev hash: {}", prev_hash_passed);
            // add the modulus to ensure non-negative numbers
            let prev_hash = prev_hash_passed + prime;
            let hash_char_removed = prev_hash
                - (((first_char as u32 - 96)
                    * (10_u32.checked_pow((pattern.len() - 1) as u32)).unwrap())
                    % prime);
            println!("hash with char removed + prime: {}", hash_char_removed);
            let new_hash = ((hash_char_removed * 10) + (next_char as u32 - 96)) % prime;
            println!("new hash: {}", new_hash);
            new_hash
        }

        // a prime to reduce the size of the values
        let prime = 461;

        // the hash of the pattern to find
        let pattern_hash = gen_hash(pattern, prime);

        // the indices of the pattern
        let mut start = 0;
        let mut end = pattern.len();

        // the previous hash
        let prev_hash: Option<u32> = None;

        let mut matches_found: Vec<u32> = vec![];

        while end <= text.len() {
            let sub_string = &text[start..end];
            let sub_string_hash: u32;
            if let Some(prev) = prev_hash {
                sub_string_hash = roll_hash(
                    text[(start - 1)..start].chars().next().unwrap(),
                    pattern,
                    prev,
                    text[end..(end + 1)].chars().next().unwrap(),
                    prime,
                )
            } else {
                sub_string_hash = gen_hash(sub_string, prime);
            }

            if sub_string_hash == pattern_hash && pattern == sub_string {
                matches_found.push(start as u32);
            }
            start += 1;
            end += 1;
        }
        matches_found
    }

    let matches = find_matches("ddd", "aaaddddbbddddd");
    println!("the matches: {:?}", matches)
}

// USING POLYNOMIAL
// fn gen_hash(pattern: &str, prime: u32) -> u32 {
//     let mut hash = 0;
//     let mut pow_val = (pattern.len() as u32) - 1;
//     for c in pattern.chars() {
//         // hash = ((hash * ran_num) + (c as u32)) % prime;
//         hash += (c as u32 - 96) * (10_u32.checked_pow(pow_val as u32).unwrap());
//         pow_val -= 1;
//     }
//     hash
// }
