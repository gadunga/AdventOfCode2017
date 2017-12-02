pub mod d1
{
    #[allow(dead_code)]
    pub fn day1_problem1(data: &str) {
        let mut sum: u32 = 0;
        
        for (i, c) in data.chars().enumerate() {
            let curr: u32 = c.to_digit(10).unwrap();
            let next: u32 = data.chars().nth((i + 1) % data.len()).unwrap().to_digit(10).unwrap();

            if curr == next {
                sum += curr;
            }
        }

        println!("Result = {}", sum);
    }

    #[allow(dead_code)]
    pub fn day1_problem2(data: &str) {
        let mut sum: u32 = 0;
        let step = data.len() / 2;
        let len = data.len();
        
        for (i, c) in data.chars().enumerate() {
            let curr: u32 = c.to_digit(10).unwrap();
            let next: u32 = data.chars().nth((i + step) % len).unwrap().to_digit(10).unwrap();

            if curr == next {
                sum += curr;
            }
        }

        println!("Result = {}", sum);
    }
}