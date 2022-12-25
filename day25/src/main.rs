use aocf::Aoc;

fn main() {
    let mut aoc = Aoc::new()
        .year(Some(2022))
        .day(Some(25))
        .init()
        .unwrap();

    // Get input data (don't force)
    let input = aoc.get_input(false).unwrap();

    let mut sum = 0;
    for line in input.lines() {
        let mut value = 0isize;
        for char in line.chars() {
            value *= 5;
            match char {
                '0' => value += 0,
                '1' => value += 1,
                '2' => value += 2,
                '-' => value -= 1,
                '=' => value -= 2,
                _ => unreachable!(),
            }
        }
        sum += value;
    }

    println!("{}", sum);

    let mut s = String::new();

    let mut target_value = sum;
    let mut carry = false;
    while target_value > 0 {
        let mut rem = target_value % 5;
        target_value /= 5;

        if carry {
            rem += 1;
            carry = false;
        }

        match rem {
            0 => s.push('0'),
            1 => s.push('1'),
            2 => s.push('2'),
            3 => {
                s.push('=');
                carry = true;
            }
            4 => {
                s.push('-');
                carry = true;
            }
            5 => {
                s.push('0');
                carry = true;
            }
            _ => unreachable!(),
        }
    }

    let s2: String = s.chars().rev().collect();
    println!("{}", s2);

}
