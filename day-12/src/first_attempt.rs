use std::collections::HashSet;
use crate::map_operations::{Coord, distance, get_possible_moves, map_get, parse_input};

pub fn path_find(map: &Vec<Vec<i32>>, curr_position: Coord, end: &Coord, previous_positions: Vec<Coord>, dead_positions: HashSet<Coord>) -> (Option<Vec<Coord>>, HashSet<Coord>) {
    let log = false;
    if previous_positions.len() > 1000 {
        if log {println!("My path is too long");};
        return (None, dead_positions);
    }
    if log {println!("------------------------------------------------");}
    let adjacent_positions = get_possible_moves(curr_position, map.len(), map[0].len());
    let current_height = map_get(map, &curr_position);

    if log {println!("I'm at {:?}, I have {} adjacent positions", curr_position, adjacent_positions.len());};
    let mut allowed_moves: Vec<Coord> = vec![];
    for adjacent_position in adjacent_positions {
        let height = map_get(map, &adjacent_position);
        if height > &(current_height + 1) {
            continue
        }
        if height < &(current_height) {
            continue
        }

        if previous_positions.contains(&adjacent_position) {
            continue
        }

        if dead_positions.contains(&adjacent_position) {
            continue
        }

        allowed_moves.push(adjacent_position);
    }
    allowed_moves.sort_by(|a: &Coord, b: &Coord| {
        let ah = map_get(map, a);
        let bh = map_get(map, b);
        let a_dis_to_e = distance(end, a);
        let b_dis_to_e = distance(end, b);
        return (*bh as f32 + b_dis_to_e).total_cmp(&(*ah as f32 + a_dis_to_e));
    });

    let mut new_dead_positions = dead_positions.clone();
    if allowed_moves.len() == 0 {
        if log {println!("I'm in a dead end");};
        return (None, dead_positions);
    }
    if log {println!("I can move to {:?}", allowed_moves);};

    let mut best_path: Option<Vec<Coord>> = None;
    for new_position in allowed_moves {
        let mut next_moves = previous_positions.clone();
        next_moves.push(new_position);
        if &new_position == end {
            return (Some(next_moves), dead_positions);
        }

        let next_dead_positions = dead_positions.clone();
        let (res, dead) = path_find(map, new_position, end, next_moves, next_dead_positions);

        if let Some(path) = res {
            if best_path.is_none() || path.len() < best_path.as_deref().unwrap().len() {
                best_path = Some(path);
            }
        } else {
            new_dead_positions.extend(dead);
            new_dead_positions.insert(new_position);
        }
    }

    (best_path, dead_positions)
}