use std::{collections::{HashMap, VecDeque, HashSet}, mem::swap};

use aocf::Aoc;
use glam::IVec2;

fn main() {
    let mut aoc = Aoc::new()
        .year(Some(2022))
        .day(Some(23))
        .init()
        .unwrap();

    // Get input data (don't force)
    let input = aoc.get_input(false).unwrap();

    let mut elves = Vec::new();
    for (y, line) in input.lines().enumerate() {
        for (x, char) in line.chars().enumerate() {
            if char == '#' {
                elves.push(IVec2::new(x as i32, y as i32));
            }
        }
    }

    let mut consider_dir = VecDeque::new();
    consider_dir.push_back(((0, 1, 2), IVec2::NEG_Y));
    consider_dir.push_back(((5, 6, 7), IVec2::Y));
    consider_dir.push_back(((0, 3, 5), IVec2::NEG_X));
    consider_dir.push_back(((2, 4, 7), IVec2::X));

    let mut elves_set: HashSet<IVec2> = HashSet::from_iter(elves.iter().cloned());
    let mut new_elves = Vec::with_capacity(elves.len());
    for i in 1.. {
        let mut new_positions = HashMap::new();
        let mut not_moved = 0;
        for (index, &elf) in elves.iter().enumerate() {
            let dirs = [IVec2::new(-1, -1), IVec2::NEG_Y, IVec2::new(1, -1), IVec2::NEG_X, IVec2::X, IVec2::new(-1, 1), IVec2::Y, IVec2::new(1, 1)];
            let occupied: Vec<bool> = dirs.iter().map(|&d| elves_set.contains(&(elf + d))).collect();

            let new_pos = if occupied.iter().all(|&b| b == false) {
                not_moved += 1;
                elf
            } else {
                let mut result = None;
                for &((a, b, c), offset) in consider_dir.iter() {
                    if !occupied[a] && !occupied[b] && !occupied[c] {
                        result = Some(elf + offset);
                        break;
                    }
                }
                result.unwrap_or(elf)
            };

            new_elves.push(new_pos);
            let list: &mut Vec<usize> = new_positions.entry(new_pos).or_default();
            list.push(index);
        }

        for (_, indices) in new_positions.iter() {
            if indices.len() > 1 {
                for &index in indices {
                    new_elves[index] = elves[index];
                }
            }
        }

        swap(&mut elves, &mut new_elves);
        new_elves.clear();
        elves_set.clear();
        elves_set.extend(elves.iter().cloned());

        consider_dir.rotate_left(1);
        if not_moved == elves.len() {
            // Part 2
            println!("{}", i);
            break;
        }

        if i == 10 {
            // Part 1
            let mut min_pos = elves[0];
            let mut max_pos = elves[0];
            for &elf in elves.iter() {
                min_pos = min_pos.min(elf);
                max_pos = max_pos.max(elf);
            }

            let rect = (max_pos - min_pos) + IVec2::ONE;
            println!("{}", rect.x * rect.y - elves.len() as i32);
        }
    }


}
