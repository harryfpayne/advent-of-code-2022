pub enum Direction {
    U,
    D,
    L,
    R,
}
pub struct Board {
    pub head: (i32, i32),
    pub tail: (i32, i32),
}
impl Board {
    pub fn mv(&mut self, d: &Direction) {
        let mut original_head = self.head.clone();
        match d {
            Direction::U => self.head.0 -= 1,
            Direction::D => self.head.0 += 1,
            Direction::L => self.head.1 -= 1,
            Direction::R => self.head.1 += 1,
        }
        if (self.head.0 - self.tail.0).abs() > 1 || (self.head.1 - self.tail.1).abs() > 1 {
            self.tail = original_head;
        }
    }
    pub fn print(&self) {
        let max = self.head.0.max(self.head.1).max(self.tail.0).max(self.tail.1).max(6);
        let min = self.head.0.min(self.head.1).min(self.tail.0).min(self.tail.1).min(0);
        for y in min..max {
            for x in min..max {
                if (y,x) == self.head {
                    print!("H")
                } else if (y,x) == self.tail {
                    print!("T")
                } else {
                    print!(".")
                }
            }
            println!()
        }
        println!("<><><><><><><>")
    }
}