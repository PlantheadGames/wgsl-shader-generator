
/// This will be the file containung the graph data structure. 
/// 
use std::collections::HashMap;

type NodeId = usize;

#[derive(Debug)]
struct Graph<T>{
    nodes: HashMap<NodeId, ShaderNode<T>>,
    incoming: HashMap<NodeId, Vec<NodeId>>,
    outgoing: HashMap<NodeId, Vec<NodeId>>,
    next_id: NodeId,
}


#[derive(Debug)]
struct ShaderNode<T>{
    id: NodeId,
    data: T,
}

impl<T> for Graph<T> {
    fn new() -> Self {
        Self {
            nodes: HashMap::new(),
            incoming: HashMap::new(),
            outgoing: HashMap::new(),
        }
    }
    fn add_node(&mut self, data: T) -> NodeId{
        let id = self.next_id;
        self.next_id += 1;
        self.nodes.insert(id, Node {id, data});
        self.incoming.insert(id, Vec::new());
        self.outgoing.insert(id, Vec::new());
        id
    }
    fn add_edge(&mut self, from: NodeId, to: NodeId) {
        self.outgoing.get_mut(&from).unwrap().push(to);
        self.incoming.get_mut(&to).unwrap().push(from);
    }

    fn get_inputs(&self, node: NodeId) -> Option<&Vec<NodeId>> {
        self.incoming.get(&node)
    }

    fn get_outputs(&self, node: NodeId) -> Option<&Vec<NodeId>> {
        self.outgoing.get(&node)
    }
}


