use crate::parse::{parse_input, Cube, Axis};

fn count_along_axis(cube: &Cube, axis: Axis) -> i32 {
    let mut surface_area = 0;
    for a in 0..cube.len()+1 {
        for b in 0..cube.len()+1 {
            let mut last = false;
            for c in 0..cube.len()+1 {
                let d = axis.get_tuple(a,b,c);
                let val = if cube.len() <= a || cube.len() <= b || cube.len() <= c {
                    false
                } else {
                    cube[d.0][d.1][d.2]
                };
                if last != val {
                    surface_area += 1;
                    last = val;
                }
            }
        }
    }
    surface_area
}

pub fn solve(input: &str) -> i32 {
    let cube = parse_input(input);

    let mut surface_area = 0;
    surface_area += count_along_axis(&cube, Axis::X);
    surface_area += count_along_axis(&cube, Axis::Y);
    surface_area += count_along_axis(&cube, Axis::Z);

    surface_area
}