use std::collections::HashMap;
use std::collections::VecDeque;

#[derive(Debug, Clone)]
enum Instruction {
    SND(String),
    SET(String, i64),
    SETr(String, String),
    ADD(String, i64),
    ADDr(String, String),
    MUL(String, i64),
    MULr(String, String),
    MOD(String, i64),
    MODr(String, String),
    RCV(String),
    JGZ(String, i64),
    JGZr(String, String),
}

fn parse(data: &str) -> (Vec<Instruction>, HashMap<String, i64>) {
    let mut registers: HashMap<String, i64> = HashMap::new();
    let mut instructions: Vec<Instruction> = Vec::new();

    for s in data.lines() {
        let mut elements = s.split_whitespace();
        let instr = elements.next().unwrap();
        let register = elements.next().unwrap().to_string();
        match register.parse() {
            Ok(n) => registers.entry(register.clone()).or_insert(n),
            Err(_) => registers.entry(register.clone()).or_insert(0),
        };
        
        match instr {
            "snd" => instructions.push(Instruction::SND(register.clone())),
            "set" => {
                let val = elements.next().unwrap();
                match val.parse() {
                    Ok(n) => instructions.push(Instruction::SET(register.clone(), n)),
                    Err(_) => instructions.push(Instruction::SETr(register.clone(), val.to_string())),
                }
            },
            "add" => {
                let val = elements.next().unwrap();
                match val.parse() {
                    Ok(n) => instructions.push(Instruction::ADD(register.clone(), n)),
                    Err(_) => instructions.push(Instruction::ADDr(register.clone(), val.to_string())),
                }
            },
            "mul" => {
                let val = elements.next().unwrap();
                match val.parse() {
                    Ok(n) => instructions.push(Instruction::MUL(register.clone(), n)),
                    Err(_) => instructions.push(Instruction::MULr(register.clone(), val.to_string())),
                }
            },
            "mod" => {
                let val = elements.next().unwrap();
                match val.parse() {
                    Ok(n) => instructions.push(Instruction::MOD(register.clone(), n)),
                    Err(_) => instructions.push(Instruction::MODr(register.clone(), val.to_string())),
                }
            },
            "rcv" => instructions.push(Instruction::RCV(register.clone())),
            "jgz" => {
                let val = elements.next().unwrap();
                match val.parse() {
                    Ok(n) => instructions.push(Instruction::JGZ(register.clone(), n)),
                    Err(_) => instructions.push(Instruction::JGZr(register.clone(), val.to_string())),
                }
            },
            _ => println!("Unexpected instruction: {}", s),
        }
    }

    (instructions, registers)
}

pub fn problem1(data: &str) {
    let (instructions, mut registers) = parse(data);
    let mut current_instruction: i64 = 0;
    let mut last_sound = 0;

    loop {
        if current_instruction >= instructions.len() as i64 {
            break;
        }
        match instructions[current_instruction as usize] {
            Instruction::SND(ref n) => last_sound = *registers.get(n).unwrap(),
            Instruction::SET(ref n, v) => *registers.get_mut(n).unwrap() = v as i64,
            Instruction::SETr(ref n, ref r) => *registers.get_mut(n).unwrap() = *registers.get(r).unwrap(),
            Instruction::ADD(ref n, v) => *registers.get_mut(n).unwrap() += v as i64,
            Instruction::ADDr(ref n, ref r) => *registers.get_mut(n).unwrap() += *registers.get(r).unwrap(),
            Instruction::MUL(ref n, v) => *registers.get_mut(n).unwrap() *= v as i64,
            Instruction::MULr(ref n, ref r) => *registers.get_mut(n).unwrap() *= *registers.get(r).unwrap(),
            Instruction::MOD(ref n, v) => *registers.get_mut(n).unwrap() %= v as i64,
            Instruction::MODr(ref n, ref r) => *registers.get_mut(n).unwrap() %= *registers.get(r).unwrap(),
            Instruction::RCV(ref n) => {
                let value = *registers.get(n).unwrap();
                if value != 0 {
                    println!("Recovered: {}", last_sound);
                    break;
                }
            },
            Instruction::JGZ(ref n, v) => {
                let value = *registers.get(n).unwrap();
                if value > 0 {
                    let offset = v;
                    if current_instruction + offset >= instructions.len() as i64 {
                        break;
                    }
                    current_instruction += offset;
                    continue;
                }
            },
            Instruction::JGZr(ref n, ref r) => {
                let value = *registers.get(n).unwrap();
                if value > 0 {
                    let offset = *registers.get(r).unwrap();
                    if current_instruction + offset >= instructions.len() as i64 {
                        break;
                    }
                    current_instruction += offset;
                    continue;
                }
            },
        }
        current_instruction += 1;
    }
}

#[derive(Debug)]
struct Program {
    registers: HashMap<String, i64>,
    rcv_queue: VecDeque<i64>,
    current_instruction: i64,
    send_count: i64,
}

pub fn problem2(data: &str) {
    let (instructions, mut registers) = parse(data);
    let mut a = Program{ registers: registers.clone(), rcv_queue: VecDeque::new(), current_instruction: 0, send_count: 0 };
    let mut b = Program{ registers: registers.clone(), rcv_queue: VecDeque::new(), current_instruction: 0, send_count: 0 };
    let instruction_len = instructions.len();
    *a.registers.get_mut("p").unwrap() = 0;
    *b.registers.get_mut("p").unwrap() = 1;
    loop {
        execute(&instructions, &mut a, &mut b.rcv_queue);
        execute(&instructions, &mut b, &mut a.rcv_queue);

        if a.current_instruction >= instruction_len as i64 ||
            b.current_instruction >= instruction_len as i64 || 
           (a.rcv_queue.len() == 0 && b.rcv_queue.len() == 0) {
            break;
        }        
    }

    println!("{}", b.send_count);

}

fn execute(code: &Vec<Instruction>, program: &mut Program, snd_queue: &mut VecDeque<i64>) {
    loop {
        if program.current_instruction >= code.len() as i64 {
            return;
        }

        match code[program.current_instruction as usize] {
            Instruction::SND(ref n) => { 
                snd_queue.push_back(*program.registers.get(n).unwrap());
                program.send_count += 1;
            },
            Instruction::SET(ref n, v) => *program.registers.get_mut(n).unwrap() = v as i64,
            Instruction::SETr(ref n, ref r) => *program.registers.get_mut(n).unwrap() = *program.registers.get(r).unwrap(),
            Instruction::ADD(ref n, v) => *program.registers.get_mut(n).unwrap() += v as i64,
            Instruction::ADDr(ref n, ref r) => *program.registers.get_mut(n).unwrap() += *program.registers.get(r).unwrap(),
            Instruction::MUL(ref n, v) => *program.registers.get_mut(n).unwrap() *= v as i64,
            Instruction::MULr(ref n, ref r) => *program.registers.get_mut(n).unwrap() *= *program.registers.get(r).unwrap(),
            Instruction::MOD(ref n, v) => *program.registers.get_mut(n).unwrap() %= v as i64,
            Instruction::MODr(ref n, ref r) => *program.registers.get_mut(n).unwrap() %= *program.registers.get(r).unwrap(),
            Instruction::RCV(ref n) => {
                if program.rcv_queue.len() == 0 {
                    return;
                }
                let value = program.rcv_queue.pop_front().unwrap();
                *program.registers.get_mut(n).unwrap() = value;
            },
            Instruction::JGZ(ref n, v) => {
                let value = *program.registers.get(n).unwrap();
                if value > 0 {
                    let offset = v;
                    if program.current_instruction + offset >= code.len() as i64 {
                        return;
                    }
                    program.current_instruction += offset;
                    continue;
                }
            },
            Instruction::JGZr(ref n, ref r) => {
                let value = *program.registers.get(n).unwrap();
                if value > 0 {
                    let offset = *program.registers.get(r).unwrap();
                    if program.current_instruction + offset >= code.len() as i64 {
                        return;
                    }
                    program.current_instruction += offset;
                    continue;
                }
            },
        }
        program.current_instruction += 1;
    }
}