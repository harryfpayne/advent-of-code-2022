use std::fmt::{Debug, Formatter};
use crate::rock_formations::get_next_rock;

#[derive(Copy, Clone, Eq, PartialEq, Debug)]
enum Point {
    Empty,
    Falling,
    Settled,
}
pub enum AirDirection {
    Left,
    Right,
}

type Coord = (usize, usize);

const HEIGHT: usize = 10000;
const WIDTH: usize = 7;

pub struct Map {
    map: Vec<Vec<Point>>
}
impl Map {
    pub fn new() -> Map {
        let row = vec![Point::Empty; WIDTH];
        let mut map = vec![row; HEIGHT];

        for i in 0..WIDTH {
            map[0][i] = Point::Settled
        }

        Map {
            map
        }
    }
    fn set(&mut self, p: Coord, v: Point) {
        self.map[p.0][p.1] = v;
    }
    fn get(&self, p: Coord) -> Option<&Point> {
        let r = self.map.get(p.0);
        if r.is_none() {
            return None;
        }
        r.unwrap().get(p.1)
    }
    pub fn spawn_rock(&mut self, turn: usize) {
        let highest_row = self.highest_rock_row();
        let next_starting_coord: Coord = (highest_row + 4, 2);
        let next_rock: Vec<Coord> = get_next_rock(turn);

        for displacement in next_rock {
            let rock_point = (next_starting_coord.0 + displacement.0, next_starting_coord.1 + displacement.1);
            self.set(rock_point, Point::Falling);
        }
    }
    pub fn fall(&mut self) -> bool {
        let mut is_settling = false;
        // Check if any point is about to fall onto a settled block
        for y in 1..HEIGHT {
            for x in 0..WIDTH {
                if self.map[y][x] == Point::Falling {
                    if self.map[y-1][x] == Point::Settled {
                        is_settling = true;
                        break;
                    }
                }
            }
        }

        for y in 1..HEIGHT {
            for x in 0..WIDTH {
                if self.map[y][x] == Point::Falling {
                    if is_settling {
                        self.map[y][x] = Point::Settled;
                    } else {
                        self.map[y][x] = Point::Empty;
                        self.map[y-1][x] = Point::Falling;
                    }
                }
            }
        }

        is_settling
    }
    pub fn blow(&mut self, direction: AirDirection) {
        let dir: i32 = match direction {
            AirDirection::Left => {
                // println!("Pushing left");
                -1
            },
            AirDirection::Right => {
                // println!("Pushing right");
                1
            },
        };

        let mut is_going_to_collide = false;
        for y in 1..HEIGHT {
            for x in 0..WIDTH {
                if self.map[y][x] == Point::Falling {
                    if x == 0 && dir == -1 {
                        is_going_to_collide = true;
                        break;
                    }
                    let next_point = (y, (x as i32 + dir) as usize);
                    let can_go_there = self.get(next_point).is_some_and(|p| p == &Point::Empty || p == &Point::Falling);
                    if !can_go_there {
                        is_going_to_collide = true;
                        break;
                    }
                }
            }
        }

        if !is_going_to_collide {
            for y in 1..HEIGHT {
                let scan_dir = match direction {
                    AirDirection::Left => (0..WIDTH).collect(),
                    AirDirection::Right => (0..WIDTH).rev().collect::<Vec<usize>>(),
                };
                for x in scan_dir {
                    if self.map[y][x] == Point::Falling {
                        let next_point = (y, (x as i32 + dir) as usize);
                        self.set(next_point, Point::Falling);
                        self.set((y,x), Point::Empty);
                    }
                }
            }
        }
    }
    pub fn highest_rock_row(&self) -> usize {
        for (y, row) in self.map.iter().enumerate().rev() {
            for point in row.iter() {
                if point == &Point::Settled {
                    return y;
                }
            }
        }
        0
    }
    pub fn get_height_profile(&self) -> [usize; WIDTH] {
        let mut a = [0; WIDTH];
        for x in 0..WIDTH {
            for (y, row) in self.map.iter().enumerate().rev() {
                if row[x] != Point::Empty {
                    a[x] = y;
                }
            }
        }
        a
    }
}

impl Debug for Map {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let mut found_something = false;
        for row in self.map.iter().rev() {
            let mut row_str = String::from("|");
            for point in row.iter() {
                row_str += match point {
                    Point::Empty => ".",
                    Point::Falling => {
                        found_something = true;
                        "@"
                    },
                    Point::Settled => {
                        found_something = true;
                        "#"
                    },
                }
            }
            row_str += "|";
            if found_something {
                writeln!(f, "{}", row_str)?;
            }
        }

        Ok(())
    }
}