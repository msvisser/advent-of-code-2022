use aocf::Aoc;

fn main() {
    let mut aoc = Aoc::new()
        .year(Some(2022))
        .day(Some(10))
        .init()
        .unwrap();

    // Get input data (don't force)
    let input = aoc.get_input(false).unwrap();

    let mut cycle = 0;
    let mut x = 1;
    let mut strength = 0;
    let mut temp_addend = None;

    let mut lines = input.lines();
    loop {
        if (cycle % 40) == 19 {
            strength += x * (cycle + 1);
        }
        if (x-1..=x+1).contains(&(cycle % 40)) {
            print!("#");
        } else {
            print!(" ");
        }
        if (cycle % 40) == 39 {
            println!();
        }

        if let Some(addend) = temp_addend.take() {
            x += addend;
        } else {
            if let Some(line) = lines.next() {
                if line.starts_with("addx ") {
                    let value = line.strip_prefix("addx ").unwrap().parse::<isize>().unwrap();
                    temp_addend = Some(value);
                }
            } else {
                break;
            }
        }
        cycle += 1;
    }

    println!("{}", strength);
}
