extern crate unicode_segmentation;
use unicode_segmentation::UnicodeSegmentation;

fn main() {
    fn find_matches(pattern_as_str: &str, text_as_str: &str) -> Vec<u32> {
        fn gen_hash(pattern: &[&str], prime: u32) -> u32 {
            let mut hash = 0;
            for c in pattern.iter() {
                let char_val = c.chars().next().unwrap() as u32;
                hash = ((hash * 10) + char_val) % prime;
            }
            hash
        }

        fn roll_hash(
            first_str: &str,
            pattern_len: usize,
            prev_hash_passed: u32,
            next_str: &str,
            prime: u32,
        ) -> u32 {
            let prev_hash = prev_hash_passed + prime;

            let mut multiplier = 1;
            let mut i = 1;
            while i < pattern_len {
                multiplier *= 10;
                multiplier %= prime;
                i += 1;
            }

            let first_char = first_str.chars().next().unwrap() as u32;
            let next_char = next_str.chars().next().unwrap() as u32;

            let hash_char_removed = (prev_hash - ((multiplier * first_char) % prime)) * 10;
            (hash_char_removed + next_char) % prime
        }

        // a prime to reduce the size of the values
        let prime = 461;

        let pattern_as_graphemes =
            UnicodeSegmentation::graphemes(pattern_as_str, true).collect::<Vec<&str>>();
        let pattern_hash = gen_hash(&pattern_as_graphemes[..], prime);

        // the indices of the pattern
        let mut start = 0;
        let mut end = pattern_as_graphemes.len() - 1;

        // the previous hash
        let mut prev_hash: Option<u32> = None;

        let mut matches_found: Vec<u32> = vec![];

        let text_as_graphemes =
            UnicodeSegmentation::graphemes(text_as_str, true).collect::<Vec<&str>>();

        while end <= (text_as_graphemes.len() - 1) {
            let sub_string_as_graphemes = &text_as_graphemes[start..end + 1];
            let sub_string_hash: u32;
            if let Some(previous_hash) = prev_hash {
                let grapheme_being_dropped = text_as_graphemes[(start - 1)..start][0];
                let added_grapheme = text_as_graphemes[(end)..][0];
                sub_string_hash = roll_hash(
                    grapheme_being_dropped,
                    pattern_as_graphemes.len(),
                    previous_hash,
                    added_grapheme,
                    prime,
                );
                prev_hash = Some(sub_string_hash);
            } else {
                sub_string_hash = gen_hash(&text_as_graphemes[start..end + 1], prime);
                prev_hash = Some(sub_string_hash);
            }
            if sub_string_hash == pattern_hash && pattern_as_graphemes == sub_string_as_graphemes {
                matches_found.push(start as u32);
            }
            start += 1;
            end += 1;
        }
        matches_found
    }

    let matches = find_matches("ddd", "dddddddda̐éö̲bbddddd");
    println!("the matches: {:?}", matches)
}
//
// USING POLYNOMIAL
// fn gen_hash(pattern: &str, prime: u32) -> u32 {
//     let mut hash = 0;
//     let mut pow_val = (pattern.len() as u32) - 1;
//     for c in pattern.chars() {
//         // hash = ((hash * ran_num) + (c as u32)) % prime;
//         hash += (c as u32) * (10_u32.checked_pow(pow_val as u32).unwrap());
//         pow_val -= 1;
//     }
//     hash
// }
