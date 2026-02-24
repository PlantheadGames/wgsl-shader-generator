use bevy::prelude::*;
/// This will be the file containing the graph data structure.
///
use std::collections::HashMap;

type NodeId = usize;

///This struct will be the end graph structure showing all the links between the ingoing and outgoing ports on the node.

#[derive(Debug)]
pub struct Graph {
    pub nodes: HashMap<NodeId, ShaderNode>,
    pub incoming_connections: HashMap<NodeId, Vec<NodeId>>,
    pub outgoing_connections: HashMap<NodeId, Vec<NodeId>>,
    pub next_id: NodeId,
}

///This is the nodes themselves. this will be a more generic structure that the overall nodes will
///conform. each node will store its own state that then gets passed up to the graph.
#[derive(Debug, Clone)]
pub struct ShaderNode {
    pub incoming_connections_type: Datatype,
    pub outgoing_connections_type: Datatype,
    pub id: NodeId,
    pub data: ShaderData,
}

///The datatype enum will be used for the core type logic of the nodes. This will determine the
///input or outputs of the nodes are valid.
#[derive(Debug, PartialEq, Clone)]
pub enum Datatype {
    Float,
    Vec2,
    Vec3,
    Mat4,
    String,
}

//// this node will hold the internal data of the shader. This will be the user defined input. for
///testing this will be a single value,
/// TO DO expand nodes have multiple data values which can be turned on or off.  
#[derive(Debug, PartialEq, Clone)]
pub enum ShaderData {
    Float(f32),
    Vec2(Vec2),
    Vec3(Vec3),
    Mat4(Mat4),
    String(String),
}
///Define the node itself to a given type. This may be changed depending on the number of nodes to
///
#[derive(Debug)]
enum _NodeKind {
    Add,
    Multiply,
    Divide,
}
impl ShaderNode {
    pub fn new(id: NodeId, data: ShaderData, incoming: Datatype, outgoing: Datatype) -> Self {
        Self {
            incoming_connections_type: incoming,
            outgoing_connections_type: outgoing,
            id,
            data,
        }
    }
}

impl Graph {
    pub fn new() -> Self {
        Self {
            nodes: HashMap::new(),
            incoming_connections: HashMap::new(),
            outgoing_connections: HashMap::new(),
            next_id: 0,
        }
    }
    pub fn add_node(&mut self, data: ShaderData, incoming: Datatype, outgoing: Datatype) -> NodeId {
        let id = self.next_id;
        self.next_id += 1;
        self.nodes
            .insert(id, ShaderNode::new(id, data, incoming, outgoing));
        self.incoming_connections.insert(id, Vec::new());
        self.outgoing_connections.insert(id, Vec::new());
        id
    }
    pub fn add_edge(&mut self, source_node: ShaderNode, destination_node: ShaderNode) {
        if source_node.outgoing_connections_type == destination_node.incoming_connections_type {
            self.outgoing_connections
                .get_mut(&source_node.id)
                .unwrap()
                .push(destination_node.id);
            self.incoming_connections
                .get_mut(&destination_node.id)
                .unwrap()
                .push(source_node.id);
        }
    }

    pub fn get_inputs(&self, node: NodeId) -> Option<&Vec<NodeId>> {
        let vec = self.incoming_connections.get(&node);
        vec
    }

    pub fn get_outputs(&self, node: NodeId) -> Option<&Vec<NodeId>> {
        let vec = self.outgoing_connections.get(&node);
        vec
    }
}
