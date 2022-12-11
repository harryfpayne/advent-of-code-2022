use std::ops::{BitXor, Range};
use one::Direction;
use crate::one;

pub const ROPE_SIZE: usize = 10;

pub struct Board {
    pub knots: [(i32,i32); ROPE_SIZE],
}
impl Board {
    pub fn new() -> Board {
        Board {
            knots: [(0,0),(0,0),(0,0),(0,0),(0,0),(0,0),(0,0),(0,0),(0,0),(0,0)],
        }
    }
    pub fn mv(&mut self, d: &Direction) {
        let mut original_positions: [(i32,i32);ROPE_SIZE] = [(0,0),(0,0),(0,0),(0,0),(0,0),(0,0),(0,0),(0,0),(0,0),(0,0)];
        original_positions.clone_from_slice(&self.knots);

        match d {
            Direction::U => self.knots[0].0 -= 1,
            Direction::D => self.knots[0].0 += 1,
            Direction::L => self.knots[0].1 -= 1,
            Direction::R => self.knots[0].1 += 1,
        }

        for i in 1..self.knots.len() {
            let cur_position = self.knots[i];
            let leader_position = self.knots[i-1];

            let y_diff = leader_position.0 - cur_position.0;
            let x_diff = leader_position.1 - cur_position.1;

            if y_diff.abs().max(x_diff.abs()) < 2 {
                continue
            }

            let y_mov = if y_diff < 0 { -1 } else if y_diff == 0 { 0 } else { 1 };
            let x_mov = if x_diff < 0 { -1 } else if x_diff == 0 { 0 } else { 1 };
            self.knots[i] = (cur_position.0 + y_mov, cur_position.1 + x_mov);
        }
    }
    pub fn print(&self, size: i32) {
        let max = size;//self.knots.iter().fold(0, |acc, c|  acc.max(c.0).max(c.1));
        let min = -size;//self.knots.iter().fold(20, |acc, c|  acc.min(c.0).min(c.1));
        for y in min..max {
            for x in min..max {
                let mut found = false;
                for (i, k) in self.knots.iter().enumerate() {
                    if &(y,x) == k {
                        print!("{}", i);
                        found = true;
                        break;
                    }
                }
                if !found {
                    if (y,x) == (0,0) {
                        print!("s");
                    } else {
                        print!(".");
                    }
                }
            }
            println!()
        }
        println!("<><><><><><><>")
    }

}