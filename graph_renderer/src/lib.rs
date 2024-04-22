mod graph;
mod graph_2d;
mod graph_nd;
mod node_2d;
mod node_nd;

pub use graph::Graph;
pub use graph_2d::Graph2D;
pub use graph_nd::GraphND;
pub use node_2d::Node2D;
pub use node_nd::{NodeND, pos_to_node};


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}
