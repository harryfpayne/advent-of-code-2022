use std::collections::{HashMap, HashSet};
use crate::parse::{parse_input, Cube, Axis, Coord};

fn count_along_axis(cube: &Cube, exposed_cache: &mut HashMap<Coord, bool>, axis: Axis) -> i32 {
    let mut surface_area = 0;
    for a in 0..cube.len()+1 {
        for b in 0..cube.len()+1 {
            let mut last = false;
            let mut last_coord = (0,0,0);
            for c in 0..cube.len()+1 {
                let d = axis.get_tuple(a,b,c);
                let val = if cube.len() <= a || cube.len() <= b || cube.len() <= c {
                    false
                } else {
                    cube[d.0][d.1][d.2]
                };
                if last != val {
                    // If I'm lava and the last cube is exposed then this is an exposed face
                    // If I was in lava and now in air and I'm exposed then this is an exposed face
                    if (val && am_i_exposed(cube, exposed_cache, last_coord, &HashSet::new()))
                        || (!val && am_i_exposed(cube, exposed_cache, d, &HashSet::new())) {
                        surface_area += 1;
                    }
                }
                last = val;
                last_coord = d;
            }
        }
    }
    surface_area
}

pub fn am_i_exposed(cube: &Cube, cache: &mut HashMap<Coord, bool>, c: Coord, path: &HashSet<Coord>) -> bool {
    if cache.contains_key(&c) {
        return *cache.get(&c).unwrap();
    }
    let max = c.0.max(c.1).max(c.2);
    let min = c.0.min(c.1).min(c.2);
    if max >= cube.len() || min == 0 {
        cache.insert(c, true);
        return true
    }
    if cube[c.0][c.1][c.2] {
        cache.insert(c, false);
        return false;
    }
    let adjacent_points = [
        (c.0+1, c.1, c.2),
        (c.0-1, c.1, c.2),
        (c.0, c.1+1, c.2),
        (c.0, c.1-1, c.2),
        (c.0, c.1, c.2+1),
        (c.0, c.1, c.2-1),
    ];

    let mut v = false;
    let mut next_path = path.clone();
    next_path.insert(c);
    for n in adjacent_points {
        if !next_path.contains(&n) && am_i_exposed(cube, cache, n, &next_path) {
            v = true;
            break
        }
    }

    cache.insert(c, v);
    v
}

pub fn solve(input: &str) -> i32 {
    let cube = parse_input(input);
    let mut cache: HashMap<Coord, bool> = HashMap::new();

    let mut surface_area = 0;
    surface_area += count_along_axis(&cube, &mut cache, Axis::X);
    surface_area += count_along_axis(&cube,&mut cache,Axis::Y);
    surface_area += count_along_axis(&cube, &mut cache,Axis::Z);

    surface_area
}