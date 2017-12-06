pub mod d6
{
    use std::collections::HashSet;

    #[allow(dead_code)]
    pub fn day6_problem1(data: &str) {
        let mut banks: Vec<i32> = Vec::new();

        let elements = data.split_whitespace();
        for e in elements {
            banks.push(e.parse::<i32>().unwrap());
        }

        let mut sequences = HashSet::new();
        loop {
            let current_sequence = generate_sequence(&banks);
            if sequences.contains(&current_sequence) {
                break;
            }

            sequences.insert(current_sequence);

            let mut current_block = get_candidate_index(&banks);
            let num_blocks = banks[current_block];

            banks[current_block] = 0;
            
            for _i in 0..num_blocks {
                current_block += 1;
                let len = banks.len();
                let index = current_block % len;
                let value = banks[index] + 1;

                banks[current_block % len] = value;
            }
        }

        println!("{:?}", banks);
        println!("{}", sequences.len());
    }

    fn generate_sequence(banks: &Vec<i32>) -> String {
        let mut sequence: String = "".to_owned();
        for i in 0..banks.len() {
            sequence.push_str(banks[i].to_string().as_str());
            if i != banks.len() - 1 {
                sequence.push_str("_");
            }
        }

        return sequence;
    }

    fn get_candidate_index(banks: &Vec<i32>) -> usize {
        let mut block_count: i32 = 0;
        let mut index = 0;

        for i in 0..banks.len() {
            if banks[i] > block_count {
                block_count = banks[i];
                index = i;
            }
        }

        return index;
    }

    #[allow(dead_code)]
    pub fn day6_problem2(data: &str) {
        let mut banks: Vec<i32> = Vec::new();

        let elements = data.split_whitespace();
        for e in elements {
            banks.push(e.parse::<i32>().unwrap());
        }

        let mut sequences = HashSet::new();
        let mut repeat_found = false;

        loop {
            let current_sequence = generate_sequence(&banks);
            if sequences.contains(&current_sequence) {
                if repeat_found {
                    break;
                }
                sequences.clear();
                repeat_found = true;
            }

            sequences.insert(current_sequence);

            let mut current_block = get_candidate_index(&banks);
            let num_blocks = banks[current_block];

            banks[current_block] = 0;
            
            for _i in 0..num_blocks {
                current_block += 1;
                let len = banks.len();
                let index = current_block % len;
                let value = banks[index] + 1;

                banks[current_block % len] = value;
            }
        }

        println!("{:?}", banks);
        println!("{}", sequences.len());
    }
}