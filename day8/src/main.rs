use aocf::Aoc;

fn tree_visible(map: &[[usize; 99]; 99], x: usize, y: usize) -> bool
{
    let my_value = map[y][x];

    let mut visible_top = true;
    for check_y in (0..y).rev() {
        if map[check_y][x] >= my_value {
            visible_top = false;
            break;
        }
    }

    let mut visible_bottom = true;
    for check_y in (y+1)..99  {
        if map[check_y][x] >= my_value {
            visible_bottom = false;
            break;
        }
    }

    let mut visible_left = true;
    for check_x in (0..x).rev() {
        if map[y][check_x] >= my_value {
            visible_left = false;
            break;
        }
    }

    let mut visible_right = true;
    for check_x in (x+1)..99 {
        if map[y][check_x] >= my_value {
            visible_right = false;
            break;
        }
    }

    visible_top || visible_bottom || visible_left || visible_right
}

fn calc_scenic_score<const N: usize>(map: &[[usize; N]; N], x: usize, y: usize) -> usize {
    let my_value = map[y][x];

    let mut view_top = 0;
    for check_y in (0..y).rev() {
        view_top += 1;
        if map[check_y][x] >= my_value {
            break;
        }
    }

    let mut view_bottom = 0;
    for check_y in (y+1)..N  {
        view_bottom += 1;
        if map[check_y][x] >= my_value {
            break;
        }
    }

    let mut view_left = 0;
    for check_x in (0..x).rev() {
        view_left += 1;
        if map[y][check_x] >= my_value {
            break;
        }
    }

    let mut view_right = 0;
    for check_x in (x+1)..N {
        view_right += 1;
        if map[y][check_x] >= my_value {
            break;
        }
    }

    view_top * view_bottom * view_left * view_right
}

fn main() {
    let mut aoc = Aoc::new()
        .year(Some(2022))
        .day(Some(8))
        .init()
        .unwrap();

    // Get input data (don't force)
    let input = aoc.get_input(false).unwrap();

    let mut map = [[0; 99]; 99];

    for (y, line) in input.lines().enumerate() {
        for (x, num) in line.chars().enumerate() {
            map[y][x] = num as usize - '0' as usize;
        }
    }

    let mut visible_trees = 0;
    for y in 0..map.len() {
        for x in 0..map[y].len() {
            if tree_visible(&map, x, y) {
                visible_trees += 1;
            }
        }
    }
    println!("{}", visible_trees);

    let mut max_scenic_score = 0;
    for y in 0..map.len() {
        for x in 0..map[y].len() {
            let scenic_score = calc_scenic_score(&map, x, y);
            if scenic_score > max_scenic_score {
                max_scenic_score = scenic_score;
            }
        }
    }
    println!("{}", max_scenic_score);

}
