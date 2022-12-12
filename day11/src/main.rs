use aocf::Aoc;

#[derive(Debug, Clone, Copy)]
enum Operation {
    Add(usize),
    Multiply(usize),
    Square,
}

#[derive(Debug, Clone)]
struct Monkey {
    items: Vec<usize>,
    operation: Operation,
    test_value: usize,
    target_if_true: usize,
    target_if_false: usize,
    inspect_count: usize,
}

fn main() {
    let mut aoc = Aoc::new()
        .year(Some(2022))
        .day(Some(11))
        .init()
        .unwrap();

    // Get input data (don't force)
    let input = aoc.get_input(false).unwrap();
    // let input = "Monkey 0:1\n  Starting items: 79, 98\n  Operation: new = old * 19\n  Test: divisible by 23\n    If true: throw to monkey 2\n    If false: throw to monkey 3\n\nMonkey 1:\n  Starting items: 54, 65, 75, 74\n  Operation: new = old + 6\n  Test: divisible by 19\n    If true: throw to monkey 2\n    If false: throw to monkey 0\n\nMonkey 2:\n  Starting items: 79, 60, 97\n  Operation: new = old * old\n  Test: divisible by 13\n    If true: throw to monkey 1\n    If false: throw to monkey 3\n\nMonkey 3:\n  Starting items: 74\n  Operation: new = old + 3\n  Test: divisible by 17\n    If true: throw to monkey 0\n    If false: throw to monkey 1";

    let monkeys_lines = input.split("\n\n");

    let mut monkeys = Vec::new();
    let mut monkeys2 = Vec::new();
    for monkey in monkeys_lines {
        let mut monkey_lines = monkey.lines();
        let _ = monkey_lines.next().unwrap();
        let items_line = monkey_lines.next().unwrap();
        let operation_line = monkey_lines.next().unwrap();
        let test_line = monkey_lines.next().unwrap();
        let target_true_line = monkey_lines.next().unwrap();
        let target_false_line = monkey_lines.next().unwrap();

        let items_line = items_line.strip_prefix("  Starting items: ").unwrap();
        let items = items_line.split(", ").map(|i| i.parse::<usize>().unwrap()).collect();

        let operation_line = operation_line.strip_prefix("  Operation: new = ").unwrap();
        let operation = if operation_line == "old * old" {
            Operation::Square
        } else if operation_line.starts_with("old + ") {
            let operation_value = operation_line.strip_prefix("old + ").unwrap().parse::<usize>().unwrap();
            Operation::Add(operation_value)
        } else if operation_line.starts_with("old * ") {
            let operation_value = operation_line.strip_prefix("old * ").unwrap().parse::<usize>().unwrap();
            Operation::Multiply(operation_value)
        } else {
            unimplemented!()
        };

        let test_line = test_line.strip_prefix("  Test: divisible by ").unwrap();
        let test_value = test_line.parse::<usize>().unwrap();

        let target_true_line = target_true_line.strip_prefix("    If true: throw to monkey ").unwrap();
        let target_if_true = target_true_line.parse::<usize>().unwrap();

        let target_false_line = target_false_line.strip_prefix("    If false: throw to monkey ").unwrap();
        let target_if_false = target_false_line.parse::<usize>().unwrap();

        let monkey = Monkey {
            items,
            operation,
            test_value,
            target_if_true,
            target_if_false,
            inspect_count: 0,
        };
        monkeys.push(monkey.clone());
        monkeys2.push(monkey);
    }

    for _ in 0..20 {
        let mut give_monkey_items = Vec::new();
        for monkey_index in 0..monkeys.len() {
            let monkey = &mut monkeys[monkey_index];
            for item in monkey.items.drain(..) {
                let new_item = match monkey.operation {
                    Operation::Add(addend) => item + addend,
                    Operation::Multiply(multiplicand) => item * multiplicand,
                    Operation::Square => item * item,
                };
                let new_item = new_item / 3;

                if new_item % monkey.test_value == 0 {
                    give_monkey_items.push((monkey.target_if_true, new_item));
                } else {
                    give_monkey_items.push((monkey.target_if_false, new_item));
                }

                monkey.inspect_count += 1;
            }

            for (target, item) in give_monkey_items.drain(..) {
                monkeys[target].items.push(item);
            }
        }
    }

    let mut inspect_counts: Vec<_> = monkeys.iter().map(|m| m.inspect_count).collect();
    inspect_counts.sort_by(|lhs, rhs| rhs.cmp(lhs));
    println!("{}", inspect_counts[0] * inspect_counts[1]);

    let modulo_factor: usize = monkeys2.iter().map(|m| m.test_value).product();

    for _ in 0..10000 {
        let mut give_monkey_items = Vec::new();
        for monkey_index in 0..monkeys2.len() {
            let monkey = &mut monkeys2[monkey_index];
            for item in monkey.items.drain(..) {
                let new_item = match monkey.operation {
                    Operation::Add(addend) => item + addend,
                    Operation::Multiply(multiplicand) => item * multiplicand,
                    Operation::Square => item * item,
                } % modulo_factor;

                if new_item % monkey.test_value == 0 {
                    give_monkey_items.push((monkey.target_if_true, new_item));
                } else {
                    give_monkey_items.push((monkey.target_if_false, new_item));
                }

                monkey.inspect_count += 1;
            }

            for (target, item) in give_monkey_items.drain(..) {
                monkeys2[target].items.push(item);
            }
        }
    }

    let mut inspect_counts: Vec<_> = monkeys2.iter().map(|m| m.inspect_count).collect();
    inspect_counts.sort_by(|lhs, rhs| rhs.cmp(lhs));
    println!("{}", inspect_counts[0] * inspect_counts[1]);
}
