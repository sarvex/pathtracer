use super::*;
use std::cmp;

/// Counts the amount of child Nodes in a list of Groups.
///
/// # Examples
/// ```
/// use pathfinder::{group, map::network, Group};
/// let mut groups = Group::from_list(&[(0, 0), (100, 100)]);
/// for group in groups.iter_mut() {
///     network::add_children(group, 50);
/// }
/// assert_eq!(group::count(&groups), 100);
/// ```
pub fn count(list: &[Group]) -> usize {
    let mut n: usize = 0;
    for g in list.iter() {
        n += g.nodes.len();
    }
    n
}

/// Returns the the largest and smallest x and y position.
///
/// # Examples
/// ```
/// use pathfinder::{group, Coordinate, Group, Node};
/// let mut group = Group::new_simple(0, 0);
/// group.push(Node::new("", Coordinate::new(100, 100)));
/// let (min, max) = group::get_parameters(&group);
/// assert_eq!(min.x, 0); // y values are identical
/// assert_eq!(max.x, 100);
/// ```
pub fn get_parameters(group: &Group) -> (Coordinate, Coordinate) {
    let mut min_x: i16 = 0;
    let mut min_y: i16 = 0;
    let mut max_x: i16 = 0;
    let mut max_y: i16 = 0;

    for node in &group.nodes {
        let (min, max) = node.get_parameters();
        max_x = std::cmp::max(max_x, max.x);
        min_x = std::cmp::min(min_x, min.x);
        max_y = std::cmp::max(max_y, max.y);
        min_y = std::cmp::min(min_y, min.y);
    }
    (Coordinate::new(min_x, min_y), Coordinate::new(max_x, max_y))
}

/// Adds a node to a given group, All parameters are optional except the group.
/// This is the underlying function used in Group::push(..).
pub fn add_node(group: &mut Group, name: Option<&str>, min: Option<u32>, max: Option<u32>) {
    let name = name.unwrap_or("");
    let min = min.unwrap_or(0);
    let max = max.unwrap_or_else(|| group.get_dynamic_radius());

    let mi = cmp::min(min, max);
    let ma = cmp::max(min, max);

    let geo = coordinate::gen_radius(group.settings.geo, mi, ma);
    let mut node = Node::new(name, geo);
    node.color = group.gen_color(geo);
    node.radius = group.settings.radius;
    group.push(node);
}

#[cfg(test)]
mod tests {
    use super::{super::Node, *};

    #[test]
    fn test_count_none() {
        let groups = Group::from_list(&[(0, 0), (100, 100)]);
        assert_eq!(count(&groups), 0);
    }

    #[test]
    fn test_count_some() {
        let mut groups = Group::from_list(&[(0, 0), (100, 100)]);
        groups[0].nodes = Node::from_list(&[(0, 0), (0, 0)]);
        assert_eq!(count(&groups), 2);
    }

    #[test]
    fn test_add_node() {
        let mut group = Group::new_simple(0, 0);
        add_node(&mut group, None, None, None);
        add_node(&mut group, Some("name"), Some(50), Some(20));
        assert_eq!(group.nodes.len(), 2);
    }

}
