use aocf::Aoc;
use std::collections::HashSet;

fn main() {
    let mut aoc = Aoc::new()
        .year(Some(2022))
        .day(Some(14))
        .init()
        .unwrap();

    // Get input data (don't force)
    let input = aoc.get_input(false).unwrap();
    // let input = "498,4 -> 498,6 -> 496,6\n503,4 -> 502,4 -> 502,9 -> 494,9\n";

    let mut stones = HashSet::new();
    let mut highest_y = 0;

    for line in input.lines() {
        let coords: Vec<_> = line.split(" -> ").map(|p| {
            let mut ps = p.split(",");
            (ps.next().unwrap().parse::<usize>().unwrap(), ps.next().unwrap().parse::<usize>().unwrap())
        }).collect();

        for pair in coords.windows(2) {
            let (ax, ay) = pair[0];
            let (bx, by) = pair[1];
            if ax == bx {
                for y in usize::min(ay, by)..=usize::max(ay, by) {
                    stones.insert((ax, y));
                }
            } else {
                for x in usize::min(ax, bx)..=usize::max(ax, bx) {
                    stones.insert((x, ay));
                }
            }
        }

        for (_, y) in coords {
            highest_y = usize::max(highest_y, y);
        }
    }

    let mut occupied_spaces = stones.clone();
    let mut rested_sand = HashSet::new();

    let mut done = false;
    while !done {
        let (mut sand_x, mut sand_y) = (500, 0);
        loop {
            if sand_y > highest_y {
                done = true;
                break;
            } else if !occupied_spaces.contains(&(sand_x, sand_y + 1)) {
                sand_y += 1;
            } else if !occupied_spaces.contains(&(sand_x - 1, sand_y + 1)) {
                sand_x -= 1;
                sand_y += 1;
            } else if !occupied_spaces.contains(&(sand_x + 1, sand_y + 1)) {
                sand_x += 1;
                sand_y += 1;
            } else {
                occupied_spaces.insert((sand_x, sand_y));
                rested_sand.insert((sand_x, sand_y));
                break;
            }
        }
    }

    println!("{}", rested_sand.len());

    let mut occupied_spaces = stones.clone();
    let mut rested_sand = HashSet::new();

    let mut done = false;
    while !done {
        let (mut sand_x, mut sand_y) = (500, 0);
        loop {
            if occupied_spaces.contains(&(500, 0)) {
                done = true;
                break;
            } else if !occupied_spaces.contains(&(sand_x, sand_y + 1)) && sand_y < (highest_y + 1) {
                sand_y += 1;
            } else if !occupied_spaces.contains(&(sand_x - 1, sand_y + 1)) && sand_y < (highest_y + 1) {
                sand_x -= 1;
                sand_y += 1;
            } else if !occupied_spaces.contains(&(sand_x + 1, sand_y + 1)) && sand_y < (highest_y + 1) {
                sand_x += 1;
                sand_y += 1;
            } else {
                occupied_spaces.insert((sand_x, sand_y));
                rested_sand.insert((sand_x, sand_y));
                break;
            }
        }
    }

    println!("{}", rested_sand.len());
}
