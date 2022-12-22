use aocf::Aoc;
use glam::IVec2;

#[derive(Debug)]
enum Jet {
    Left,
    Right,
}

#[derive(Debug)]
struct Rock<'a> {
    points: &'a Vec<IVec2>,
    pos: IVec2,
}

#[derive(Clone, Debug, PartialEq)]
struct State {
    current_rock: usize,
    current_jet: usize,
    heights: [Option<usize>; 7],
}

impl<'a> Rock<'a> {
    fn try_move(&mut self, dir: IVec2, field: &Vec<[bool; 7]>) -> bool {
        for &point in self.points {
            let new_point_pos = point + self.pos + dir;
            if new_point_pos.x < 0 || new_point_pos.x >= 7 || new_point_pos.y < 0 {
                return false;
            }
            if field[new_point_pos.y as usize][new_point_pos.x as usize] {
                return false;
            }
        }

        self.pos += dir;
        true
    }
}

fn sim_single_rock(state: &mut State, field: &mut Vec<[bool; 7]>, rock_points: &Vec<Vec<IVec2>>, jets: &Vec<Jet>) {
    let highest_rock = field.iter()
        .position(|&row| !row.iter().any(|&b| b))
        .unwrap_or(0);

    while field.len() < highest_rock + 3 + 4 {
        field.push([false; 7]);
    }

    let mut rock = Rock {
        points: &rock_points[state.current_rock],
        pos: IVec2 { x: 2, y: highest_rock as i32 + 3 },
    };
    state.current_rock = (state.current_rock + 1) % rock_points.len();

    loop {
        let jet = &jets[state.current_jet];
        state.current_jet = (state.current_jet + 1) % jets.len();
        match jet {
            Jet::Left => {
                rock.try_move(IVec2::NEG_X, &field);
            }
            Jet::Right => {
                rock.try_move(IVec2::X, &field);
            }
        }

        if !rock.try_move(IVec2::NEG_Y, &field) {
            break;
        }
    }

    for &point in rock.points {
        let point_pos = point + rock.pos;
        field[point_pos.y as usize][point_pos.x as usize] = true;
    }
}

fn sim(iterations: usize, field: &mut Vec<[bool; 7]>, rock_points: &Vec<Vec<IVec2>>, jets: &Vec<Jet>) -> usize {
    let mut state = State {
        current_rock: 0,
        current_jet: 0,
        heights: [None; 7],
    };

    for _ in 0..iterations {
        sim_single_rock(&mut state, field, rock_points, jets);
    }

    let highest_rock = field.iter()
            .position(|row| !row.iter().any(|&b| b))
            .unwrap();
    highest_rock
}

fn sim2(rock_points: &Vec<Vec<IVec2>>, jets: &Vec<Jet>) {
    let mut field: Vec<[bool; 7]> = Vec::new();
    let mut state = State {
        current_rock: 0,
        current_jet: 0,
        heights: [None; 7],
    };

    let mut seen_states: Vec<State> = Vec::new();
    let mut seen_highest_rock: Vec<usize> = Vec::new();

    let mut rock_index = 0;
    loop {
        sim_single_rock(&mut state, &mut field, rock_points, jets);

        let highest_rock = field.iter()
            .position(|row| !row.iter().any(|&b| b))
            .unwrap();

        for i in 0..7 {
            state.heights[i] = field.iter().rposition(|row| row[i]).map(|p| (highest_rock-1) - p);
        }

        if let Some(index) = seen_states.iter().position(|search_state| *search_state == state) {
            let loop_steps = rock_index - index;
            let loop_growth = highest_rock - seen_highest_rock[index];

            let steps_to_go = 1000000000000 - rock_index;
            let repeats = steps_to_go / loop_steps;
            let steps_to_sim = steps_to_go % loop_steps;

            println!("{} {}", loop_steps, loop_growth);

            for _ in 0..steps_to_sim {
                sim_single_rock(&mut state, &mut field, rock_points, jets);
            }
            let new_highest = field.iter()
                .position(|row| !row.iter().any(|&b| b))
                .unwrap();

            println!("{}", (new_highest-1) + repeats * loop_growth);

            break;
        }

        seen_states.push(state.clone());
        seen_highest_rock.push(highest_rock);
        rock_index += 1;
    }
}

fn main() {
    let mut aoc = Aoc::new()
        .year(Some(2022))
        .day(Some(17))
        .init()
        .unwrap();

    // Get input data (don't force)
    let input = aoc.get_input(false).unwrap();
    // let input = ">>><<><>><<<>><>>><<<>>><<<><<<>><>><<>>";

    let mut jets = Vec::new();
    for char in input.trim().chars() {
        let dir = match char {
            '<' => Jet::Left,
            '>' => Jet::Right,
            _ => unimplemented!(),
        };
        jets.push(dir);
    }

    let rock_points = vec![
        vec![IVec2::new(0, 0), IVec2::new(1, 0), IVec2::new(2, 0), IVec2::new(3, 0)],
        vec![IVec2::new(1, 0), IVec2::new(0, 1), IVec2::new(1, 1), IVec2::new(2, 1), IVec2::new(1, 2)],
        vec![IVec2::new(0, 0), IVec2::new(1, 0), IVec2::new(2, 0), IVec2::new(2, 1), IVec2::new(2, 2)],
        vec![IVec2::new(0, 0), IVec2::new(0, 1), IVec2::new(0, 2), IVec2::new(0, 3)],
        vec![IVec2::new(0, 0), IVec2::new(1, 0), IVec2::new(0, 1), IVec2::new(1, 1)],
    ];

    let mut field: Vec<[bool; 7]> = Vec::new();
    let highest_rock = sim(2022, &mut field, &rock_points, &jets);
    println!("{}", highest_rock);

    sim2(&rock_points, &jets);
}
