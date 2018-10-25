#![feature(extern_prelude)]

pub mod graph;

use graph::Node;

/// find the nearest nodes amond all the nodes that remains in the Vec of nodes
/// of the graph
fn find_nearest_node<'a>(nodes: &'a Vec<Node<'a>>) -> Option<&'a Node<'a>> {
    nodes.iter().min()
}

/// remove a node from the vec of nodes of graph.
/// The node is found by its identifier.
fn remove_node(nodes: &mut Vec<Node>, node_to_remove: &Node) {
    let position = nodes
        .iter()
        .position(|node| node.get_id() == node_to_remove.get_id());

    if let Some(index) = position {
        nodes.remove(index);
    }
}

/// update all next nodes of the given node (`node`).
/// - update the cumulative value
/// - update its predecessor
fn update_next_node(next_node_id: &str, node: &mut Node) {
    let mut transition = node.get_next_mut(next_node_id);
    if let Some(trans) = transition {
        let mut next = trans.get_next_node();
        let distance = trans.get_weight();

        if next.get_value() > node.get_value() + distance {
            next.set_value( node.get_value() + distance);
        }
    }
}

#[cfg(test)]
mod tests {

    use super::find_nearest_node;
    use super::graph::Node;
    use super::remove_node;

    #[test]
    fn should_find_the_nearest_node_given_no_node() {
        let nodes = vec![];
        // call
        let actual = find_nearest_node(&nodes);
        // assertions
        assert_eq!(actual, None);
    }

    #[test]
    fn should_find_the_nearest_node_given_one_node() {
        let node1 = Node::new("node1".to_string());
        let nodes = vec![node1];
        // call
        let actual = find_nearest_node(&nodes);
        // assertions
        assert_eq!(actual.unwrap().get_id(), "node1");
    }
    #[test]
    fn should_find_the_nearest_node_given_3_node() {
        let mut node1 = Node::new("node1".to_string());
        node1.set_value(45);
        let mut node2 = Node::new("node2".to_string());
        node2.set_value(95);
        let mut node3 = Node::new("node3".to_string());
        node3.set_value(10);

        let nodes = vec![node1, node2, node3];
        // call
        let actual = find_nearest_node(&nodes);
        // assertions
        assert_eq!(actual.unwrap().get_id(), "node3");
    }
    #[test]
    fn should_remove_a_node_node_given_3_node() {
        let node1 = Node::new("node1".to_string());
        let node2 = Node::new("node2".to_string());
        let node3 = Node::new("node3".to_string());

        let node_to_remove = Node::new("node1".to_string());

        let mut nodes = vec![node1, node2, node3];
        // call
        remove_node(&mut nodes, &node_to_remove);
        // assertions
        let node2 = Node::new("node2".to_string());
        let node3 = Node::new("node3".to_string());
        assert_eq!(nodes, vec![node2, node3]);
    }
}
