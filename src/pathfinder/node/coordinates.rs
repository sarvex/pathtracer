/*
    Coordinates
    -----------
    Stores an x and y coordinate representing a position on a map.
*/

extern crate rand;
use super::util::roll;
use std::f64;
use std::cmp::Ordering;

#[derive(Eq)]
pub struct Coordinates {
    pub x: i16,
    pub y: i16,
}

impl PartialEq for Coordinates {
    fn eq(&self, other: &Coordinates) -> bool {
        (self.x == other.x) && (self.y == other.y)
    }
}

impl Coordinates {

    pub fn new(x: i16, y: i16) -> Coordinates {
        Coordinates {
            x,
            y
        }
    }

    pub fn gen() -> Coordinates {
        Coordinates {
            x: rand::random::<i16>(),
            y: rand::random::<i16>(),
        }
    }

    pub fn gen_within_radius(coord: Coordinates, radius: i16) -> Coordinates {
        if radius < 4 {
            panic!("too low radius provided");
        }

        let mut rng = rand::thread_rng();

        // Randomly gets the radius of the circle.

        let r = roll((radius/2) as u32, radius as u32) as f64;

        // gets a point on the circle's circumference.
        let cir = |a: f64, b: f64| a + r * b;

        // Gets the Angle
        let angle = roll(0, 10);
        let a: f64 = f64::consts::PI * (0.1 * angle as f64);

        let roll2: i16 = roll(0, 2 +(radius/4) as u32) as i16;

        let x = cir(coord.x as f64, a.cos()) as i16;                // x = cx + r * cos(a)
        let y = cir(coord.y as f64, a.sin()) as i16 -roll2;            // y = cy + r * sin(a)

        Coordinates {
            x,
            y
        }
    }
}

impl Clone for Coordinates {
    fn clone(&self) -> Coordinates {
        Coordinates {
            x: self.x,
            y: self.y
        }
    }
}

impl Ord for Coordinates {
    fn cmp(&self, other: &Coordinates) -> Ordering {
        self.x.cmp(&other.x) // TODO improve.
    }
}

impl PartialOrd for Coordinates {
    fn partial_cmp(&self, other: &Coordinates) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}