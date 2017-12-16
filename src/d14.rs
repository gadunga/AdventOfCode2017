use std::collections::VecDeque;

struct Point {
    x: i32,
    y: i32,
}

#[allow(dead_code)]
pub fn problem1() {
    let key = "hfdlxzhv";
    let mut sum = 0;
    
    for i in 0..128 {
        sum += calc_used_blocks(key.to_string(), i);
    }

    println!("{}", sum);
}

pub fn problem2() {
    let mut memory: [[u8; 128]; 128] = [[0; 128]; 128];
    let key = "hfdlxzhv";

    for i in 0..128 {
        let row_data = get_row_data(key.to_string(), i);
        for byte in 0..row_data.len() {
            memory[i as usize][byte * 8] = if (1 << 7) & row_data[byte] != 0 { 1 } else { 0 };
            memory[i as usize][byte * 8 + 1] = if (1 << 6) & row_data[byte] != 0 { 1 } else { 0 };
            memory[i as usize][byte * 8 + 2] = if (1 << 5) & row_data[byte] != 0 { 1 } else { 0 };
            memory[i as usize][byte * 8 + 3] = if (1 << 4) & row_data[byte] != 0 { 1 } else { 0 };
            memory[i as usize][byte * 8 + 4] = if (1 << 3) & row_data[byte] != 0 { 1 } else { 0 };
            memory[i as usize][byte * 8 + 5] = if (1 << 2) & row_data[byte] != 0 { 1 } else { 0 };
            memory[i as usize][byte * 8 + 6] = if (1 << 1) & row_data[byte] != 0 { 1 } else { 0 };
            memory[i as usize][byte * 8 + 7] = if (1 << 0) & row_data[byte] != 0 { 1 } else { 0 };
        }
    }

    let mut region_count = 0;
    let mut queue: VecDeque<Point> = VecDeque::new();

    for x in 0..128 {
        for y in 0..128 {
            let value = memory[x as usize][y as usize];
            if value != 1 {
                continue;
            }

            // Value is 1
            region_count += 1;
            queue.clear();
            queue.push_back(Point { x: x, y: y });

            loop {
                if queue.is_empty() {
                    break;
                }

                let current_point = queue.pop_front().unwrap();
                if current_point.x < 0 || 
                    current_point.x > 127 || 
                    current_point.y < 0 || 
                    current_point.y > 127 || 
                    memory[current_point.x as usize][current_point.y as usize] != 1 {
                    continue;
                }

                memory[current_point.x as usize][current_point.y as usize] = 2;

                queue.push_back(Point { x: current_point.x - 1, y: current_point.y });
                queue.push_back(Point { x: current_point.x + 1, y: current_point.y });
                queue.push_back(Point { x: current_point.x, y: current_point.y - 1 });
                queue.push_back(Point { x: current_point.x, y: current_point.y + 1 });
            }
        }
    }

    
    println!("{}", region_count);
}

fn get_row_data(key: String, row: u32) -> Vec<u8> {
    let hash_input = key + "-" + row.to_string().as_str();
    let mut lengths = hash_input.as_bytes().to_vec();
    lengths.push(17);
    lengths.push(31);
    lengths.push(73);
    lengths.push(47);
    lengths.push(23);

    hash(lengths)
}

fn calc_used_blocks(key: String, row: u32) -> usize {
    let row_data = get_row_data(key, row);
    let mut sum = 0;

    for n in row_data {
        sum += count_bits(n);
    }

    sum
}

fn count_bits(input: u8) -> usize {
    let mut result = 0;
    let mut n = input;
    loop {
        if n == 0 {
            break;
        }

        n &= n - 1;
        result += 1;
    }

    result
}

fn hash(bytes: Vec<u8>) -> Vec<u8> {
    let mut numbers: Vec<u8> = Vec::new();
    let mut current_position = 0;
    let mut skip_size = 0;

    for i in 0..256 {
        numbers.push(i as u8);
    }

    let numbers_len = numbers.len();
    for _ in 0..64 {
        for b in &bytes {
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

    result
}

fn xor(arr: &[u8], start: usize, length: usize) -> u8 {
    let mut val: u8 = 0;
    let end = start + length;
    for i in start..end {
        val ^= arr[i];
    }
    val
} 