//! Author: Pontus Laestadius
//! Version: 0.3
//! Since: 2017-04-06
//!
//! Visualizes a moving gif.
//!

extern crate gif;
extern crate image;
extern crate pathfinder;
extern crate rand;

use image::Rgba;
use pathfinder::{map::gif::*, *};

fn main() {
    let mut gif = Gif::new("out.gif", 200, 100).unwrap();
    let radius = [30, 20, 40];
    let color = [[250, 20, 20, 255], [20, 20, 250, 255], [20, 250, 20, 255]];

    for _ in 0..10 {
        let mut groups = Group::from_list(&[(0, 0), (45, 40), (110, 20)]);

        for (j, ref mut group) in groups.iter_mut().enumerate() {
            group.settings.radius = Some(radius[j]);
            group.settings.color = Rgba { data: color[j] };
            map::network::add_children(group, 100);
        }

        let mut map = Map::new();
        map = map.map(&groups);
        gif.push_frame(&map.image.unwrap()).unwrap();
    }
}
