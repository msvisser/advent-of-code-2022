use aocf::Aoc;
use glam::IVec3;
use std::collections::HashSet;

fn flood_fill(pos: IVec3, visited: &mut HashSet<IVec3>, cubes: &HashSet<IVec3>, min_bound: IVec3, max_bound: IVec3) {
    visited.insert(pos);
    for dir in [IVec3::X, IVec3::Y, IVec3::Z, IVec3::NEG_X, IVec3::NEG_Y, IVec3::NEG_Z] {
        let new_pos = pos + dir;
        if new_pos.cmplt(min_bound).any() || new_pos.cmpgt(max_bound).any() {
            continue;
        }
        if !visited.contains(&new_pos) && !cubes.contains(&new_pos) {
            flood_fill(new_pos, visited, cubes, min_bound, max_bound);
        }
    }
}

fn main() {
    let mut aoc = Aoc::new()
        .year(Some(2022))
        .day(Some(18))
        .init()
        .unwrap();

    // Get input data (don't force)
    let input = aoc.get_input(false).unwrap();
    // let input = "2,2,2\n1,2,2\n3,2,2\n2,1,2\n2,3,2\n2,2,1\n2,2,3\n2,2,4\n2,2,6\n1,2,5\n3,2,5\n2,1,5\n2,3,5";

    let mut surface_area = 0;
    let mut cubes = HashSet::new();

    for line in input.lines() {
        let mut parts = line.split(',');
        let cube = IVec3 {
            x: parts.next().unwrap().parse().unwrap(),
            y: parts.next().unwrap().parse().unwrap(),
            z: parts.next().unwrap().parse().unwrap(),
        };

        cubes.insert(cube);
        surface_area += 6;
        for dir in [IVec3::X, IVec3::Y, IVec3::Z, IVec3::NEG_X, IVec3::NEG_Y, IVec3::NEG_Z] {
            if cubes.contains(&(cube + dir)) {
                surface_area -= 2;
            }
        }
    }

    println!("{}", surface_area);

    let mut min_pos = IVec3::new(99, 99, 99);
    let mut max_pos = IVec3::ZERO;
    for &cube in cubes.iter() {
        min_pos = min_pos.min(cube);
        max_pos = max_pos.max(cube);
    }
    min_pos -= IVec3::ONE;
    max_pos += IVec3::ONE;

    assert!(!cubes.contains(&min_pos));
    let mut visited = HashSet::new();
    flood_fill(min_pos, &mut visited, &cubes, min_pos, max_pos);

    let outside_plus_cubes: HashSet<_> = cubes.union(&visited).collect();

    let mut new_surface_area = surface_area;
    let mut new_cubes = cubes.clone();
    for z in min_pos.z..=max_pos.z {
        for y in min_pos.y..=max_pos.y {
            for x in min_pos.x..=max_pos.x {
                let pos = IVec3::new(x, y, z);

                if !outside_plus_cubes.contains(&pos) {
                    new_cubes.insert(pos);
                    new_surface_area += 6;
                    for dir in [IVec3::X, IVec3::Y, IVec3::Z, IVec3::NEG_X, IVec3::NEG_Y, IVec3::NEG_Z] {
                        if new_cubes.contains(&(pos + dir)) {
                            new_surface_area -= 2;
                        }
                    }
                }
            }
        }
    }

    println!("{}", new_surface_area);
}
