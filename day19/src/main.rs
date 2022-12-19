use aocf::Aoc;
use regex::Regex;
use std::collections::{VecDeque, HashMap};
use rayon::prelude::*;

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

#[derive(Clone, Copy, Debug, Hash, PartialEq, Eq)]
struct State {
    ore_robots: usize,
    clay_robots: usize,
    obsidian_robots: usize,
    geode_robots: usize,

    ore: usize,
    clay: usize,
    obsidian: usize,
    geode: usize,
}

impl State {
    fn update_resources(&mut self) {
        self.ore += self.ore_robots;
        self.clay += self.clay_robots;
        self.obsidian += self.obsidian_robots;
        self.geode += self.geode_robots;
    }

    fn new() -> State {
        State {
            ore_robots: 1,
            clay_robots: 0,
            obsidian_robots: 0,
            geode_robots: 0,

            ore: 0,
            clay: 0,
            obsidian: 0,
            geode: 0,
        }
    }
}

fn main() {
    let mut aoc = Aoc::new()
        .year(Some(2022))
        .day(Some(19))
        .init()
        .unwrap();

    // Get input data (don't force)
    let input = aoc.get_input(false).unwrap();
    // let input = "Blueprint 1: Each ore robot costs 4 ore. Each clay robot costs 2 ore. Each obsidian robot costs 3 ore and 14 clay. Each geode robot costs 2 ore and 7 obsidian.\nBlueprint 2: Each ore robot costs 2 ore. Each clay robot costs 3 ore. Each obsidian robot costs 3 ore and 8 clay. Each geode robot costs 3 ore and 12 obsidian.";

    let mut blueprints = Vec::new();

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

        blueprints.push(blueprint);
    }

    let total: usize = blueprints.par_iter().map(|b| b.id * search(24, b)).sum();
    println!("{}", total);

    let total2: usize = blueprints.par_iter().take(3).map(|b| search(32, b)).product();
    println!("{}", total2);
}

fn search(minutes: usize, blueprint: &Blueprint) -> usize {
    let mut queue = VecDeque::new();
    queue.push_back((State::new(), minutes));

    let mut cache = HashMap::new();

    let mut max_geodes = 0;

    let max_ore_robots = blueprint.ore_robot_ore_cost.max(blueprint.clay_robot_ore_cost).max(blueprint.obsidian_robot_ore_cost).max(blueprint.geode_robot_ore_cost);
    let max_clay_robots = blueprint.obsidian_robot_clay_cost;
    let max_obsidian_robots = blueprint.geode_robot_obsidian_cost;

    while let Some((mut current_state, current_minutes)) = queue.pop_front() {
        current_state.ore = current_state.ore.min(current_minutes * max_ore_robots);
        current_state.clay = current_state.clay.min(current_minutes * max_clay_robots);
        current_state.obsidian = current_state.obsidian.min(current_minutes * max_obsidian_robots);

        if let Some(&cache_minutes) = cache.get(&current_state) {
            if cache_minutes >= current_minutes {
                continue;
            }
        }

        if current_minutes == 0 {
            max_geodes = max_geodes.max(current_state.geode);
            continue;
        }

        let can_make_geode_robot = current_state.ore >= blueprint.geode_robot_ore_cost && current_state.obsidian >= blueprint.geode_robot_obsidian_cost;
        let can_make_ore_robot = current_state.ore >= blueprint.ore_robot_ore_cost;
        let can_make_clay_robot = current_state.ore >= blueprint.clay_robot_ore_cost;
        let can_make_obsidian_robot = current_state.ore >= blueprint.obsidian_robot_ore_cost && current_state.clay >= blueprint.obsidian_robot_clay_cost;

        if can_make_geode_robot {
            let mut new_state = current_state;

            new_state.ore -= blueprint.geode_robot_ore_cost;
            new_state.obsidian -= blueprint.geode_robot_obsidian_cost;
            new_state.update_resources();
            new_state.geode_robots += 1;

            queue.push_back((new_state, current_minutes - 1));
            continue;
        }

        if can_make_ore_robot && current_state.ore_robots < max_ore_robots {
            let mut new_state = current_state;

            new_state.ore -= blueprint.ore_robot_ore_cost;
            new_state.update_resources();
            new_state.ore_robots += 1;

            queue.push_back((new_state, current_minutes - 1));
        }

        if can_make_clay_robot && current_state.clay_robots < max_clay_robots {
            let mut new_state = current_state;

            new_state.ore -= blueprint.clay_robot_ore_cost;
            new_state.update_resources();
            new_state.clay_robots += 1;

            queue.push_back((new_state, current_minutes - 1));
        }

        if can_make_obsidian_robot && current_state.obsidian_robots < max_obsidian_robots {
            let mut new_state = current_state;

            new_state.ore -= blueprint.obsidian_robot_ore_cost;
            new_state.clay -= blueprint.obsidian_robot_clay_cost;
            new_state.update_resources();
            new_state.obsidian_robots += 1;

            queue.push_back((new_state, current_minutes - 1));
        }

        {
            let mut new_state = current_state;
            new_state.update_resources();
            queue.push_back((new_state, current_minutes - 1));
        }

        cache.insert(current_state, current_minutes);
    }

    max_geodes
}
