use aocf::Aoc;
use regex::Regex;
use std::{collections::HashSet, ops::Sub};

fn main() {
    let mut aoc = Aoc::new()
        .year(Some(2022))
        .day(Some(15))
        .init()
        .unwrap();

    // Get input data (don't force)
    let input = aoc.get_input(false).unwrap();
    let target_line = 2000000;
    let input = "Sensor at x=2, y=18: closest beacon is at x=-2, y=15\nSensor at x=9, y=16: closest beacon is at x=10, y=16\nSensor at x=13, y=2: closest beacon is at x=15, y=3\nSensor at x=12, y=14: closest beacon is at x=10, y=16\nSensor at x=10, y=20: closest beacon is at x=10, y=16\nSensor at x=14, y=17: closest beacon is at x=10, y=16\nSensor at x=8, y=7: closest beacon is at x=2, y=10\nSensor at x=2, y=0: closest beacon is at x=2, y=10\nSensor at x=0, y=11: closest beacon is at x=2, y=10\nSensor at x=20, y=14: closest beacon is at x=25, y=17\nSensor at x=17, y=20: closest beacon is at x=21, y=22\nSensor at x=16, y=7: closest beacon is at x=15, y=3\nSensor at x=14, y=3: closest beacon is at x=15, y=3\nSensor at x=20, y=1: closest beacon is at x=15, y=3";
    let target_line = 10;

    let re = Regex::new(r"^Sensor at x=(-?\d+), y=(-?\d+): closest beacon is at x=(-?\d+), y=(-?\d+)$").unwrap();
    let mut set = HashSet::new();
    let mut beacons_on_line = HashSet::new();

    let mut sensors = Vec::new();

    for line in input.lines() {
        let captures = re.captures(line).unwrap();
        let sensor_x = captures[1].parse::<isize>().unwrap();
        let sensor_y = captures[2].parse::<isize>().unwrap();
        let beacon_x = captures[3].parse::<isize>().unwrap();
        let beacon_y = captures[4].parse::<isize>().unwrap();

        if beacon_y == target_line {
            beacons_on_line.insert(beacon_x);
        }

        let distance = sensor_x.abs_diff(beacon_x) + sensor_y.abs_diff(beacon_y);
        sensors.push((sensor_x, sensor_y, distance));

        let height_diff =  sensor_y.abs_diff(target_line);
        if height_diff <= distance {
            let left = (distance - height_diff) as isize;

            for x in (sensor_x - left)..=(sensor_x + left) {
                set.insert(x);
            }
        }
    }

    println!("{}", set.sub(&beacons_on_line).len());

    // for y in 0..=4000000isize {
    //     for x in 0..=4000000isize {
    //         let mut sensor_nearby = false;
    //         for &(sx, sy, sd) in sensors.iter() {
    //             let distance = x.abs_diff(sx) + y.abs_diff(sy);
    //             if distance <= sd {
    //                 sensor_nearby = true;
    //                 break;
    //             }
    //         }
    //         if !sensor_nearby {
    //             println!("{} {}", x, y);
    //         }
    //     }
    // }

    for &(ax, ay, ad) in sensors.iter() {
        for &(bx, by, bd) in sensors.iter() {
            let dist = ax.abs_diff(bx) + ay.abs_diff(by);
            if dist == ad + bd + 2 {
                // println!("{} {} {} {}", ax, ay, bx, by);

                let dx = ax - bx;
                let dy = ay - by;
                let px = bx + dx/2;
                let py = by + dy/2;

                let mut overlap = false;
                for &(cx, cy, cd) in sensors.iter() {
                    let dist = cx.abs_diff(px) + cy.abs_diff(py);
                    if dist <= cd {
                        overlap = true;
                        break;
                    }
                }
                if !overlap {
                    println!("{} {}", px, py);
                }
            }
        }
    }
}
