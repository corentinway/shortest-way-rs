use std::collections::HashMap;

#[derive(Debug, Clone)]
pub struct Node<'a> {
    /// unique identifier
    id: String,
    /// offset for the weight of the node when a next node is added
    offset: i32,
    /// value to compute the shortest path : distance or running time
    value: i32,
    /// next nodes
    next: HashMap<String, Transition<'a>>,
}

impl<'a> Node<'a> {
    pub fn new(identifier: String) -> Self {
        Node {
            id: identifier,
            offset: 0,
            value: std::i32::MAX,
            next: HashMap::new(),
        }
    }
    pub fn with_offset(identifier: String, offset: i32) -> Self {
        Node {
            id: identifier,
            offset: offset,
            value: std::i32::MAX,
            next: HashMap::new(),
        }
    }
    pub fn add_next(&mut self, next_node: &'a Node, weight: i32) {
        let transition = Transition::new(&next_node, weight, self.offset);

        self.next.insert(next_node.id.clone(), transition);
    }
    pub fn get_id(&self) -> &str {
        &self.id
    }
    pub fn set_value(&mut self, value: i32) {
        self.value = value;
    }
}

use std::cmp::Ordering;

impl<'a> Ord for Node<'a> {
    fn cmp(&self, other: &Node) -> Ordering {
        self.value.cmp(&other.value)
    }
}

impl<'a> PartialOrd for Node<'a> {
    fn partial_cmp(&self, other: &Node) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl<'a> PartialEq for Node<'a> {
    fn eq(&self, other: &Node) -> bool {
        self.value == other.value
    }
}
impl<'a> Eq for Node<'a> {}

/// This is a transition between 2 nodes
#[derive(Debug, PartialEq, Clone)]
struct Transition<'a> {
    node: &'a Node<'a>,
    weight: i32,
}

impl<'a> Transition<'a> {
    pub fn new(node: &'a Node, weight: i32, offset: i32) -> Transition<'a> {
        Transition {
            node: &node,
            weight: weight + offset,
        }
    }
}

#[cfg(test)]
mod tests {

    use super::Node;
    use super::Transition;
    use std::collections::HashMap;

    #[test]
    fn should_create_on_node_with_default_values() {
        let id = String::from("123");
        // call
        let node = Node::new(id);
        // assertions
        assert_eq!(
            Node {
                id: "123".to_string(),
                offset: 0,
                value: std::i32::MAX,
                next: HashMap::new(),
            },
            node
        );
    }

    #[test]
    fn should_create_a_node_with_an_offset() {
        let id = String::from("123");
        let offset = 32;
        // call
        let node = Node::with_offset(id, offset);
        // assertions
        assert_eq!(
            Node {
                id: "123".to_string(),
                offset: 32,
                value: std::i32::MAX,
                next: HashMap::new(),
            },
            node
        );
    }

    #[test]
    fn should_create_a_transition() {
        let node = Node::new("node1".to_string());
        let weight = 10;
        let offset = 33;
        // call
        let transition = Transition::new(&node, weight, offset);
        // assertions
        assert_eq!(
            Transition {
                node: &node,
                weight: 43,
            },
            transition
        );
    }
    #[test]
    fn should_add_a_next_node() {
        let node2 = Node::new("node2".to_string());
        let mut node1 = Node::new("node1".to_string());
        let weight: i32 = 10;

        // call
        node1.add_next(&node2, weight);

        // assertions
        let mut next = HashMap::new();
        let transition = Transition {
            node: &node2,
            weight: 10 + 0,
        };
        next.insert(String::from("node2"), transition);

        assert_eq!(
            Node {
                id: "node1".to_string(),
                offset: 0,
                value: std::i32::MAX,
                next: next,
            },
            node1
        );
    }
}
