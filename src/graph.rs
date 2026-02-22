
/// This will be the file containing the graph data structure. 
/// 
use std::collections::HashMap;

type NodeId = usize;

///This struct will be the end graph structure showing all the links between the ingoing and outgoing ports on the node. 

#[derive(Debug)]
struct Graph<T>{
    nodes: HashMap<NodeId, ShaderNode<T>>,
    incoming: HashMap<NodeId, Vec<NodeId>>,
    outgoing: HashMap<NodeId, Vec<NodeId>>,
    next_id: NodeId,
}

///This is the nodes themselves. this will be a more generic structure that the overall nodes will
///conform. each node will store its own state that then gets passed up to the graph. 
#[derive(Debug)]
struct ShaderNode<T>{
    id: NodeId,
    data: T,
}

///The datatype enum will be used for the core type logic of the nodes. This will determine the
///input or outputs of the nodes are valid.
#[derive(Debug)]
enum Datatype{
    Float,
    Vec2,
    Vec3,
    Vec4,
    Mat4
}

///Define the node itself to a given type. This may be changed depending on the number of nodes to
///
#[derive(Debug)]
enum NodeKind{
    Add,
    Multiply,
    Divide,

}

impl<T> for Graph<T> {
    fn new() -> Self {
        Self {
            nodes: HashMap::new(),
            incoming: HashMap::new(),
            outgoing: HashMap::new(),
            next_id: 0
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


