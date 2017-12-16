/*
Generator A starts with 699
Generator B starts with 124
Factors:
generator A uses 16807
generator B uses 48271
 */

#[allow(dead_code)]
pub fn problem1() {
    let mut a = 699;
    let mut b = 124;
    let mut count = 0;
    for _ in 0..40000000 {
        a = generate(a, 16807);
        b = generate(b, 48271);

        if a & 0x0FFFF == b & 0xFFFF {
            count += 1;
        }
    }

    println!("Match count = {}", count);
}

pub fn problem2() { 
    let mut a = 699;
    let mut b = 124;
    let mut count = 0;
    
    let mut a_values: Vec<u64> = Vec::new();
    let mut b_values: Vec<u64> = Vec::new();

    loop {
        if a_values.len() == 5000000 && a_values.len() == b_values.len() {
            break;
        }

        if a_values.len() != 5000000 {
            a = generate(a, 16807);
            if a % 4 == 0 {
                a_values.push(a);
            }
        }
        
        if b_values.len() != 5000000 {
            b = generate(b, 48271);
            if b % 8 == 0 {
                b_values.push(b);
            }
        }        
    }

    for _ in 0..a_values.len() {
        let temp_a = a_values.pop().unwrap();
        let temp_b = b_values.pop().unwrap();

        if temp_a & 0x0FFFF == temp_b & 0xFFFF {
            count += 1;
        }
    }

    println!("Match count = {}", count);
}

fn generate(input: u64, factor: u64) -> u64 {
    let result = input * factor;
    result % 2147483647
}