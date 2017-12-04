pub mod d3
{
    /*For example, if x=(a,b)x=(a,b) and y=(c,d)y=(c,d), the Manhattan distance between xx and yy is

|a−c|+|b−d||a−c|+|b−d|. */
    #[derive(Debug)]
    struct Point {
        x: i32,
        y: i32,
    }

    #[allow(dead_code)]
    pub fn day3_problem1(data: i32) {
        let mut level = 0;
        let mut n: i32;
        let mut corners: [i32; 4]; // BR, BL, TL, TR
        loop {
            n = 2 * level + 1;
            corners = [
                n * n - 1,
                n * n - n,
                n * n - 2 * n + 1,
                n * n - 3 * n + 2,
             ];

            if corners[0] > data {
                break;
            }

            level += 1;
        }

        let loc: Point;
        if data <= corners[3] {
            loc = Point { x: n, y: n - (corners[3] - data) };
        } else if data > corners[3] && data <= corners[2] {
            loc = Point { x: n - (corners[2] - data), y: n };
        } else if data > corners[2] && data <= corners[1] {
            loc = Point { x: 0, y: n - (corners[1] - data) }
        } else {
            loc = Point { x: n - (corners[0] - data), y: 0 };
        }

        let center = Point { x: (n - 1) / 2, y: (n - 1) / 2 };

        let result = (loc.x - center.x).abs() + (loc.y - center.y).abs() - 1;

        println!("{:?}", result);
    }

    #[derive(Debug)]
    struct P2Point {
        index: i32,
        coord: Point,
        value: i32,
    }

    #[allow(dead_code)]
    pub fn day3_problem2(data: i32) {
        let mut current_level: Vec<P2Point> = Vec::new();
        let mut prev_level: Vec<P2Point> = Vec::new();
        prev_level.push(P2Point { index: 0, coord: Point {x: 0, y: 0}, value: 1});

        let mut level: i32 = 0;
        let mut n;
        let mut old_n: i32 = 0;
        let mut corners: [i32; 4]; // BR, BL, TL, TR
        let mut loc: Point;
        
        loop {
            n = 2 * level + 1;
            let start = old_n * old_n;
            let end = n * n - 1;

            corners = [
                n * n - 1,
                n * n - n,
                n * n - 2 * n + 1,
                n * n - 3 * n + 2,
            ];

            for i in start..end+1 {
                if i <= corners[3] {
                    loc = Point { x: level, y: level - (corners[3] - i) };
                } else if i > corners[3] && i <= corners[2] {
                    loc = Point { x: (corners[2] - i) - level, y: level };
                } else if i > corners[2] && i <= corners[1] {
                    loc = Point { x: level * -1, y: (corners[1] - i) - level }
                } else {
                    loc = Point { x: level - (corners[0] - i), y: level * -1 };
                }

                let mut new_point = P2Point { index: i, coord: loc, value: 0};
                new_point.value = sum(i, &new_point.coord, &current_level, &prev_level);

                if new_point.value >= data {
                    println!("{:?}", new_point);
                    return;
                }

                current_level.push(new_point);
            }
            
            level += 1;
            old_n = n;
            prev_level = current_level;
            current_level = Vec::new();
        }
    }

    fn sum(curr_index: i32, loc: & Point, current_level: & Vec<P2Point>, prev_level: & Vec<P2Point>) -> i32 {
        if loc.x == 0 && loc.y == 0 {
            return 1;
        }
        
        let mut sum: i32 = 0;

        for c in current_level {
            if c.index == curr_index - 1  || 
            ((c.coord.x - loc.x).abs() <= 1 && (c.coord.y - loc.y).abs() <= 1){
                sum += c.value;
            }
        }

        for c in prev_level { 
            if (c.coord.x - loc.x).abs() <= 1 && (c.coord.y - loc.y).abs() <= 1{
                sum += c.value;
            }
        }

        return sum;
    }
}