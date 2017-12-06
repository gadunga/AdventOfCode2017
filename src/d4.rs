pub mod d4
{
    use std::collections::HashSet;
    use std::collections::HashMap;

    #[allow(dead_code)]
    pub fn day4_problem1(data: &str) {
        let mut valid_count = 0;

        for s in data.lines(){
            let mut phrases = HashSet::new();
            let elements = s.split_whitespace();
            let mut invalid = false;

            for e in elements {
                if phrases.contains(e) {
                    invalid = true;
                    break;
                }

                phrases.insert(e);
            }

            if invalid {
                continue;
            }

            valid_count += 1;
        }

        println!("Valid passphrases {}", valid_count);
    }

    #[allow(dead_code)]
    pub fn day4_problem2(data: &str) {
        let mut valid_count = 0;

        for s in data.lines(){
            let mut phrases = HashMap::new();
            let elements = s.split_whitespace();
            let mut invalid = false;
            let parts: Vec<&str> = elements.collect();

            for i in 0..parts.len() {
                let mut characters = HashMap::new();

                for c in parts[i].chars() {
                    let count = characters.entry(c).or_insert(0);
                    *count += 1;
                }

                phrases.insert(parts[i], characters);
            }

            for i in 0..parts.len() {
                let mut anagram_found = false;

                for j in i+1..parts.len() {
                    if parts[j].len() != parts[i].len() {
                        continue;
                    }

                    let compare_phrase = &phrases[parts[j]];
                    let mut is_match = true;

                    for (key, value) in &phrases[parts[i]] {
                        if !compare_phrase.contains_key(&key) || *(compare_phrase.get(&key).unwrap()) != *value {
                            is_match = false;
                            break;
                        }
                    }

                    if is_match {
                        anagram_found = true;
                        break;
                    }
                } 

                if anagram_found {
                    invalid = true;
                    break;
                }
            }

            if invalid {
                continue;
            }

            valid_count += 1;
        }

        println!("Valid passphrases {}", valid_count);
    }
}