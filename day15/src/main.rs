use aocf::Aoc;
use regex::Regex;
use std::collections::VecDeque;

fn main() {
    let mut aoc = Aoc::new()
        .year(Some(2022))
        .day(Some(15))
        .init()
        .unwrap();

    // Get input data (don't force)
    let input = aoc.get_input(false).unwrap();
    let target_line = 2000000;
    let range_max = 4000000;
    // let input = "Sensor at x=2, y=18: closest beacon is at x=-2, y=15\nSensor at x=9, y=16: closest beacon is at x=10, y=16\nSensor at x=13, y=2: closest beacon is at x=15, y=3\nSensor at x=12, y=14: closest beacon is at x=10, y=16\nSensor at x=10, y=20: closest beacon is at x=10, y=16\nSensor at x=14, y=17: closest beacon is at x=10, y=16\nSensor at x=8, y=7: closest beacon is at x=2, y=10\nSensor at x=2, y=0: closest beacon is at x=2, y=10\nSensor at x=0, y=11: closest beacon is at x=2, y=10\nSensor at x=20, y=14: closest beacon is at x=25, y=17\nSensor at x=17, y=20: closest beacon is at x=21, y=22\nSensor at x=16, y=7: closest beacon is at x=15, y=3\nSensor at x=14, y=3: closest beacon is at x=15, y=3\nSensor at x=20, y=1: closest beacon is at x=15, y=3";
    // let target_line = 10;
    // let range_max = 20;

    let re = Regex::new(r"^Sensor at x=(-?\d+), y=(-?\d+): closest beacon is at x=(-?\d+), y=(-?\d+)$").unwrap();

    let mut sensors = Vec::new();
    let mut beacons = Vec::new();

    for line in input.lines() {
        let captures = re.captures(line).unwrap();
        let sensor_x = captures[1].parse::<isize>().unwrap();
        let sensor_y = captures[2].parse::<isize>().unwrap();
        let beacon_x = captures[3].parse::<isize>().unwrap();
        let beacon_y = captures[4].parse::<isize>().unwrap();

        let distance = sensor_x.abs_diff(beacon_x) + sensor_y.abs_diff(beacon_y);
        sensors.push((sensor_x, sensor_y, distance));
        beacons.push((beacon_x, beacon_y));
    }

    for y in 0..=range_max {
        let mut ranges = Vec::new();
        for &(sensor_x, sensor_y, sensor_dist) in sensors.iter() {
            let height_diff =  sensor_y.abs_diff(y);
            if height_diff <= sensor_dist {
                let left = (sensor_dist - height_diff) as isize;
                ranges.push((sensor_x - left)..=(sensor_x + left));
            }
        }

        ranges.sort_by(|a, b| {
            if a.start() == b.start() {
                a.end().cmp(b.end())
            } else {
                a.start().cmp(b.start())
            }
        });
        let mut ranges = VecDeque::from(ranges);
        let mut ranges_done = Vec::new();

        // Merge the ranges
        loop {
            let a = ranges.pop_front().unwrap();
            if let Some(b) = ranges.pop_front() {
                if a.start() < b.start() && b.end() < a.end() {
                    // a contains b
                    ranges.push_front(a);
                } else if b.start() < a.start() && a.end() < b.end() {
                    // b contains a
                    ranges.push_front(b);
                } else if (a.end() + 1) >= *b.start() {
                    // a and b overlap
                    ranges.push_front(*a.start()..=*b.end());
                } else {
                    // no overlap
                    ranges_done.push(a);
                    ranges.push_front(b);
                }
            } else {
                ranges_done.push(a);
                break;
            }
        }

        if y == target_line {
            assert_eq!(ranges_done.len(), 1);
            println!("{:?}", ranges_done[0].end() - ranges_done[0].start());
        }

        if ranges_done.len() != 1 {
            assert_eq!(ranges_done.len(), 2);
            let x = *ranges_done[0].end() + 1;
            println!("{}", x * 4000000 + y);
        }
    }
}
