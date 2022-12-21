use aocf::Aoc;
use std::collections::HashMap;
use z3::ast::Ast;
use std::ops::{Add, Sub, Mul};

#[derive(Debug)]
enum Operation {
    Value(usize),
    Add(String, String),
    Sub(String, String),
    Mult(String, String),
    Div(String, String),
}

fn resolve(name: &str, map: &HashMap<String, Operation>) -> usize
{
    let operation = map.get(name).unwrap();
    match operation {
        Operation::Value(value) => *value,
        Operation::Add(a, b) => resolve(a, map) + resolve(b, map),
        Operation::Sub(a, b) => resolve(a, map) - resolve(b, map),
        Operation::Mult(a, b) => resolve(a, map) * resolve(b, map),
        Operation::Div(a, b) => resolve(a, map) / resolve(b, map),
    }
}

fn main() {
    let mut aoc = Aoc::new()
        .year(Some(2022))
        .day(Some(21))
        .init()
        .unwrap();

    // Get input data (don't force)
    let input = aoc.get_input(false).unwrap();

    let mut map = HashMap::new();

    for line in input.lines() {
        let mut parts = line.split(": ");
        let name = parts.next().unwrap();
        let operation = parts.next().unwrap();

        if let Ok(value) = operation.parse::<usize>() {
            map.insert(name.to_string(), Operation::Value(value));
        } else {
            let mut parts = operation.split(' ');
            let a = parts.next().unwrap();
            let op = parts.next().unwrap();
            let b = parts.next().unwrap();

            map.insert(name.to_string(), match op {
                "+" => Operation::Add(a.to_string(), b.to_string()),
                "-" => Operation::Sub(a.to_string(), b.to_string()),
                "*" => Operation::Mult(a.to_string(), b.to_string()),
                "/" => Operation::Div(a.to_string(), b.to_string()),
                _ => unimplemented!(),
            });
        }
    }

    println!("{:?}", map);
    println!("{}", resolve("root", &map));

    let cfg = z3::Config::new();
    let ctx = z3::Context::new(&cfg);
    let solver = z3::Solver::new(&ctx);

    let mut named_nodes = HashMap::new();
    for (name, _) in map.iter() {
        let node = z3::ast::Int::new_const(&ctx, name.to_string());
        named_nodes.insert(name.clone(), node);
    }

    for (name, operation) in map.iter() {
        let name_node = named_nodes.get(name).unwrap();

        if name == "humn" {
            solver.assert(&named_nodes["humn"].lt(&z3::ast::Int::from_u64(&ctx, 3412650897406)));
            continue;
        }
        if name == "root" {
            if let Operation::Add(a, b) = &map["root"] {
                solver.assert(&(&named_nodes[a])._eq(&named_nodes[b]));
            } else {
                unimplemented!();
            }
            continue;
        }

        match operation {
            Operation::Value(value) => solver.assert(
                &name_node._eq(&z3::ast::Int::from_u64(&ctx, *value as u64))
            ),
            Operation::Add(a, b) => solver.assert(
                &name_node._eq(&(&named_nodes[a]).add(&named_nodes[b]))
            ),
            Operation::Sub(a, b) => solver.assert(
                &name_node._eq(&(&named_nodes[a]).sub(&named_nodes[b]))
            ),
            Operation::Mult(a, b) => solver.assert(
                &name_node._eq(&(&named_nodes[a]).mul(&named_nodes[b]))
            ),
            Operation::Div(a, b) => solver.assert(
                &name_node._eq(&(&named_nodes[a]).div(&named_nodes[b]))
            ),
        }
    }

    println!("{:?}", solver.check());
    println!("{:?}", solver.get_model());
}
