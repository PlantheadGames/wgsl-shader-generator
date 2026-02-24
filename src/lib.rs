pub mod graph;
pub use graph::*;

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    pub fn test_add() {
        assert_eq!(1 + 1, 2);
    }
    #[test]
    pub fn test_inital_graph_generation() {
        let mut graph = graph::Graph::new();
        assert!(graph.nodes.is_empty());
        let _node0 = graph.add_node(
            ShaderData::String("this is a node".to_string()),
            Datatype::Vec3,
            Datatype::String,
        );
        assert!(!graph.nodes.is_empty());
    }

    #[test]
    pub fn test_node_connections() {
        let mut graph = graph::Graph::new();
        assert!(graph.nodes.is_empty());

        let node0 = graph.add_node(
            ShaderData::String("this is a node".to_string()),
            Datatype::Vec3,
            Datatype::String,
        );
        let node1 = graph.add_node(
            ShaderData::String("this is a second node".to_string()),
            Datatype::String,
            Datatype::Float,
        );

        graph.add_edge(
            graph.nodes.get(&node0).unwrap().clone(),
            graph.nodes.get(&node1).unwrap().clone(),
        );

        graph.get_inputs(node1);
        graph.get_outputs(node0);

        assert_eq!(
            graph
                .nodes
                .get(&node0)
                .unwrap()
                .clone()
                .outgoing_connections_type,
            graph
                .nodes
                .get(&node1)
                .unwrap()
                .clone()
                .incoming_connections_type
        );
        println!("{:#?}", graph);
    }
}

pub fn one_add_one() {
    assert_eq!(1 + 1, 2);
}

pub fn criterion_inital_graph_generation() {
    let mut graph = graph::Graph::new();
    assert!(graph.nodes.is_empty());
    let _node0 = graph.add_node(
        ShaderData::String("this is a node".to_string()),
        Datatype::Vec3,
        Datatype::String,
    );
    assert!(!graph.nodes.is_empty());
}
pub fn criterion_node_connections() {
    let mut graph = graph::Graph::new();
    assert!(graph.nodes.is_empty());

    let node0 = graph.add_node(
        ShaderData::String("this is a node".to_string()),
        Datatype::Vec3,
        Datatype::String,
    );
    let node1 = graph.add_node(
        ShaderData::String("this is a second node".to_string()),
        Datatype::String,
        Datatype::Float,
    );

    graph.add_edge(
        graph.nodes.get(&node0).unwrap().clone(),
        graph.nodes.get(&node1).unwrap().clone(),
    );

    graph.get_inputs(node1);
    graph.get_outputs(node0);

    assert_eq!(
        graph
            .nodes
            .get(&node0)
            .unwrap()
            .clone()
            .outgoing_connections_type,
        graph
            .nodes
            .get(&node1)
            .unwrap()
            .clone()
            .incoming_connections_type
    );
}
