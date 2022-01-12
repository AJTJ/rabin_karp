fn main() {
    // let text = "abghijkcdef";

    fn gen_hash(pattern: &str, prime: i32) -> i32 {
        let mut hash = 0;
        let mut pow_val = (pattern.len() as i32) - 1;
        for c in pattern.chars() {
            // hash = ((hash * ran_num) + (c as i32)) % prime;
            hash = (hash + (c as i32 - 96) * (10_i32.checked_pow(pow_val as u32).unwrap())) % prime;
            pow_val -= 1;
        }
        hash
    }

    fn roll_hash(
        first_char: char,
        pattern: &str,
        prev_hash: i32,
        next_char: char,
        prime: i32,
    ) -> i32 {
        println!("prev hash: {}", prev_hash);
        let hash_char_removed = prev_hash
            - (((first_char as i32 - 96)
                * (10_i32.checked_pow((pattern.len() - 1) as u32)).unwrap())
                % prime);
        println!("hash with char removed: {}", hash_char_removed);
        let new_hash = ((hash_char_removed * 10) + (next_char as i32 - 96)) % prime;
        println!("new hash: {}", new_hash);
        new_hash
    }

    // a prime to reduce the size of the values
    let prime = 461;

    let pattern = "ddd";
    // the hash of the pattern to find
    let pattern_hash = gen_hash(pattern, prime);

    let hash_of_two = gen_hash("dd", prime);
    println!("hash of two: {}", hash_of_two);

    // TESTING
    roll_hash(
        pattern.chars().next().unwrap(),
        pattern,
        pattern_hash,
        'd',
        prime,
    );

    // // the indices of the pattern
    // let mut start = 0;
    // let mut end = pattern.len();

    // // previous pattern hash
    // let mut prev_hash: Option<i32> = None;

    // // hash of dropped char
    // let mut first_char_hash: Option<i32> = None;

    // while end <= text.len() {
    //     let sub_string = &text[start..end];
    //     let sub_string_hash = gen_hash(sub_string, prime);

    //     if sub_string_hash == pattern_hash && pattern == sub_string {
    //         return println!("pattern starts at: {}", start);
    //     }
    //     start += 1;
    //     end += 1;
    // }
}

// println!("sub hash: {}, sub_string: {}", sub_string_hash, sub_string);

// fn gen_hash(str: &str, exp: i32) -> i32 {
//     let mut exp_val = (str.len() - 1) as i32;
//     let mut total = 0;
//     for c in str.chars() {
//         let hash_val = (c as i32) ^ exp_val;
//         println!("{}, {}, {}", exp_val, total, hash_val);
//         total = total + hash_val;
//         exp_val = exp_val - 1;
//     }

//     total
// }

// println!(
//     "hash: {}, hash * ran_num: {}, c as i32: {}, prime: {}",
//     hash,
//     (hash * ran_num),
//     c as i32,
//     prime
// );

// println!("new hash {}", hash);

// USING POLYNOMIAL
// fn gen_hash(pattern: &str, prime: i32) -> i32 {
//     let mut hash = 0;
//     let mut pow_val = (pattern.len() as i32) - 1;
//     for c in pattern.chars() {
//         // hash = ((hash * ran_num) + (c as i32)) % prime;
//         hash += (c as i32 - 96) * (10_i32.checked_pow(pow_val as u32).unwrap());
//         pow_val -= 1;
//     }
//     hash
// }
