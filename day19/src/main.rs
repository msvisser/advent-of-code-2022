#![feature(int_roundings)]

use aocf::Aoc;
use regex::Regex;

#[derive(Debug)]
struct Blueprint {
    id: usize,
    ore_robot_ore_cost: usize,
    clay_robot_ore_cost: usize,
    obsidian_robot_ore_cost: usize,
    obsidian_robot_clay_cost: usize,
    geode_robot_ore_cost: usize,
    geode_robot_obsidian_cost: usize,
}

#[derive(Clone, Copy, Debug)]
struct State {
    ore_robots: usize,
    clay_robots: usize,
    obsidian_robots: usize,
    geode_robots: usize,

    ore: usize,
    clay: usize,
    obsidian: usize,
    geode: usize,

    minutes: usize,
}

impl State {
    fn wait_minutes(&mut self, minutes: usize) {
        self.ore += self.ore_robots * minutes;
        self.clay += self.clay_robots * minutes;
        self.obsidian += self.obsidian_robots * minutes;
        self.geode += self.geode_robots * minutes;
        self.minutes -= minutes;
    }
}

fn main() {
    let mut aoc = Aoc::new()
        .year(Some(2022))
        .day(Some(19))
        .init()
        .unwrap();

    // Get input data (don't force)
    // let input = aoc.get_input(false).unwrap();
    let input = "Blueprint 1: Each ore robot costs 4 ore. Each clay robot costs 2 ore. Each obsidian robot costs 3 ore and 14 clay. Each geode robot costs 2 ore and 7 obsidian.";

    let regex = Regex::new(r"Blueprint (\d+): Each ore robot costs (\d+) ore\. Each clay robot costs (\d+) ore\. Each obsidian robot costs (\d+) ore and (\d+) clay\. Each geode robot costs (\d+) ore and (\d+) obsidian\.").unwrap();
    for line in input.lines() {
        let captures = regex.captures(line).unwrap();
        let blueprint = Blueprint {
            id: captures[1].parse().unwrap(),
            ore_robot_ore_cost: captures[2].parse().unwrap(),
            clay_robot_ore_cost: captures[3].parse().unwrap(),
            obsidian_robot_ore_cost: captures[4].parse().unwrap(),
            obsidian_robot_clay_cost: captures[5].parse().unwrap(),
            geode_robot_ore_cost: captures[6].parse().unwrap(),
            geode_robot_obsidian_cost: captures[7].parse().unwrap(),
        };

        println!("{:?}", blueprint);

        let state = State {
            ore_robots: 1,
            clay_robots: 0,
            obsidian_robots: 0,
            geode_robots: 0,

            ore: 0,
            clay: 0,
            obsidian: 0,
            geode: 0,

            minutes: 24,
        };

        let result = search(state, &blueprint);
        println!("{}", result);
    }
}

fn search(state: State, blueprint: &Blueprint) -> usize {
    let mut max_geodes = 0;

    if state.ore_robots > 0 {
        let minutes_until_ore_robot = (blueprint.ore_robot_ore_cost.saturating_sub(state.ore)).div_ceil(state.ore_robots);
        if state.minutes >= minutes_until_ore_robot + 1 {
            let mut sub_state = state;
            sub_state.wait_minutes(minutes_until_ore_robot + 1);
            sub_state.ore -= blueprint.ore_robot_ore_cost;
            sub_state.ore_robots += 1;
            max_geodes = max_geodes.max(search(sub_state, blueprint));
        }

        let minutes_until_clay_robot = (blueprint.clay_robot_ore_cost.saturating_sub(state.ore)).div_ceil(state.ore_robots);
        if state.minutes >= minutes_until_clay_robot + 1 {
            let mut sub_state = state;
            sub_state.wait_minutes(minutes_until_clay_robot + 1);
            sub_state.ore -= blueprint.clay_robot_ore_cost;
            sub_state.clay_robots += 1;
            max_geodes = max_geodes.max(search(sub_state, blueprint));
        }
    }

    if state.ore_robots > 0 && state.clay_robots > 0 {
        let minutes_until_obsidian_robot_ore = (blueprint.obsidian_robot_ore_cost.saturating_sub(state.ore)).div_ceil(state.ore_robots);
        let minutes_until_obsidian_robot_clay = (blueprint.obsidian_robot_clay_cost.saturating_sub(state.clay)).div_ceil(state.clay_robots);
        let minutes_until_obsidian_robot = minutes_until_obsidian_robot_ore.max(minutes_until_obsidian_robot_clay);
        if state.minutes >= minutes_until_obsidian_robot + 1 {
            let mut sub_state = state;
            sub_state.wait_minutes(minutes_until_obsidian_robot);
            sub_state.ore -= blueprint.obsidian_robot_ore_cost;
            sub_state.clay -= blueprint.obsidian_robot_clay_cost;
            sub_state.obsidian_robots += 1;
            max_geodes = max_geodes.max(search(sub_state, blueprint));
        }
    }

    if state.ore_robots > 0 && state.obsidian_robots > 0 {
        let minutes_until_geode_robot_ore = (blueprint.geode_robot_ore_cost.saturating_sub(state.ore)).div_ceil(state.ore_robots);
        let minutes_until_geode_robot_obsidian = (blueprint.geode_robot_obsidian_cost.saturating_sub(state.obsidian)).div_ceil(state.obsidian_robots);
        let minutes_until_geode_robot = minutes_until_geode_robot_ore.max(minutes_until_geode_robot_obsidian);
        if state.minutes >= minutes_until_geode_robot + 1 {
            let mut sub_state = state;
            sub_state.wait_minutes(minutes_until_geode_robot + 1);
            sub_state.ore -= blueprint.geode_robot_ore_cost;
            sub_state.obsidian -= blueprint.geode_robot_obsidian_cost;
            sub_state.geode_robots += 1;
            max_geodes = max_geodes.max(search(sub_state, blueprint));
        }
    }

    if state.minutes > 0 {
        let mut sub_state = state;
        sub_state.ore += sub_state.ore_robots;
        sub_state.clay += sub_state.clay_robots;
        sub_state.obsidian += sub_state.obsidian_robots;
        sub_state.geode += sub_state.geode_robots;
        sub_state.minutes -= 1;
        max_geodes = max_geodes.max(search(sub_state, blueprint));
    } else {
        max_geodes = max_geodes.max(state.geode);
    }

    max_geodes
}
