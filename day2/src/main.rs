use aocf::Aoc;

fn main() {
    let mut aoc = Aoc::new()
        .year(Some(2022))
        .day(Some(2))
        .init()
        .unwrap();

    // Get input data (don't force)
    let input = aoc.get_input(false).unwrap();

    let mut score = 0;
    for line in input.lines() {
        let move_a = line.chars().nth(0).unwrap();
        let move_b = line.chars().nth(2).unwrap();

        // rock: A X
        // paper: B Y
        // scissors: C Z

        if move_b == 'X' {
            score += 1;
        } else if move_b == 'Y' {
            score += 2;
        } else {
            score += 3;
        }

        if (move_a == 'A' && move_b == 'X') || (move_a == 'B' && move_b == 'Y') || (move_a == 'C' && move_b == 'Z') {
            // draw 3 points
            score += 3;
        } else if (move_a == 'A' && move_b == 'Y') || (move_a == 'B' && move_b == 'Z') || (move_a == 'C' && move_b == 'X') {
            // win 6 points
            score += 6;
        }
    }

    println!("{}", score);

    score = 0;
    for line in input.lines() {
        let move_a = line.chars().nth(0).unwrap();
        let outcome = line.chars().nth(2).unwrap();

        if move_a == 'A' {
            if outcome == 'X' {
                // rock + lose = scissors
                score += 0 + 3;
            } else if outcome == 'Y' {
                // rock + draw = rock
                score += 3 + 1;
            } else if outcome == 'Z' {
                // rock + win = paper
                score += 6 + 2;
            }
        } else if move_a == 'B' {
            if outcome == 'X' {
                // paper + lose = rock
                score += 0 + 1;
            } else if outcome == 'Y' {
                // paper + draw = paper
                score += 3 + 2;
            } else if outcome == 'Z' {
                // paper + win = scissors
                score += 6 + 3;
            }
        } else {
            if outcome == 'X' {
                // scissors + lose = paper
                score += 0 + 2;
            } else if outcome == 'Y' {
                // scissors + draw = scissors
                score += 3 + 3;
            } else if outcome == 'Z' {
                // scissors + win = rock
                score += 6 + 1;
            }
        }

    }
    println!("{}", score);
}
