pub mod graph {
    pub mod graph_items {
        pub mod edge {
            use std::collections::HashMap;

            #[derive(Clone, Debug, PartialEq, Eq)]
            pub struct Edge {
                pub from: String,
                pub to: String,
                pub attrs: HashMap<String, String>,
            }

            impl Edge {
                pub fn new(from: &str, to: &str) -> Self {
                    Edge {
                        from: from.to_string(),
                        to: to.to_string(),
                        attrs: HashMap::new(),
                    }
                }

                pub fn with_attrs(mut self, attrs: &[(&str, &str)]) -> Self {
                    self.attrs.extend(
                        attrs
                            .into_iter()
                            .map(|(k, v)| (k.to_string(), v.to_string())),
                    );
                    self
                }

                pub fn get_attr(&self, label: &str) -> Option<&str> {
                    self.attrs.get(label).map(String::as_str)
                }
            }
        }

        pub mod node {
            use std::collections::HashMap;

            #[derive(Clone, Debug, PartialEq, Eq)]
            pub struct Node {
                pub name: String,
                pub attrs: HashMap<String, String>,
            }

            impl Node {
                pub fn new(name: &str) -> Self {
                    Node {
                        name: name.to_string(),
                        attrs: HashMap::new(),
                    }
                }

                pub fn with_attrs(mut self, attrs: &[(&str, &str)]) -> Self {
                    self.attrs.extend(
                        attrs
                            .into_iter()
                            .map(|(k, v)| (k.to_string(), v.to_string())),
                    );
                    self
                }

                pub fn get_attr(&self, label: &str) -> Option<&str> {
                    self.attrs.get(label).map(String::as_str)
                }
            }

            use std::fmt;
            impl fmt::Display for Node {
                fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
                    let body_attrs:String = match self.attrs.is_empty() {
                        false => { let mut qh = self.attrs.iter()
                            .fold(" [".to_string(), |mut result, (k,v)| {result.push_str(&format!("{}=\"{}\" ",*k,*v)); result} );
                            qh.push_str("]");
                            qh
                        },
                        true => "".to_string()
                    };
                    write!(f, "{}{}", self.name, body_attrs)
                }
            }
        }
    }

    use self::graph_items::{edge::Edge, node::Node};
    use std::collections::HashMap;
    use std::fmt;

    #[derive(Debug, Default)]
    pub struct Graph {
        pub edges: Vec<Edge>,
        pub nodes: Vec<Node>,
        pub attrs: HashMap<String, String>,
    }

    impl Graph {
        pub fn new() -> Self {
            Graph::default()
        }

        pub fn with_edges(mut self, edges: &[Edge]) -> Self {
            self.edges.extend_from_slice(edges);
            self
        }

        pub fn with_nodes(mut self, nodes: &[Node]) -> Self {
            self.nodes.extend_from_slice(nodes);
            self
        }

        pub fn with_attrs(mut self, attrs: &[(&str, &str)]) -> Self {
            self.attrs.extend(
                attrs
                    .into_iter()
                    .map(|(k, v)| (k.to_string(), v.to_string())),
            );
            self
        }

        pub fn get_attr(&self, label: &str) -> Option<&str> {
            self.attrs.get(label).map(String::as_str)
        }

        pub fn get_node(&self, name: &str) -> Option<&Node> {
            self.nodes.iter().find(|n| n.name == name)
        }
    }

    impl fmt::Display for Graph {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            let preamble:String = format!("graph {{\n");
            let body_attrs:String = match self.attrs.is_empty() {
                false => { let mut qh = self.attrs.iter()
                    .fold("\tgraph [".to_string(), |mut result, (k,v)| {result.push_str(&format!("{}=\"{}\" ",*k,*v)); result} );
                    qh.push_str("]\n");
                    qh
                },
                true => "".to_string()
            };
            let nodes:String = self.nodes.iter().map(|n| {let mut i="\t".to_string(); i.push_str(&n.to_string()); i}).collect::<Vec<_>>().join("\n")+"\n";
            let suffix:String = format!("}}");
            write!(f, "{}{}{}{}", preamble, body_attrs, nodes, suffix)
        }
    }
}