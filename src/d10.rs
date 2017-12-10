extern crate hex;

#[allow(dead_code)]
pub fn problem1(data: &str) {
    let mut numbers: Vec<i32> = Vec::new();
    let mut current_position = 0;
    let mut skip_size = 0;
    let v: Vec<&str> = data.split(',').collect();

    for i in 0..256 {
        numbers.push(i);
    }

    let numbers_len = numbers.len();    

    for s in v {
        let length = s.parse::<usize>().unwrap();
        let mut swap_count = 0;
        let mut s = current_position;
        let mut e = current_position + length - 1;
        
        loop {
            swap_count += 1;
            numbers.swap(s, e % numbers_len);
            s = if s + 1 < numbers_len { s + 1 } else { 0 };
            e = if e > 0 { e - 1 } else { numbers_len - 1 };
            if swap_count >= length / 2 {
                break;
            }
        }
        current_position = (current_position + length + skip_size) % numbers_len;
        skip_size += 1;
    }

    println!("{}", numbers[0] * numbers[1]);
}

#[allow(dead_code)]
pub fn problem2(data: &str) {
    let mut numbers: Vec<u8> = Vec::new();
    let mut current_position = 0;
    let mut skip_size = 0;
    let mut lengths = data.as_bytes().to_vec();

    lengths.push(17);
    lengths.push(31);
    lengths.push(73);
    lengths.push(47);
    lengths.push(23);

    for i in 0..256 {
        numbers.push(i as u8);
    }

    let numbers_len = numbers.len();
    for _ in 0..64 {
        for b in &lengths {
            let length = *b as usize;
            let mut swap_count = 0;
            let mut s = current_position;
            let mut e = current_position + length - 1;
            
            loop {
                swap_count += 1;
                numbers.swap(s, e % numbers_len);
                s = if s + 1 < numbers_len { s + 1 } else { 0 };
                e = if e > 0 { e - 1 } else { numbers_len - 1 };
                if swap_count >= length / 2 {
                    break;
                }
            }
            current_position = (current_position + length + skip_size) % numbers_len;
            skip_size += 1;
        }
    }

    let mut result: Vec<u8> = Vec::new();
    let mut start = 0;
    loop {
        result.push(xor(numbers.as_slice(), start, 16));
        start += 16;
        if start == 256 {
            break;
        }
    }

    let hex_string = hex::encode(result);
    println!("{}", hex_string);
}

fn xor(arr: &[u8], start: usize, length: usize) -> u8 {
    let mut val: u8 = 0;
    let end = start + length;
    for i in start..end {
        val ^= arr[i];
    }
    val
} 