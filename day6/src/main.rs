use aocf::Aoc;

fn all_unique(data: &[u8]) -> bool {
    for i in 0..data.len() {
        for j in (i+1)..data.len() {
            if data[i] == data[j] {
                return false;
            }
        }
    }

    true
}

fn main() {
    let mut aoc = Aoc::new()
        .year(Some(2022))
        .day(Some(6))
        .init()
        .unwrap();

    // Get input data (don't force)
    let input = aoc.get_input(false).unwrap();

    for (i, win) in input.as_bytes().windows(4).enumerate() {
        if all_unique(win) {
            println!("{}", i + 4);
            break;
        }
    }

    for (i, win) in input.as_bytes().windows(14).enumerate() {
        if all_unique(win) {
            println!("{}", i + 14);
            break;
        }
    }
}
