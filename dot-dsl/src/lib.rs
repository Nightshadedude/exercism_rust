use std::collections::HashMap;

macro_rules! impl_attrs {
    () => {
        pub fn get_attr(&self, key: &str) -> Option<&str> {
            self.attrs.get(key).map(|s| s.as_str())
        }

        pub fn with_attrs(mut self, attrs: &[(&str, &str)]) -> Self {
            self.attrs = attrs
                .iter()
                .map(|(a, b)| (a.to_string(), b.to_string()))
                .collect();
            self
        }
    };
}

#[derive(PartialEq, Eq, Default)]
pub struct Graph {
    pub nodes: Vec<Node>,
    pub edges: Vec<Edge>,
    pub attrs: HashMap<String, String>,
}

impl Graph {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn with_edges(mut self, edges: &[Edge]) -> Self {
        self.edges = Vec::from(edges);
        self
    }

    pub fn with_nodes(mut self, nodes: &[Node]) -> Self {
        self.nodes = Vec::from(nodes);
        self
    }

    pub fn get_node(&self, name: &str) -> Option<&Node> {
        self.nodes.iter().find(|n| n.name == name)
    }
    impl_attrs!();
}

#[derive(PartialEq, Eq, Default, Debug, Clone)]
pub struct Edge {
    from: String,
    to: String,
    attrs: HashMap<String, String>,
}

impl Edge {
    pub fn new(from: &str, to: &str) -> Self {
        Edge {
            from: String::from(from),
            to: String::from(to),
            ..Self::default()
        }
    }

    impl_attrs!();
}

#[derive(PartialEq, Eq, Default, Debug, Clone)]
pub struct Node {
    name: String,
    attrs: HashMap<String, String>,
}

impl Node {
    pub fn new(name: &str) -> Self {
        Node {
            name: String::from(name),
            ..Self::default()
        }
    }
    impl_attrs!();
}

pub mod graph {
    pub use super::Graph;
    pub mod graph_items {
        pub mod edge {
            pub use super::super::super::Edge;
        }

        pub mod node {
            pub use super::super::super::Node;
        }
    }
}
