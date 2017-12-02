pub mod d2
{
    use std;

    #[allow(dead_code)]
    pub fn d2_problem1(data: &str) {
        let mut sum: i32 = 0;

        for s in data.lines(){
            let mut low_value = std::i32::MAX;
            let mut high_value = std::i32::MIN;

            let elements = s.split_whitespace();

            for e in elements {
                let current = e.parse::<i32>().unwrap();

                high_value = std::cmp::max(high_value, current);
                low_value = std::cmp::min(low_value, current);
            }

            sum += high_value - low_value;
        }

        println!("Result = {}", sum);
    }

    #[allow(dead_code)]
    pub fn d2_problem2(data: &str) {
        let mut sum: i32 = 0;

        for s in data.lines(){
            let mut a = 0;
            let mut b = 0;

            let elements: Vec<i32> = s.split_whitespace().map(|s| s.parse().unwrap()).collect();

            for i in 0..elements.len() {
                let mut found = false;
                let mut j = i + 1;
                for j in i+1..elements.len() {
                    let current = elements[j];
                    let high_value = std::cmp::max(elements[i], current);
                    let low_value = std::cmp::min(elements[i], current);

                    if high_value % low_value == 0 {
                        sum += high_value / low_value;
                        found = true;
                        break;
                    }
                }

                if found {
                    break;
                }
            }
        }

        println!("Result = {}", sum);
    }
}