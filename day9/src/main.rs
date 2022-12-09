use aocf::Aoc;
use std::collections::HashSet;

fn move_tail(head_x: isize, head_y: isize, tail_x: isize, tail_y: isize) -> (isize, isize) {
    let diff_tail_x: isize = head_x - tail_x;
    let diff_tail_y: isize = head_y - tail_y;

    if (diff_tail_x == -2 && diff_tail_y == -1) || (diff_tail_x == -1 && diff_tail_y == -2) || (diff_tail_x == -2 && diff_tail_y == -2) {
        return (tail_x - 1, tail_y - 1);
    } else if (diff_tail_x == 2 && diff_tail_y == -1) || (diff_tail_x == 1 && diff_tail_y == -2) || (diff_tail_x == 2 && diff_tail_y == -2) {
        return (tail_x + 1, tail_y - 1);
    } else if (diff_tail_x == -2 && diff_tail_y == 1) || (diff_tail_x == -1 && diff_tail_y == 2) || (diff_tail_x == -2 && diff_tail_y == 2) {
        return (tail_x - 1, tail_y + 1);
    } else if (diff_tail_x == 2 && diff_tail_y == 1) || (diff_tail_x == 1 && diff_tail_y == 2) || (diff_tail_x == 2 && diff_tail_y == 2) {
        return (tail_x + 1, tail_y + 1);
    } else if diff_tail_x == -2 && diff_tail_y == 0 {
        return (tail_x - 1, tail_y);
    } else if diff_tail_x == 2 && diff_tail_y == 0 {
        return (tail_x + 1, tail_y);
    } else if diff_tail_y == -2 && diff_tail_x == 0 {
        return (tail_x, tail_y - 1);
    } else if diff_tail_y == 2 && diff_tail_x == 0 {
        return (tail_x, tail_y + 1);
    } else {
        // println!("{} {}", diff_tail_x, diff_tail_y);
        // println!("{} {}", tail_x, tail_y);
        assert!(diff_tail_x.abs() < 2);
        assert!(diff_tail_y.abs() < 2);
        return (tail_x, tail_y);
    }
}

fn main() {
    let mut aoc = Aoc::new()
        .year(Some(2022))
        .day(Some(9))
        .init()
        .unwrap();

    // Get input data (don't force)
    let input = aoc.get_input(false).unwrap();
    // let input = "R 4\nU 4\nL 3\nD 1\nR 4\nD 1\nL 5\nR 2\n";
    // let input = "R 5\nU 8\nL 8\nD 3\nR 17\nD 10\nL 25\nU 20\n";

    let mut head_x = 0;
    let mut head_y = 0;
    let mut tail_x = 0;
    let mut tail_y = 0;

    let mut set = HashSet::new();
    for line in input.lines() {
        let mut parts = line.split(" ");
        let direction = parts.next().unwrap();
        let count = parts.next().unwrap().parse::<usize>().unwrap();

        for _ in 0..count {
            set.insert((tail_x, tail_y));

            if direction == "L" {
                head_x -= 1;
            } else if direction == "R" {
                head_x += 1;
            } else if direction == "U" {
                head_y -= 1;
            } else if direction == "D" {
                head_y += 1;
            }

            (tail_x, tail_y) = move_tail(head_x, head_y, tail_x, tail_y);
        }
        set.insert((tail_x, tail_y));
    }

    println!("{}", set.len());

    let mut head_x: isize = 0;
    let mut head_y: isize = 0;
    let mut tails_x: [isize; 9] = [0; 9];
    let mut tails_y: [isize; 9] = [0; 9];

    let mut set = HashSet::new();
    for line in input.lines() {
        let mut parts = line.split(" ");
        let direction = parts.next().unwrap();
        let count = parts.next().unwrap().parse::<usize>().unwrap();

        for _ in 0..count {
            set.insert((tails_x[8], tails_y[8]));

            if direction == "L" {
                head_x -= 1;
            } else if direction == "R" {
                head_x += 1;
            } else if direction == "U" {
                head_y -= 1;
            } else if direction == "D" {
                head_y += 1;
            }

            (tails_x[0], tails_y[0]) = move_tail(head_x, head_y, tails_x[0], tails_y[0]);
            for i in 0..8 {
                (tails_x[i+1], tails_y[i+1]) = move_tail(tails_x[i], tails_y[i], tails_x[i+1], tails_y[i+1]);
            }
        }
        set.insert((tails_x[8], tails_y[8]));
    }

    println!("{}", set.len());
}
