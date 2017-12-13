pub enum Direction {
   NW,
   N,
   NE,
   SE,
   S,
   SW,
}

#[derive(Debug)]
struct Point {
   q: i32,
   r: i32,
   s: i32,
}

impl Point {
   pub fn new(q: i32, r: i32, s: i32) -> Point {
       assert!(q + r + s == 0, "q = {}, r = {}, s = {}", q, r, s);
       Point { q: q, r: r, s: s }
   }

   pub fn add(&self, other: &Point) -> Point {
       Point { q: self.q + other.q, r: self.r + other.r, s: self.s + other.s }
   }

   pub fn sub(&self, other: &Point) -> Point {
       Point { q: self.q - other.q, r: self.r - other.r, s: self.s - other.s }
   }

   pub fn length(&self) -> i32 {
       (self.q.abs() + self.r.abs() + self.s.abs()) / 2
   }

   pub fn distance(&self, other: &Point) -> i32 {
       self.sub(other).length()
   }

   pub fn neighbor(&self, direction: Direction) -> Point {
       let step = match direction {
           Direction::NW => Point { q: 1, r: 0, s: -1 },
           Direction::N => Point { q: 1, r: -1, s: 0 },
           Direction::NE => Point { q: 0, r: -1, s: 1 },
           Direction::SE => Point { q: -1, r: 0, s: 1 },
           Direction::S => Point { q: -1, r: 1, s: 0 },
           Direction::SW => Point { q: 0, r: 1, s: -1 },
       };
       
       self.add(&step)
   }
}

#[allow(dead_code)]
pub fn problem1(data: &str) {
   let directions: Vec<&str> = data.split(',').collect();
   let mut current = Point { q: 0, r: 0, s: 0 };

   for d in directions {
       let dir = match d {
           "nw" => Direction::NW,
           "n" => Direction::N,
           "ne" => Direction::NE,
           "se" => Direction::SE,
           "s" => Direction::S,
           "sw" => Direction::SW,
           _ => panic!("Unregcognized symbol: {}", d),
       };

       current = current.neighbor(dir);
   }

   println!("Distance {}", current.distance(&Point { q: 0, r: 0, s: 0 }));
}

#[allow(dead_code)]
pub fn problem2(data: &str) {
   let v: Vec<&str> = data.split(',').collect();
   let directions: Vec<&str> = data.split(',').collect();
   let origin = Point { q: 0, r: 0, s: 0 };
   let mut current = Point { q: 0, r: 0, s: 0 };
   let mut max_distance = 0;

   for d in directions {
       let dir = match d {
           "nw" => Direction::NW,
           "n" => Direction::N,
           "ne" => Direction::NE,
           "se" => Direction::SE,
           "s" => Direction::S,
           "sw" => Direction::SW,
           _ => panic!("Unregcognized symbol: {}", d),
       };

       current = current.neighbor(dir);

       let distance = current.distance(&origin);
       if distance > max_distance {
           max_distance = distance;
       }
   }

   println!("Max Distance {}", max_distance);
}