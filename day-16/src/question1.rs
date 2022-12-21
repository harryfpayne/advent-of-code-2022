use std::collections::HashSet;
use crate::valve::{get_distance_matrix, parse_input, State, Valve};

pub fn search_q1(
    valves: &Vec<Valve>,
    distances: &Vec<Vec<i32>>,
    valves_worth_visiting: HashSet<usize>,
    current: usize,
    turns_left: i32,
    score: i32
) -> i32 {
    let mut best_score = score.clone();
    for _next_valve in valves_worth_visiting.iter() {
        let next_valve = *_next_valve;
        let turns_to_activate = distances.get(current).unwrap().get(next_valve).unwrap() + 1;
        if turns_to_activate > turns_left {
            continue;
        }

        let turns_left_after_activate = turns_left - turns_to_activate;
        let score_after_activate = score + (turns_left_after_activate * valves.get(next_valve).unwrap().flow_rate);
        let mut next_valves_worth_visiting = valves_worth_visiting.clone();
        next_valves_worth_visiting.remove(_next_valve);
        let next_score = search_q1(
            valves,
            distances,
            next_valves_worth_visiting,
            next_valve,
            turns_left_after_activate,
            score_after_activate
        );
        best_score = best_score.max(next_score);
    }

    best_score
}

pub fn question_1(input: &str) -> i32 {
    let (valves, start) = parse_input(input);
    let distances = get_distance_matrix(&valves);
    let valves_worth_visiting = valves
        .iter()
        .enumerate()
        .filter(|(i, v)| v.flow_rate > 0)
        .map(|(i,_)| i)
        .collect::<HashSet<usize>>();

    search_q1(&valves, &distances, valves_worth_visiting, start, 30, 0)
}
