use std::collections::HashSet;

use aocf::Aoc;
use glam::IVec3;
use pathfinding::prelude::bfs;

fn can_move_to(pos: IVec3, field_width: i32, field_height: i32, blizzards: &HashSet<IVec3>) -> bool {
    if pos.x == 0 && pos.y == -1 {
        return true;
    }
    if pos.x == (field_width - 1) && pos.y == field_height {
        return true;
    }

    if pos.x >= 0 && pos.y >= 0 && pos.x < field_width && pos.y < field_height {
        !blizzards.contains(&pos)
    } else {
        false
    }
}

fn neighbours(pos: IVec3, field_width: i32, field_height: i32, period: i32, blizzards: &HashSet<IVec3>) -> Vec<IVec3> {
    let mut result = Vec::new();

    for dir in [IVec3::NEG_X, IVec3::X, IVec3::NEG_Y, IVec3::Y, IVec3::ZERO] {
        let mut new_pos = pos + dir;
        new_pos.z = (new_pos.z + 1) % period;
        if can_move_to(new_pos, field_width, field_height, &blizzards) {
            result.push(new_pos);
        }
    }

    result
}

fn main() {
    let mut aoc = Aoc::new()
        .year(Some(2022))
        .day(Some(24))
        .init()
        .unwrap();

    // Get input data (don't force)
    let input = aoc.get_input(false).unwrap();

    let field_height = (input.lines().count() as i32) - 2;
    let field_width = (input.lines().next().unwrap().chars().count() as i32) - 2;
    let period = num::integer::lcm(field_width, field_height);

    println!("{} {} {}", field_width, field_height, period);

    let mut blizzards = HashSet::new();
    for (y, line) in input.lines().skip(1).enumerate() {
        for (x, char) in line.chars().skip(1).enumerate() {
            match char {
                '<' => {
                    for time in 0..period {
                        let pos_x = (x as i32 - time).rem_euclid(field_width as i32);
                        blizzards.insert(IVec3::new(pos_x, y as i32, time));
                    }
                }
                '>' => {
                    for time in 0..period {
                        let pos_x = (x as i32 + time).rem_euclid(field_width as i32);
                        blizzards.insert(IVec3::new(pos_x, y as i32, time));
                    }
                }
                '^' => {
                    for time in 0..period {
                        let pos_y = (y as i32 - time).rem_euclid(field_height as i32);
                        blizzards.insert(IVec3::new(x as i32, pos_y, time));
                    }
                }
                'v' => {
                    for time in 0..period {
                        let pos_y = (y as i32 + time).rem_euclid(field_height as i32);
                        blizzards.insert(IVec3::new(x as i32, pos_y, time));
                    }
                }
                _ => {}
            }
        }
    }

    let result_path = bfs(
        &IVec3::new(0, -1, 0),
        |&pos| neighbours(pos, field_width, field_height, period, &blizzards),
        |pos| pos.x == (field_width - 1) && pos.y == field_height
    ).unwrap();
    let part1 = result_path.len() - 1;
    println!("{}", part1);

    let result_path = bfs(
        &IVec3::new(field_width - 1, field_height, (part1 as i32) % period),
        |&pos| neighbours(pos, field_width, field_height, period, &blizzards),
        |pos| pos.x == 0 && pos.y == -1
    ).unwrap();
    let part2a = result_path.len() - 1;

    let result_path = bfs(
        &IVec3::new(0, -1, (part1 as i32 + part2a as i32) % period),
        |&pos| neighbours(pos, field_width, field_height, period, &blizzards),
        |pos| pos.x == (field_width - 1) && pos.y == field_height
    ).unwrap();
    let part2b = result_path.len() - 1;
    println!("{} {} {}", part2a, part2b, part1 + part2a + part2b);
}
