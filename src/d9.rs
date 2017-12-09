#[allow(dead_code)]
pub fn problem1(data: &str) {
    let mut score: i32 = 0;
    let mut depth: i32 = 0;
    let mut ignore_next = false;
    let mut in_garbage = false;

    for s in data.chars(){
        if ignore_next {
            ignore_next = false;
            continue;
        }
        match s {
            '{' => {
                if !in_garbage {
                    depth += 1;
                }
            },
            '}' => {
                if !in_garbage {
                    score += depth;
                    depth -= 1;
                }
            },
            '<' => {
                in_garbage = true;
            },
            '>' => {
                in_garbage = false;
            },
            '!' => {
                ignore_next = true;
            },
            ',' => {},
            _ => {
                if !in_garbage {
                    panic!("Unregcognized symbol: {}", s) 
                }
            }
        }
    }

    println!("Score: {}", score);
}

#[allow(dead_code)]
pub fn problem2(data: &str) {
    let mut garbage: i32 = 0;
    let mut ignore_next = false;
    let mut in_garbage = false;

    for s in data.chars(){
        if ignore_next {
            ignore_next = false;
            continue;
        }
        match s {
            '<' => {
                if in_garbage {
                    garbage += 1;
                }
                in_garbage = true;
            },
            '>' => {
                in_garbage = false;
            },
            '!' => {
                ignore_next = true;
            },
            _ => {
                if in_garbage {
                    garbage += 1;
                }
            }
        }
    }

    println!("Garbage count: {}", garbage);
}