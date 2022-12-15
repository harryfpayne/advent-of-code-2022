pub type Coord = (usize, usize);
pub type Map = Vec<Vec<i32>>;

pub fn parse_input(input: &str) -> (Map, Coord, Coord) {
    let mut map = vec![];

    let mut start = (0,0);
    let mut end = (0,0);

    for (y, line) in input.lines().enumerate() {
        map.push(vec![]);
        for (x, char) in line.chars().enumerate() {
            if char == 'S' {
                start = (y,x);
                map[y].push(0)
            } else if char == 'E' {
                end = (y,x);
                map[y].push('z' as i32 - 'a' as i32 + 2)
            } else {
                map[y].push(char as i32 - 'a' as i32 + 1);
            }
        }
    }

    (map, start, end)
}

pub fn map_get<'a>(map: &'a Map, c: &Coord) -> &'a i32 {
    map.get(c.0).expect("invalid y").get(c.1).expect("invalid x")
}

pub fn distance(a: &Coord, b: &Coord) -> f32 {
    let opo = a.0.abs_diff(b.0);
    let adj = a.1.abs_diff(b.1);
    let max = opo.max(adj) as f32;
    let min = opo.min(adj) as f32;
    max + 0.428 * min * min / max
}

pub fn get_possible_moves(curr_position: Coord, map_height: usize, map_width: usize) -> Vec<Coord> {
    let movement_vectors: [(i32,i32);4] = [(0,1), (1,0), (-1,0), (0,-1)];
    let mut allowed = vec![];
    for x in movement_vectors {
        let new_y = (curr_position.0 as i32 + x.0);
        let new_x = (curr_position.1 as i32 + x.1);
        if new_y >= 0 && new_y < map_height as i32 && new_x >= 0 && new_x < map_width as i32 {
            allowed.push((new_y as usize, new_x as usize));
        }
    }

    allowed
}