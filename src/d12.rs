use std::collections::HashMap;
use std::collections::VecDeque;

#[derive(Debug, Clone)]
struct Program {
   id: i32,
   pipes: Vec<i32>,
}

#[allow(dead_code)]
pub fn problem1(data: &str) {
   let mut programs = HashMap::new();

   for s in data.lines(){
       let mut elements = s.split_whitespace();
       let id = elements.next().unwrap().parse::<i32>().unwrap();
       let mut connections: Vec<i32> = Vec::new();

       for s in elements {
           if s ==  "<->" {
               continue;
           }
           connections.push(s.trim_matches(|c| c == ',').parse::<i32>().unwrap());
       }

       programs.insert(id, Program { id: id, pipes: connections });
   }

   let zero = programs.get(&0).unwrap();
   let members = process_group(&zero, &programs);
   println!("Count {}", members.len() + 1);
}

#[allow(dead_code)]
pub fn problem2(data: &str) {
   let mut programs = HashMap::new();
   let mut ids = Vec::new();

   for s in data.lines(){
       let mut elements = s.split_whitespace();
       let id = elements.next().unwrap().parse::<i32>().unwrap();
       let mut connections: Vec<i32> = Vec::new();

       for s in elements {
           if s ==  "<->" {
               continue;
           }
           connections.push(s.trim_matches(|c| c == ',').parse::<i32>().unwrap());
       }

       programs.insert(id, Program { id: id, pipes: connections });
       ids.push(id);
   }
   
   let mut group_count = 0;

   loop {
       if programs.len() == 0 {
           break;
       }
       
       let mut id: i32;
       loop {
           if ids.len() == 0 {
               id = -1;
               break;
           }

           id = ids.pop().unwrap();
           if programs.contains_key(&id) {
               break;
           }
       }

       if id == -1 {
           break;
       }

       let current = get_program(id, &programs);    
       let members = process_group(&current, &programs);

       for m in members {
           programs.remove(&m);
       }

       group_count += 1;
   }
   
   println!("Count {}", group_count);
}

fn get_program(id: i32, set: &HashMap<i32, Program>) -> Program {
   set.get(&id).unwrap().clone()

}

fn process_group(root: &Program, set: &HashMap<i32, Program>) -> Vec<i32> {
   let mut queue: VecDeque<i32> = VecDeque::new();
   let mut members = Vec::new();

   for c in &root.pipes {
       queue.push_back(*c);
   }

   loop {
       let current = match queue.pop_front() {
           None => break,
           Some(e) => e,
       };

       if members.contains(&current) {
           continue;
       }

       let current_program = set.get(&current).unwrap();

       for c in &current_program.pipes {
           if *c == root.id {
               continue;
           }

           queue.push_back(*c);
       }

       members.push(current);
   }

   members
}