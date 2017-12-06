pub mod d5
{
    #[allow(dead_code)]
    pub fn day5_problem1(data: &str) {
        let mut steps: i32 = 0;
        let mut offsets: Vec<i32> = Vec::new();

        for s in data.lines(){
            offsets.push(s.parse::<i32>().unwrap());
        }

        let mut index: i32 = 0;
        loop {
            steps += 1;
            let current_offset = offsets[index as usize];
            let new_offset = current_offset + 1;

            if current_offset + index >= offsets.len() as i32 {
                break;
            }
            offsets[index as usize] = new_offset;
            index += current_offset;
        }

        println!("Result = {}", steps);
    }

    #[allow(dead_code)]
    pub fn day5_problem2(data: &str) {
        let mut steps: i32 = 0;
        let mut offsets: Vec<i32> = Vec::new();

        for s in data.lines(){
            offsets.push(s.parse::<i32>().unwrap());
        }

        let mut index: i32 = 0;
        loop {
            steps += 1;
            let current_offset = offsets[index as usize];
            let mut new_offset = current_offset;

            if new_offset >= 3 {
                new_offset -= 1;
            } else {
                new_offset += 1;
            }

            if current_offset + index >= offsets.len() as i32 {
                break;
            }
            offsets[index as usize] = new_offset;
            index += current_offset;
        }

        println!("Result = {}", steps);
    }
}