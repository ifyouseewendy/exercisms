use std::collections::HashMap;

pub mod graph {
    pub mod graph_items {
        pub mod edge {
            #[derive(Clone, Debug, PartialEq)]
            pub struct Edge {
                from: String,
                to: String,
                attrs: crate::HashMap<String, String>,
            }

            impl Edge {
                pub fn new(from: &str, to: &str) -> Self {
                    Self {
                        from: from.to_string(),
                        to: to.to_string(),
                        attrs: crate::HashMap::new(),
                    }
                }

                pub fn with_attrs(mut self, attrs: &[(&str, &str)]) -> Self {
                    for attr in attrs.iter() {
                        self.attrs.insert(attr.0.to_string(), attr.1.to_string());
                    }
                    self
                }
            }
        }
        pub mod node {
            #[derive(Clone, Debug, PartialEq)]
            pub struct Node {
                pub name: String,
                attrs: crate::HashMap<String, String>,
            }

            impl Node {
                pub fn new(s: &str) -> Self {
                    Self {
                        name: s.to_string(),
                        attrs: crate::HashMap::new(),
                    }
                }
                pub fn with_attrs(mut self, attrs: &[(&str, &str)]) -> Self {
                    for attr in attrs.iter() {
                        self.attrs.insert(attr.0.to_string(), attr.1.to_string());
                    }
                    self
                }
                pub fn get_attr(&self, key: &str) -> Option<&str> {
                    self.attrs.get(key).map(|v| v.as_str())
                }
            }
        }
    }

    type Node = graph_items::node::Node;
    type Edge = graph_items::edge::Edge;

    #[derive(Default)]
    pub struct Graph {
        pub nodes: Vec<Node>,
        pub edges: Vec<Edge>,
        pub attrs: crate::HashMap<String, String>,
    }

    impl Graph {
        pub fn new() -> Self {
            Default::default()
        }

        pub fn with_nodes(mut self, nodes: &[Node]) -> Self {
            for node in nodes.iter() {
                self.nodes.push(node.clone());
            }
            self
        }
        pub fn with_edges(mut self, edges: &[Edge]) -> Self {
            for edge in edges.iter() {
                self.edges.push(edge.clone());
            }
            self
        }
        pub fn with_attrs(mut self, attrs: &[(&str, &str)]) -> Self {
            for attr in attrs.iter() {
                self.attrs.insert(attr.0.to_string(), attr.1.to_string());
            }
            self
        }

        pub fn get_node(&self, name: &str) -> Option<&Node> {
            self.nodes.iter().find(|&n| n.name == name)
        }
    }
}
