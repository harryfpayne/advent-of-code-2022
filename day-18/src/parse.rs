pub type Cube = Vec<Vec<Vec<bool>>>;
pub type Coord = (usize, usize, usize);

pub fn parse_input(input: &str) -> Cube {
    let mut max = 0;
    let mut coords: Vec<Coord> = vec![];
    for line in input.lines() {
        let mut b = line.split(",");
        let c = (
            b.next().unwrap().parse::<usize>().unwrap(),
            b.next().unwrap().parse::<usize>().unwrap(),
            b.next().unwrap().parse::<usize>().unwrap(),
        );
        max = max.max(c.0 + 1).max(c.1 + 1).max(c.2 + 1);
        coords.push(c);
    }

    let mut cube: Cube = vec![vec![vec![false; max]; max]; max];
    for x in coords {
        cube[x.0][x.1][x.2] = true;
    }

    cube
}

pub enum Axis {
    X,
    Y,
    Z,
}
impl Axis {
    pub fn get_tuple(&self, a: usize, b: usize, c: usize) -> Coord {
        match self {
            Axis::X => (a, b, c),
            Axis::Y => (a, c, b),
            Axis::Z => (b, c, a),
        }
    }
}