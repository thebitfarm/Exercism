#[macro_use]
extern crate maplit;
extern crate dot_dsl;

use dot_dsl::graph::graph_items::edge::Edge;
use dot_dsl::graph::graph_items::node::Node;
use dot_dsl::graph::Graph;


fn main() {

    let nodes = vec![Node::new("a").with_attrs(&[("color", "green"),("font-family","Arial")]),
        Node::new("b").with_attrs(&[("weight", "bold")])
    ];

    let graph = Graph::new().with_attrs(&[("foo", "1")]).with_nodes(&nodes);

    println!("Graph is: {}", graph);


}