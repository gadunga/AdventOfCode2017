use std::collections::HashMap;

#[allow(dead_code)]
pub fn problem1(data: &str) {
    let mut scanner_ranges: HashMap<i32, i32> = HashMap::new();
    let mut max_depth = 0;

    for s in data.lines() {
        let mut elements = s.split_whitespace();
        let layer = elements.next()
                            .unwrap()
                            .trim_matches(|c| c == ':')
                            .parse::<i32>()
                            .unwrap();
        let range = elements.next()
                            .unwrap()
                            .parse::<i32>()
                            .unwrap();
        
        scanner_ranges.insert(layer, range);

        if layer > max_depth {
            max_depth = layer;
        }
    }

    let mut sum = 0;
    for i in 0..max_depth + 1 {       
        let range = if scanner_ranges.contains_key(&i) { scanner_ranges[&i] } else { 0 };
        if is_detected(range, i) {
            sum += (i as i32) * range;
        }
    }

    println!("{}", sum);
}

#[allow(dead_code)]
pub fn problem2(data: &str) {
    let mut scanner_ranges: HashMap<i32, i32> = HashMap::new();
    let mut max_depth = 0;

    for s in data.lines() {
        let mut elements = s.split_whitespace();
        let layer = elements.next()
                            .unwrap()
                            .trim_matches(|c| c == ':')
                            .parse::<i32>()
                            .unwrap();
        let range = elements.next()
                            .unwrap()
                            .parse::<i32>()
                            .unwrap();
        
        scanner_ranges.insert(layer, range);

        if layer > max_depth {
            max_depth = layer;
        }
    }

    let mut delay = 0;

    loop {
        let mut detected = false;
        for i in 0..max_depth + 1 {
            let range = if scanner_ranges.contains_key(&i) { scanner_ranges[&i] } else { 0 };
            if is_detected(range, i + delay) {
                detected = true;
                break;
            }
        }

        if detected == false {
            println!("{}", delay);
            return;
        }

        delay += 1;
    }
}

fn is_detected(range: i32, time: i32) -> bool {
    match range {
        0 => false,
        n => {
            if time % ((n - 1) * 2) == 0 {
                return true;
            }
            false
        }
    }
}