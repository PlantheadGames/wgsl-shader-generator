pub mod graph;
pub use graph::*;


#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    pub fn one_add_one(){
        assert_eq!(1 + 1, 2);
    }
#[test]
    pub fn test_inital_graph_generation(){
        let mut graph = graph::Graph::new();
        assert!(graph.nodes.is_empty());
        let node0 = graph.add_node("this is a node", Datatype::Vec3, Datatype::Str);
        assert!(!graph.nodes.is_empty());
    }

#[test]
    pub fn test_node_connections(){
        let mut graph = graph::Graph::new();
        assert!(graph.nodes.is_empty());
        let node0 = graph.add_node("this is a node", Datatype::Vec3, Datatype::Str);
        let node1 = graph.add_node("this is a second node" , Datatype::Str, Datatype::Float);
        graph.add_edge(*graph.nodes.get(&node0).unwrap(),*graph.nodes.get(&node1).unwrap());
        graph.get_inputs(node1);
        graph.get_outputs(node0);
        assert_eq!(graph.nodes.get(&node0).unwrap().outgoing_connections_type, graph.nodes.get(&node1).unwrap().incoming_connections_type);
    println!("{:#?}", graph);
    }

}


