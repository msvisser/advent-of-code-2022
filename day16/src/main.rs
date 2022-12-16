use aocf::Aoc;
use regex::Regex;
use std::collections::{HashMap, HashSet};
use pathfinding::prelude::bfs;

#[derive(Debug)]
struct SearchState {
    distances: HashMap<(String, String), usize>,
    valve_nodes: HashSet<String>,
    flow_map: HashMap<String, usize>,

    current_node: String,

    minutes_left: usize,
    pressure_released: usize,

    valves_opened: Vec<(String, usize)>,
}

fn search(state: &mut SearchState) -> (usize, Vec<(String, usize)>) {
    let valve_nodes = state.valve_nodes.clone();
    if valve_nodes.is_empty() || state.minutes_left == 0 {
        return (state.pressure_released, state.valves_opened.clone());
    }

    let temp_current_node = state.current_node.clone();
    let mut max_pressure_released = 0;
    let mut max_valves_opened = Vec::new();
    for node in valve_nodes.iter() {
        let dist = *state.distances.get(&(temp_current_node.clone(), node.clone())).unwrap();
        if state.minutes_left < dist + 1 {
            if state.pressure_released > max_pressure_released {
                max_pressure_released = state.pressure_released;
                max_valves_opened = state.valves_opened.clone();
            }
            continue;
        }

        state.minutes_left -= dist + 1;
        let pressure = state.minutes_left * state.flow_map.get(node).unwrap();
        state.pressure_released += pressure;

        state.valves_opened.push((node.clone(), state.minutes_left));

        state.valve_nodes.remove(node);
        state.current_node = node.clone();

        let (search_pressure, search_valves_opened) = search(state);
        if search_pressure > max_pressure_released {
            max_pressure_released = search_pressure;
            max_valves_opened = search_valves_opened;
        }

        state.current_node = temp_current_node.clone();
        state.valve_nodes.insert(node.clone());

        state.valves_opened.pop();

        state.pressure_released -= pressure;
        state.minutes_left += dist + 1;
    }
    (max_pressure_released, max_valves_opened)
}

fn main() {
    let mut aoc = Aoc::new()
        .year(Some(2022))
        .day(Some(16))
        .init()
        .unwrap();

    // Get input data (don't force)
    let input = aoc.get_input(false).unwrap();
    // let input = "Valve AA has flow rate=0; tunnels lead to valves DD, II, BB\nValve BB has flow rate=13; tunnels lead to valves CC, AA\nValve CC has flow rate=2; tunnels lead to valves DD, BB\nValve DD has flow rate=20; tunnels lead to valves CC, AA, EE\nValve EE has flow rate=3; tunnels lead to valves FF, DD\nValve FF has flow rate=0; tunnels lead to valves EE, GG\nValve GG has flow rate=0; tunnels lead to valves FF, HH\nValve HH has flow rate=22; tunnel leads to valve GG\nValve II has flow rate=0; tunnels lead to valves AA, JJ\nValve JJ has flow rate=21; tunnel leads to valve II";

    let mut graph_map = HashMap::new();
    let mut flow_map = HashMap::new();

    let re = Regex::new(r"^Valve ([A-Z]{2}) has flow rate=(\d+); tunnels? leads? to valves? (.*)$").unwrap();
    for line in input.lines() {
        let captures = re.captures(line).unwrap();

        let node = &captures[1];
        let flow = captures[2].parse::<usize>().unwrap();
        let targets: Vec<_> = captures[3].split(", ").map(|x| x.to_string()).collect();

        graph_map.insert(node.to_string(), targets);
        flow_map.insert(node.to_string(), flow);
    }

    let mut distances = HashMap::new();
    let mut valve_nodes = HashSet::new();
    for (node_a, &flow_a) in flow_map.iter() {
        if flow_a == 0 && node_a != "AA" {
            continue;
        }
        for (node_b, &flow_b) in flow_map.iter() {
            if flow_b == 0 {
                continue;
            }
            let distance = bfs(&node_a, |&p| graph_map.get(p).unwrap(), |&p| p == node_b).unwrap().len() - 1;
            distances.insert((node_a.clone(), node_b.clone()), distance);
        }
        if node_a != "AA" {
            valve_nodes.insert(node_a.clone());
        }
    }

    let mut state = SearchState {
        distances: distances.clone(),
        valve_nodes: valve_nodes.clone(),
        flow_map: flow_map.clone(),
        current_node: "AA".to_string(),
        minutes_left: 30,
        pressure_released: 0,
        valves_opened: Vec::new(),
    };
    let (pressure_released, _) = search(&mut state);
    println!("{}", pressure_released);

    let mut state = SearchState {
        distances,
        valve_nodes,
        flow_map,
        current_node: "AA".to_string(),
        minutes_left: 26,
        pressure_released: 0,
        valves_opened: Vec::new(),
    };
    let (pressure_released, valves_opened) = search(&mut state);

    for (valve, _) in valves_opened {
        state.valve_nodes.remove(&valve);
    }
    state.current_node = "AA".to_string();
    state.minutes_left = 26;
    state.pressure_released = pressure_released;

    let (pressure_released, _) = search(&mut state);
    println!("{}", pressure_released);

}
