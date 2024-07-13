use crate::point::Point;

pub struct Mesh {
    pub number_nodes: usize,
    pub number_elements: usize,
    pub node_list: Vec<Point<f64>>,
    pub element_list: Vec<Point<usize>>,
}

impl Mesh {
    pub fn new(number_elements: usize) -> Self {
        Mesh {
            number_nodes: 3,
            number_elements,
            node_list: vec![Point::new(0., 0.), Point::new(1., 0.), Point::new(0.5, 1.)],
            element_list: vec![Point::new(1, 2), Point::new(2, 3), Point::new(3, 1)],
        }
    }
}
