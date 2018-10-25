#![feature(extern_prelude)]

pub mod graph;

use graph::Node;

fn find_nearest_node<'a>(nodes: &'a Vec<Node<'a>>) -> Option<&'a Node<'a>> {
    nodes.iter().min()
}

fn remove_node(nodes: &mut Vec<Node>, node_to_remove: &Node) {
    let position = nodes
        .iter()
        .position(|node| node.get_id() == node_to_remove.get_id());

    if let Some(index) = position {
        nodes.remove(index);
    }
}

#[cfg(test)]
mod tests {

    use super::find_nearest_node;
    use super::remove_node;
    use super::graph::Node;

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
        let  node1 = Node::new("node1".to_string());
        let  node2 = Node::new("node2".to_string());
        let  node3 = Node::new("node3".to_string());

        let node_to_remove = node1.clone();

        let mut nodes = vec![node1, node2, node3];
        // call
        remove_node(&mut nodes, &node_to_remove);
        // assertions
        let  node2 = Node::new("node2".to_string());
        let  node3 = Node::new("node3".to_string());
        assert_eq!(nodes, vec![node2, node3]);
    }
}
