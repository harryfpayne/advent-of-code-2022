use std::cmp::Ordering;
use std::collections::{BinaryHeap, HashMap};
use crate::map_operations::{get_possible_moves, map_get, parse_input};

type Coord = (usize, usize);
struct Point {
    loc: Coord,
    distance: i32,
}

impl Eq for Point {}
impl PartialEq<Self> for Point {
    fn eq(&self, other: &Self) -> bool {
        self.distance.eq(&other.distance)
    }
}
impl PartialOrd<Self> for Point {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}
impl Ord for Point {
    fn cmp(&self, other: &Self) -> Ordering {
        other.distance.cmp(&self.distance)
    }
}

fn get_moves(map: &Vec<Vec<i32>>, point: Coord) -> Vec<Coord> {
    let adjacent_positions = get_possible_moves(point, map.len(), map[0].len());

    let current_height = map_get(map, &point);
    let mut allowed_moves: Vec<Coord> = vec![];
    for adjacent_position in adjacent_positions {
        let height = map_get(map, &adjacent_position);
        if current_height.abs_diff(*height) > 1 {
            continue
        }
        allowed_moves.push(adjacent_position)
    }

    allowed_moves
}

fn path_find(map: &Vec<Vec<i32>>, start: Coord, end: Coord, stop_on_1: bool) -> Option<i32> {
    let mut distance_map = HashMap::new();
    for y in 0..map.len() {
        for x in 0..map[y].len() {
            distance_map.insert((y,x), i32::MAX);
        }
    }

    let mut heap: BinaryHeap<Point> = BinaryHeap::new();
    heap.push(Point{ loc: start, distance: 0 });
    distance_map.insert(start, 0);

    let mut points_looked_at = 0;
    while let Some(Point{loc, distance}) = heap.pop() {
        points_looked_at += 1;
        if !stop_on_1 && loc == end {
            return Some(distance)
        }
        if stop_on_1 && map_get(map, &loc).eq(&1) {
            return Some(distance)
        }
        if distance > *distance_map.get(&loc).expect("distance doesn't exist in map") {
            continue
        }

        for adjacent_coord in get_moves(map, loc) {
            let new_dist = distance + 1;
            if new_dist < *distance_map.get(&adjacent_coord).expect("disn't doesn't exist in map") {
                heap.push(Point {
                    loc: adjacent_coord,
                    distance: new_dist,
                });
                distance_map.insert(adjacent_coord, new_dist);
            }
        }
    };

    None
}

pub fn question_1(input: &str) -> Option<i32> {
    let (map, start, end) = parse_input(input);
    path_find(&map, start, end, false)
}

pub fn question_2(input: &str) -> Option<i32> {
    let (map, start, end) = parse_input(input);
    path_find(&map, end, (0,0), true)
}
