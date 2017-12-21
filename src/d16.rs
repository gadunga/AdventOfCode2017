use std::collections::VecDeque;
use std::collections::HashMap;

#[derive(Debug, Clone)]
enum Operation {
    Spin(usize),
    Exchange(usize, usize),
    Partner(u8, u8),
}

fn parse(data: &str) -> Vec<Operation> {
    let moves: Vec<&str> = data.split(',').collect();
    let mut result: Vec<Operation> = Vec::new();

    for s in &moves {
        if s.starts_with("s") {
            result.push(Operation::Spin(s[1..].parse().unwrap()));
            continue;
        }

        if s.starts_with("x") {
            let mut exchanges = s[1..].split('/');
            result.push(Operation::Exchange(exchanges.next().unwrap().parse().unwrap(), exchanges.next().unwrap().parse().unwrap()));
            continue;
        }

        if s.starts_with("p") {
            let exchanges: Vec<&str> = s[1..].split('/').collect();
            result.push(Operation::Partner(exchanges[0].chars().next().unwrap() as u8, exchanges[1].chars().next().unwrap() as u8));
        }
    }

    result
}

#[allow(dead_code)]
pub fn problem1(data: &str) {
    let moves = parse(data);
    let mut programs: VecDeque<u8> = VecDeque::new();
    for i in 0..16 {
        programs.push_back('a' as u8 + i as u8);
    }

    for instr in &moves {
        match *instr {
            Operation::Spin(n) => {
                for _ in 0..n {
                    let temp = programs.pop_back().unwrap();
                    programs.push_front(temp);
                }
            },
            Operation::Exchange(a, b) => {
                programs.swap(a, b);
            },
            Operation::Partner(a, b) => {
                let a = programs.iter().position(|fa| a == *fa).unwrap();
                let b = programs.iter().position(|fb| b == *fb).unwrap();
                programs.swap(a, b);
            },
        }
    }

    for c in programs {
        print!("{}", c as char);
    }
    print!("\n")
}

#[allow(dead_code)]
pub fn problem2(data: &str) {
    let moves = parse(data);
    let mut programs: VecDeque<u8> = VecDeque::new();

    let mut cycles: HashMap<VecDeque<u8>, usize> = HashMap::new();
    let mut lookup: HashMap<usize, VecDeque<u8>> = HashMap::new();
    let iterations = 1000000000;

    for i in 0..16 {
        programs.push_back('a' as u8 + i as u8);
    }

    for i in 0..iterations {
        if cycles.contains_key(&programs) {
            let cycle = i - cycles.get(&programs).unwrap();
            let index = (iterations - i) % cycle;
            programs = lookup.get(&index).cloned().unwrap();
            break;
        }

        cycles.insert(programs.clone(), i);
        lookup.insert(i, programs.clone());

        for instr in &moves {
            match *instr {
                Operation::Spin(n) => {
                    for _ in 0..n {
                        let temp = programs.pop_back().unwrap();
                        programs.push_front(temp);
                    }
                },
                Operation::Exchange(a, b) => {
                    programs.swap(a, b);
                },
                Operation::Partner(a, b) => {
                    let a = programs.iter().position(|fa| a == *fa).unwrap();
                    let b = programs.iter().position(|fb| b == *fb).unwrap();
                    programs.swap(a, b);
                },
            }
        }
    }

    for c in programs {
        print!("{}", c as char);
    }
    print!("\n")
}
