use std::collections::HashMap;

#[derive(Debug)]
struct Program {
    name: String,
    weight: i32,
    children_names: Vec<String>,
}

#[allow(dead_code)]
pub fn day7_problem1(data: &str) {
    let mut programs: Vec<String> = Vec::new();
    let mut parents: Vec<Program> = Vec::new();

    for s in data.lines() {
        let mut current = Program { name: "".to_string(), weight: 0, children_names: Vec::new() };
        let mut elements = s.split_whitespace();

        current.name = elements.next().unwrap().to_string();
        current.weight = elements.next().unwrap().trim_matches(|c| c == '(' || c == ')').parse::<i32>().unwrap();

        programs.push(current.name.clone());

        for c in elements {
            if c ==  "->" {
                continue;
            }
            current.children_names.push(c.trim_matches(',').to_string());
        }

        parents.push(current);
    }

    for p in parents {
        for s in &p.children_names {
            let index = programs.iter().position(|x| x == s).unwrap();
            programs.remove(index);
        }
    }

    println!("{:?}", programs);
}

#[allow(dead_code)]
pub fn day7_problem2(data: &str) {
    let mut program_weights = HashMap::new();
    let mut parents = HashMap::new();

    for s in data.lines() {
        let mut current = Program { name: "".to_string(), weight: 0, children_names: Vec::new() };
        let mut elements = s.split_whitespace();

        current.name = elements.next().unwrap().to_string();
        current.weight = elements.next().unwrap().trim_matches(|c| c == '(' || c == ')').parse::<i32>().unwrap();

        for c in elements {
            if c ==  "->" {
                continue;
            }
            current.children_names.push(c.trim_matches(',').to_string());
        }
        program_weights.insert(current.name.clone(), current.weight);
        parents.insert(current.name.clone(), current);
    }

    let mut elements: Vec<&Program> = Vec::new();

    for (_, v) in &parents {
        elements.push(v);
    }

    for e in elements {
        calculate_weight(&e, &parents, &program_weights);
    }
}

fn calculate_weight(prog: &Program, parents: &HashMap<String, Program>, weights: &HashMap<String, i32>) -> i32 {
    let mut sum = prog.weight;
    let mut child_weight_count = HashMap::new();
    let mut child_weights = HashMap::new();
    for s in &prog.children_names {
        if parents.contains_key(s) {
            let child_prog = parents.get(s).unwrap();
            let weight = calculate_weight(child_prog, parents, weights);
            let count = child_weight_count.entry(weight).or_insert(0);
            *count += 1;
            sum += weight;
            child_weights.insert(s, weight);
            continue;
        }
        
        let count = child_weight_count.entry(*weights.get(s).unwrap()).or_insert(0);
        *count += 1;
        sum += weights.get(s).unwrap();
        child_weights.insert(s, *weights.get(s).unwrap());
    }

    if prog.children_names.len() != 0 as usize && child_weight_count.len() != 1 {
        println!("{} ({}): {:?}", prog.name, prog.weight, child_weights);
        println!("{:?}", child_weight_count);
    }

    sum
}