pub mod selector;
pub mod serialize;
pub mod tree;
pub mod tree_sink;

pub use tree::{DomTree, NodeData, NodeId};
pub use tree_sink::{parse_fragment, parse_html};
