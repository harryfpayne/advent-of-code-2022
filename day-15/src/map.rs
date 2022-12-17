use std::collections::{HashMap, HashSet};
use std::thread;
use kdam::tqdm;

pub type Coord = (i32, i32);
#[derive(PartialEq, Debug)]
pub enum Point {
    Sensor,
    Beacon,
    NoBeacon,
    Unknown,
}

pub struct Map {
    pub(crate) sensors: HashMap<Coord, (Coord, i32)>,
    pub(crate) max_x: i32,
    pub(crate) min_x: i32,
}
impl Map {
    pub fn distance(a: &Coord, b: &Coord) -> i32 {
        let y = a.0.abs_diff(b.0);
        let x = a.1.abs_diff(b.1);
        (y+x) as i32
    }
    pub fn what_is_point(&self, p: &Coord) -> Point {
        for (sensor, (beacon, sensor_distance)) in self.sensors.iter() {
            if sensor == p {
                return Point::Sensor;
            }
            if beacon == p {
                return Point::Beacon;
            }
            let point_distance = Map::distance(sensor, p);

            if &point_distance <= sensor_distance {
                return Point::NoBeacon;
            }
        }
        return Point::Unknown;
    }
    pub fn count_no_beacons_in_row(&self, row: i32) -> i32 {
        let mut count = 0;
        for x in (self.min_x..self.max_x + 1) {
            let point = (row, x);
            if self.what_is_point(&point) == Point::NoBeacon {
                count += 1
            } else {
            }
        }
        count
    }
    pub fn find_no_beacon(&self) {
        for y in tqdm!(0..area) {
            for x in 0..area + 1 {
                let point = (y, x);
                if self.what_is_point(&point) == Point::Unknown {
                    println!("{:?}", point);
                }
            }
        }
    }
}

pub const area: i32 = 4000000;