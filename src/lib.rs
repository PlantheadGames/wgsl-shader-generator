mod graph;
pub use graph::*;


#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    pub fn test_add(){
        assert_eq!(1 + 1, 2);
    }
#[test]
    pub fn test_inital_graph_generation(){
        let mut graph = graph::Graph::new();
        let node0 = graph.add_node("this is a node", Datatype::Str);
        let node1 = graph.add_node("this is a second node" , Datatype::Str);
        //    let node3 = graph.add_node(1);
        graph.add_edge(*graph.nodes.get(&node0).unwrap(),*graph.nodes.get(&node1).unwrap());
        graph.get_inputs(node1);
        graph.get_outputs(node0);
        assert_eq!(graph.nodes.get(&node0).unwrap().outgoing_connections_type, graph.nodes.get(&node1).unwrap().incoming_connections_type);
    println!("{:#?}", graph);
    }
}
