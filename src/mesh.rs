use crate::point::Point;

/// At first we are going to solve a hexagon
///       4 ________ 3
///        /\      /\
///       /  \ e3 /  \
///      / e4 \  / e2 \
///    5/______\/6_____\ 2
///     \      /\      /
///      \ e5 /  \ e1 /
///       \  / e0 \  /
///        \/______\/
///        0        1

const NODE0: Point<f64> = Point::new(0., 0.);
const NODE1: Point<f64> = Point::new(1., 0.);
const NODE2: Point<f64> = Point::new(1.5, 0.866025404);
const NODE3: Point<f64> = Point::new(1., 1.73205080);
const NODE4: Point<f64> = Point::new(0., 1.73205080);
const NODE5: Point<f64> = Point::new(-0.5, 0.866025404);
const NODE6: Point<f64> = Point::new(0.5, 0.866025404);

pub struct Mesh {
    pub number_nodes: usize,
    pub number_elements: usize,
    pub node_list: Vec<Point<f64>>,
    pub element_list: Vec<[usize; 3]>,
}

impl Mesh {
    pub fn new() -> Self {
        Mesh {
            number_nodes: 7,
            number_elements: 6,
            node_list: vec![NODE0, NODE1, NODE2, NODE3, NODE4, NODE5, NODE6],
            element_list: vec![
                [0, 1, 6],
                [1, 2, 6],
                [2, 3, 6],
                [3, 4, 6],
                [4, 5, 6],
                [5, 0, 6],
            ],
        }
    }
}
