use aocf::Aoc;
use pathfinding::prelude::astar;

#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
struct Pos {
    x: i32,
    y: i32,
}

impl Pos {
  fn distance(&self, other: &Pos) -> u32 {
    (self.x.abs_diff(other.x) + self.y.abs_diff(other.y)) as u32
  }

  fn successors(&self, height: &Vec<Vec<usize>>) -> Vec<(Pos, u32)> {
    let &Pos {
        x,
        y,
    } = self;
    let xu = x as usize;
    let yu = y as usize;

    let mut result = Vec::new();
    if xu > 0 && height[yu][xu-1] <= height[yu][xu]+1 {
        result.push((Pos { x: x - 1, y }, 1));
    }
    if xu < height[yu].len() - 1 && height[yu][xu+1] <= height[yu][xu]+1 {
        result.push((Pos { x: x + 1, y }, 1));
    }
    if yu > 0 && height[yu-1][xu] <= height[yu][xu]+1 {
        result.push((Pos { x, y: y - 1 }, 1));
    }
    if yu < height.len() - 1 && height[yu+1][xu] <= height[yu][xu]+1 {
        result.push((Pos { x, y: y + 1 }, 1));
    }

    result
  }
}

fn main() {
    let mut aoc = Aoc::new()
        .year(Some(2022))
        .day(Some(12))
        .init()
        .unwrap();

    // Get input data (don't force)
    let input = aoc.get_input(false).unwrap();
    // let input = "Sabqponm\nabcryxxl\naccszExk\nacctuvwj\nabdefghi\n";

    let mut height = Vec::new();
    let mut visited = Vec::new();
    let mut start_pos = (0, 0);
    let mut end_pos = (0, 0);

    for (y, line) in input.lines().enumerate() {
        let mut row = Vec::new();
        let mut row_visited = Vec::new();
        for (x, c) in line.chars().enumerate() {
            match c {
                'S' => {
                    start_pos = (x, y);
                    row.push(0);
                },
                'E' => {
                    end_pos = (x, y);
                    row.push(25);
                },
                'a'..='z' => row.push(c as usize - 'a' as usize),
                _ => unimplemented!(),
            }
            row_visited.push(false);
        }
        height.push(row);
        visited.push(row_visited);
    }

    let start_pos = Pos { x: start_pos.0 as i32, y: start_pos.1 as i32 };
    let end_pos = Pos { x: end_pos.0 as i32, y: end_pos.1 as i32 };

    let result = astar(
        &start_pos,
        |p: &Pos| p.successors(&height),
        |p| p.distance(&end_pos),
        |p| p == &end_pos
    );
    let path_len = result.unwrap().1;
    println!("{}", path_len);

    let mut smallest = path_len;
    for y in 0..height.len() {
        for x in 0..height[0].len() {
            if height[y][x] == 0 {
                let start_pos = Pos { x: x as i32, y: y as i32 };
                let result = astar(
                    &start_pos,
                    |p: &Pos| p.successors(&height),
                    |p| p.distance(&end_pos),
                    |p| p == &end_pos
                );
                if let Some((_, new_path_len)) = result {
                    if new_path_len < smallest {
                        smallest = new_path_len;
                    }
                }
            }
        }
    }
    println!("{}", smallest);
}
