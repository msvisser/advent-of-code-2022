use aocf::Aoc;
use glam::IVec2;

#[derive(Debug, Clone, Copy)]
enum Space {
    Void,
    Free,
    Wall,
}

impl Space {
    fn is_free(&self) -> bool {
        match self {
            Space::Void => false,
            Space::Free => true,
            Space::Wall => false,
        }
    }

    fn is_void(&self) -> bool {
        match self {
            Space::Void => true,
            Space::Free => false,
            Space::Wall => false,
        }
    }
}

#[derive(Debug)]
enum Instruction {
    Move(usize),
    TurnLeft,
    TurnRight,
}

fn main() {
    let mut aoc = Aoc::new()
        .year(Some(2022))
        .day(Some(22))
        .init()
        .unwrap();

    // Get input data (don't force)
    let input = aoc.get_input(false).unwrap();
    // let input = "        ...#    \n        .#..    \n        #...    \n        ....    \n...#.......#    \n........#...    \n..#....#....    \n..........#.    \n        ...#....\n        .....#..\n        .#......\n        ......#.\n\n10R5L5R10L4R5L5\n";

    let mut parts = input.split("\n\n");
    let map_str = parts.next().unwrap();
    let instructions_str = parts.next().unwrap();

    let mut map = Vec::new();
    let mut map_width = 0;
    for line in map_str.lines() {
        let mut row = Vec::new();
        for char in line.chars() {
            row.push(match char {
                ' ' => Space::Void,
                '.' => Space::Free,
                '#' => Space::Wall,
                _ => unreachable!(),
            });
        }
        map_width = map_width.max(row.len());
        while row.len() < map_width {
            row.push(Space::Void);
        }
        map.push(row);
    }
    let map_height = map.len();

    let mut instructions = Vec::new();
    for part in instructions_str.split_inclusive(&['L', 'R']) {
        if part.ends_with('L') {
            instructions.push(Instruction::Move(part.strip_suffix('L').unwrap().parse().unwrap()));
            instructions.push(Instruction::TurnLeft);
        } else if part.ends_with('R') {
            instructions.push(Instruction::Move(part.strip_suffix('R').unwrap().parse().unwrap()));
            instructions.push(Instruction::TurnRight);
        } else {
            instructions.push(Instruction::Move(part.trim().parse().unwrap()));
        }
    }

    {
        let mut start_position = IVec2::new(0, 0);
        for x in 0..map[0].len() {
            if let Space::Free = map[0][x] {
                start_position.x = x as i32;
                break;
            }
        }

        let mut current_position = start_position;
        let mut current_facing = IVec2::X;
        for instruction in instructions.iter() {
            match instruction {
                Instruction::Move(value) => {
                    for _ in 0..*value {
                        let mut next_position = current_position + current_facing;
                        next_position.x = next_position.x.rem_euclid(map_width as i32);
                        next_position.y = next_position.y.rem_euclid(map_height as i32);

                        if current_facing == IVec2::NEG_X && map[next_position.y as usize][next_position.x as usize].is_void() {
                            next_position.x = map[current_position.y as usize].iter().rposition(|space| !space.is_void()).unwrap() as i32;
                        } else if current_facing == IVec2::X && map[next_position.y as usize][next_position.x as usize].is_void() {
                            next_position.x = map[current_position.y as usize].iter().position(|space| !space.is_void()).unwrap() as i32;
                        } else if current_facing == IVec2::NEG_Y && map[next_position.y as usize][next_position.x as usize].is_void() {
                            next_position.y = map.iter().rposition(|row| !row[next_position.x as usize].is_void()).unwrap() as i32;
                        } else if current_facing == IVec2::Y && map[next_position.y as usize][next_position.x as usize].is_void() {
                            next_position.y = map.iter().position(|row| !row[next_position.x as usize].is_void()).unwrap() as i32;
                        }

                        assert!(next_position.x >= 0);
                        assert!(next_position.y >= 0);
                        if let Space::Wall = map[next_position.y as usize][next_position.x as usize] {
                            break;
                        } else {
                            assert!(map[next_position.y as usize][next_position.x as usize].is_free());
                            current_position = next_position;
                        }
                    }
                }
                Instruction::TurnLeft => {
                    current_facing = match current_facing {
                        IVec2::X => IVec2::NEG_Y,
                        IVec2::Y => IVec2::X,
                        IVec2::NEG_X => IVec2::Y,
                        IVec2::NEG_Y => IVec2::NEG_X,
                        _ => unreachable!(),
                    };
                }
                Instruction::TurnRight => {
                    current_facing = match current_facing {
                        IVec2::X => IVec2::Y,
                        IVec2::Y => IVec2::NEG_X,
                        IVec2::NEG_X => IVec2::NEG_Y,
                        IVec2::NEG_Y => IVec2::X,
                        _ => unreachable!(),
                    };
                }
            }
        }

        let direction = match current_facing {
            IVec2::X => 0,
            IVec2::Y => 1,
            IVec2::NEG_X => 2,
            IVec2::NEG_Y => 3,
            _ => unreachable!(),
        };

        println!("{}", (current_position.y + 1) * 1000 + (current_position.x + 1) * 4 + direction);
    }

    {
        let mut faces = Vec::new();
        let mut faces_offset = Vec::new();
        for y in (0..map_height).step_by(50) {
            for x in (0..map_width).step_by(50) {
                if !map[y][x].is_void() {
                    let mut face = Vec::new();
                    for yy in 0..50 {
                        let mut row = Vec::new();
                        for xx in 0..50 {
                            row.push(map[y+yy][x+xx]);
                        }
                        face.push(row);
                    }
                    faces.push(face);
                    faces_offset.push(IVec2::new(x as i32, y as i32));
                }
            }
        }

        let mut current_position = IVec2::new(0, 0);
        let mut current_face = 0;
        let mut current_facing = IVec2::X;

        for instruction in instructions.iter() {
            match instruction {
                Instruction::Move(value) => {
                    for _ in 0..*value {
                        let next_position = current_position + current_facing;
                        if next_position.x < 0 || next_position.x >= 50 || next_position.y < 0 || next_position.y >= 50 {
                            let (x, y) = current_position.into();
                            let (new_direction, new_face, new_x, new_y) = match (current_face, current_facing) {
                                (0, IVec2::X) => (IVec2::X, 1, 0, y),
                                (0, IVec2::Y) => (IVec2::Y, 2, x, 0),
                                (0, IVec2::NEG_X) => (IVec2::X, 3, 0, 49 - y),
                                (0, IVec2::NEG_Y) => (IVec2::X, 5, 0, x),
                                (1, IVec2::X) => (IVec2::NEG_X, 4, 49, 49 - y),
                                (1, IVec2::Y) => (IVec2::NEG_X, 2, 49, x),
                                (1, IVec2::NEG_X) => (IVec2::NEG_X, 0, 49, y),
                                (1, IVec2::NEG_Y) => (IVec2::NEG_Y, 5, x, 49),
                                (2, IVec2::X) => (IVec2::NEG_Y, 1, y, 49),
                                (2, IVec2::Y) => (IVec2::Y, 4, x, 0),
                                (2, IVec2::NEG_X) => (IVec2::Y, 3, y, 0),
                                (2, IVec2::NEG_Y) => (IVec2::NEG_Y, 0, x, 49),
                                (3, IVec2::X) => (IVec2::X, 4, 0, y),
                                (3, IVec2::Y) => (IVec2::Y, 5, x, 0),
                                (3, IVec2::NEG_X) => (IVec2::X, 0, 0, 49 - y),
                                (3, IVec2::NEG_Y) => (IVec2::X, 2, 0, x),
                                (4, IVec2::X) => (IVec2::NEG_X, 1, 49, 49 - y),
                                (4, IVec2::Y) => (IVec2::NEG_X, 5, 49, x),
                                (4, IVec2::NEG_X) => (IVec2::NEG_X, 3, 49, y),
                                (4, IVec2::NEG_Y) => (IVec2::NEG_Y, 2, x, 49),
                                (5, IVec2::X) => (IVec2::NEG_Y, 4, y, 49),
                                (5, IVec2::Y) => (IVec2::Y, 1, x, 0),
                                (5, IVec2::NEG_X) => (IVec2::Y, 0, y, 0),
                                (5, IVec2::NEG_Y) => (IVec2::NEG_Y, 3, x, 49),
                                _ => unreachable!(),
                            };

                            if let Space::Wall = faces[new_face][new_y as usize][new_x as usize] {
                                break;
                            } else {
                                current_position = IVec2::new(new_x, new_y);
                                current_face = new_face;
                                current_facing = new_direction;
                            }
                        } else {
                            if let Space::Wall = faces[current_face][next_position.y as usize][next_position.x as usize] {
                                break;
                            } else {
                                assert!(faces[current_face][next_position.y as usize][next_position.x as usize].is_free());
                                current_position = next_position;
                            }
                        }
                    }
                }
                Instruction::TurnLeft => {
                    current_facing = match current_facing {
                        IVec2::X => IVec2::NEG_Y,
                        IVec2::Y => IVec2::X,
                        IVec2::NEG_X => IVec2::Y,
                        IVec2::NEG_Y => IVec2::NEG_X,
                        _ => unreachable!(),
                    };
                }
                Instruction::TurnRight => {
                    current_facing = match current_facing {
                        IVec2::X => IVec2::Y,
                        IVec2::Y => IVec2::NEG_X,
                        IVec2::NEG_X => IVec2::NEG_Y,
                        IVec2::NEG_Y => IVec2::X,
                        _ => unreachable!(),
                    };
                }
            }
        }

        let direction = match current_facing {
            IVec2::X => 0,
            IVec2::Y => 1,
            IVec2::NEG_X => 2,
            IVec2::NEG_Y => 3,
            _ => unreachable!(),
        };

        let pos = current_position + faces_offset[current_face];
        println!("{}", (pos.y + 1) * 1000 + (pos.x + 1) * 4 + direction);
    }

}
