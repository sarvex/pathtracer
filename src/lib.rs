#![cfg_attr(feature="clippy", feature(plugin))]
#![cfg_attr(feature="clippy", plugin(clippy))]

extern crate image;
extern crate rand;

pub mod node;
pub mod map;
pub mod tools;
pub mod group;
pub mod data;

mod tests;

/// Holds a position used for Nodes and Groups.
#[derive(Debug, Eq, Copy, Clone)]
pub struct Coordinate {
    pub x: i16,
    pub y: i16,
}

/// A positioned object that can be drawn on an image::ImageBuffer.
#[derive(Clone)]
pub struct Node<T: Shape> {
    pub hash: u64,
    pub geo: Coordinate,
    pub color: image::Rgba<u8>,
    pub radius: Option<u32>,
    shape: T
}

/// Holds a set of nodes and applies properties to all child nodes when drawn.
/// The group itself has no displayed output and is not visible.
#[derive(Clone)]
pub struct Group<T: Shape> {
    pub settings: Node<T>,
    pub nodes: Vec<Node<T>>,
}

// ------------------------------------------------------------------

pub trait Shape {
	fn new() -> Self;
    fn area(&self, size: u32) -> Vec<Coordinate>;
}

#[derive(Debug)]
pub struct Square {}

#[derive(Debug)]
pub struct Circle {}

// ------------------------------------------------------------------

impl Shape for Square {
	fn new() -> Square {
		Square {}
	}

	/// Returns all coordinates that the shape occupies. 
	/// Assume that you start at coordinate x: 0, y: 0.
    fn area(&self, size: u32) -> Vec<Coordinate> {
        let mut vec = Vec::new();
        for i in 0..size {
            for j in 0..size {
                vec.push(Coordinate::new(i as i16, j as i16));
            }
        }
        vec
    }
}

impl Shape for Circle {
    fn new() -> Circle {
        Circle {}
    }

    /// Returns all coordinates that the shape occupies. 
    /// Assume that you start at coordinate x: 0, y: 0.
    fn area(&self, size: u32) -> Vec<Coordinate> {
        let mut vec = Vec::new();

        let l = |size| {
            let mut vec = Vec::new();
            let r2 = (size*size) as i16;
            let size = size as i16;
            let h = size/2;
            for x in 0..size +1 {
                let f = (r2 - x*x) as f64;
                let y = (f.sqrt() + 0.1) as i16;
                vec.push(Coordinate::new(h +x,h +y));
                vec.push(Coordinate::new(h +x,h -y));
                vec.push(Coordinate::new(h -x,h +y));
                vec.push(Coordinate::new(h -x,h -y));
            }
            vec
        };

        for i in 0..size+1 {
            vec.append(&mut l(i));
        }

        vec
    }
}

// ------------------------------------------------------------------

impl Coordinate {
    /// Constructs a Coordinate struct.
    pub fn new(x: i16, y: i16) -> Coordinate {
        Coordinate {
            x,
            y
        }
    }
}

impl<T: Shape> Node<T> {
    /// Constructs a Node struct.
    pub fn new(name: &str, geo: Coordinate) -> Node<T> {
        Node {
            hash: data::calculate_hash(&name),
            geo,
            color: image::Rgba {data: [0,0,0,255]},
            radius: None,
            shape: T::new(),
        }
    }
}

impl<T: Shape> Group<T> {
    /// Constructs a new Group
    pub fn new(name: &str, coordinates: Coordinate) -> Group<T> {
        Group {
            settings: Node::new(name, coordinates),
            nodes: Vec::new(),
        }
    }
}

// ------------------------------------------------------------------

impl Coordinate {
    // Calculates the different in x and y of two Coordinates.
    pub fn diff(&self, other: &Coordinate) -> (i16, i16) {
        node::coordinates::diff(&self, other)
    }
}

impl<T: Shape> Node<T> {
    /// Draws a node on an ImageBuffer.
    pub fn draw(&self, image: &mut image::ImageBuffer<image::Rgba<u8>, Vec<u8>>, x_offset: u32, y_offset: u32, size: u32) {
        // Adds the offset to the geo location as i16. Because geo can be negative but offset can not.
        let x = self.geo.x +x_offset as i16;
        let y = self.geo.y +y_offset as i16;
        let size = match self.radius {
            Some(_) => self.radius.unwrap(),
            None => size
        };

      	for offset in self.shape.area(size) {
      		image.put_pixel((x +offset.x) as u32, (y +offset.y) as u32, image::Rgba {data: [0,0,0,255]});
      	}
    }
}

impl<T: Shape> Group<T> {

    /// Returns the nodes that exists inside the Group.
    pub fn get_nodes(&self) -> &Vec<Node<T>> {
        &self.nodes
    }

    /// Draws the Nodes inside that Group. If none the Group is draw as blank.
    pub fn draw(&self, image: &mut image::ImageBuffer<image::Rgba<u8>, Vec<u8>>, x_offset: u32, y_offset: u32, size: u32) {
        for node in self.nodes.iter() {
            node.draw(image, x_offset, y_offset, size);
        }
    }

    /// Adds a Node dynamically to the Group.
    pub fn new_node(&mut self, name: &str) {
        let geo = node::coordinates::gen_radius(&self.settings.geo, 0, self.get_dynamic_radius());
        self.new_node_inner(geo, name);
    }

    /// Adds a Node with a static distance from the center of the Group.
    pub fn new_node_min_auto(&mut self, name: &str, min: u32) -> &Node<T> {
        let geo = node::coordinates::gen_radius(&self.settings.geo, 0, min+5);
        self.new_node_inner(geo, name)
    }

    /// Adds a Node with a specific minimum and maximum distance from the center of the Group.
    pub fn new_node_min_max(&mut self, name: &str, min: u32, max: u32) -> &Node<T> {
        let geo = node::coordinates::gen_radius(&self.settings.geo, min, max);
        self.new_node_inner(geo, name)
    }

    /// Constructs a new node for the Group and mirrors the properties to it.
    pub fn new_node_inner(&mut self, geo: Coordinate, name: &str) -> &Node<T> {
        let mut node = Node::new(name,geo.clone());
        node.color = self.gen_color(geo);
        self.push(node);
        &self.nodes.get(self.nodes.len() -1).unwrap()
    }

    /// Pushes a Node to the Group.
    pub fn push(&mut self, node: Node<T>) {
        self.nodes.push(node);
    }

    /// Returns a dynamic radius based on the number of Nodes in the Group.
    pub fn get_dynamic_radius(&self) -> u32 {
        match self.settings.radius {
            Some(x) => x,
            None => 10 + self.nodes.len()as u32 /2,
        }
    }

    // Generates an image::Rgba based on the color of the Group and the distance from center.
    pub fn gen_color(&self, coordinates: Coordinate) -> image::Rgba<u8> {
        let radius = self.get_dynamic_radius() as i16;
        let (x_dif, y_dif) = self.settings.geo.diff(&coordinates);
        let x_scale: f64 = (x_dif as f64/radius as f64) as f64;
        let y_scale: f64 = (y_dif as f64/radius as f64) as f64;
        let c = self.settings.color.data;
        let max_multi: f64 = ((c[0] as i32 + c[1] as i32 + c[2] as i32)/3) as f64;
        let modify = (-max_multi*(x_scale+y_scale)/2.0) as i32;
        image::Rgba {data: [
            tools::border(c[0], modify),
            tools::border(c[1], modify),
            tools::border(c[2], modify),
            tools::border(c[3], 0)
        ]}
    }
}
