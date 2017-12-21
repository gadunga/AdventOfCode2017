#[allow(dead_code)]
pub fn problem1(data: &str) {
    let steps: usize = data.parse().unwrap();
    let mut current_position = 0;
    let mut values: Vec<i32> = Vec::new();
    values.push(0);

    for i in 1..2018 {
        let insert_at = ((current_position + steps) % values.len()) + 1;
        if insert_at == values.len() {
            values.push(i);
        } else {
            values.insert(insert_at, i);
        }

        current_position = insert_at;
    }

    println!("{}", values[current_position + 1]);
}

pub fn problem2(data: &str) {
    let steps: usize = data.parse().unwrap();
    let mut val = 0;
    let mut current_position: usize = 0;

    for i in 1..50000001 {
        current_position = (current_position + steps) % i;

        if current_position == 0 {
            val = i;
        }

        current_position += 1;
    }
    println!("{}", val);
}