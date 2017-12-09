use std::collections::HashMap;

#[derive(Debug, Clone)]
enum Operator {
    GT,
    LT,
    EQ,
    GTEQ,
    LTEQ,
    NE,
}

#[derive(Debug, Clone)]
struct Instruction {
    register: String,
    increment: bool,
    inc_val: i32,
    op_reg: String,
    op: Operator,
    val: i32,
}

#[allow(dead_code)]
pub fn problem1(data: &str) {
    let mut instructions: Vec<Instruction> = Vec::new();
    let mut registers = HashMap::new();

    for s in data.lines(){
        instructions.push(parse_instruction(s));
    }

    for i in instructions {
        let compare_register_value = *registers.entry(i.op_reg).or_insert(0);
        let register_value = registers.entry(i.register).or_insert(0);
        let compare_result = match i.op {
            Operator::GT => compare_register_value > i.val,
            Operator::LT => compare_register_value < i.val,
            Operator::EQ => compare_register_value == i.val,
            Operator::LTEQ => compare_register_value <= i.val,
            Operator::GTEQ => compare_register_value >= i.val,
            Operator::NE => compare_register_value != i.val,
        };

        if compare_result {
            if i.increment {
                *register_value += i.inc_val;
            } else {
                *register_value -= i.inc_val;
            }
        }
    }

    let mut max_val: i32 = 0;
    for (_, v) in &registers {
        max_val = if *v > max_val { *v } else { max_val };
    }

    println!("{}", max_val);
}

#[allow(dead_code)]
pub fn problem2(data: &str) {
    let mut instructions: Vec<Instruction> = Vec::new();
    let mut registers = HashMap::new();

    for s in data.lines(){
        instructions.push(parse_instruction(s));
    }

    let mut max_val: i32 = 0;

    for i in instructions {
        let compare_register_value = *registers.entry(i.op_reg).or_insert(0);
        let register_value = registers.entry(i.register).or_insert(0);
        let compare_result = match i.op {
            Operator::GT => compare_register_value > i.val,
            Operator::LT => compare_register_value < i.val,
            Operator::EQ => compare_register_value == i.val,
            Operator::LTEQ => compare_register_value <= i.val,
            Operator::GTEQ => compare_register_value >= i.val,
            Operator::NE => compare_register_value != i.val,
        };

        if compare_result {
            if i.increment {
                *register_value += i.inc_val;
            } else {
                *register_value -= i.inc_val;
            }

            max_val = if *register_value > max_val { *register_value } else { max_val };
        }
    }

    println!("{}", max_val);
}

fn parse_instruction(line: &str) -> Instruction {
    let mut elements = line.split_whitespace();
    let name = elements.next().unwrap();
    
    let operation_str = elements.next().unwrap();
    let inc = if operation_str == "inc" {
        true
    } else {
        false
    };

    let inc_value = elements.next().unwrap().parse::<i32>().unwrap();

    elements.next(); // "if"

    let cond_reg = elements.next().unwrap();
    let op_str = elements.next().unwrap();
    let op = match op_str {
        ">" => Operator::GT,
        "<" => Operator::LT,
        "==" => Operator::EQ,
        "<=" => Operator::LTEQ,
        ">=" => Operator::GTEQ,
        "!=" => Operator::NE,
        _ => {
            println!("{}", op_str);
            panic!("Unregcognized symbol") 
        },
    };

    let comp_value = elements.next().unwrap().parse::<i32>().unwrap();

    Instruction{register: name.to_string(), increment: inc, inc_val: inc_value, op_reg: cond_reg.to_string(), op: op, val: comp_value }
}