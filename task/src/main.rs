use std::collections::VecDeque;

// Define the Node struct
#[derive(Debug)]
struct Node {
    id: usize,
    children: Vec<Node>,
}

impl Node {
    fn new(id: usize) -> Self {
        Node {
            id,
            children: Vec::new(),
        }
    }

    fn add_child(&mut self, child: Node) {
        self.children.push(child);
    }
}

// Function to generate the sequence of digits from Pi (after the first digit)
fn pi_digits() -> Vec<usize> {
    vec![1, 4, 1, 5, 9, 2, 6, 5, 3, 5]  // A short sequence for illustration
}

// Function to build the tree
fn build_tree() -> Node {
    let digits = pi_digits();
    let mut root = Node::new(1);
    let mut queue: VecDeque<&mut Node> = VecDeque::new();
    queue.push_back(&mut root);

    let mut current_id = 2;

    for &digit in &digits {
        let level_size = queue.len();
        for _ in 0..level_size {
            if let Some(parent) = queue.pop_front() {
                for _ in 0..digit {
                    let mut child = Node::new(current_id);
                    current_id += 1;
                    queue.push_back(&mut child);
                    parent.add_child(child);
                }
            }
        }
    }

    root
}

// Function to traverse the tree and collect node IDs
fn traverse_tree(node: &Node, result: &mut Vec<usize>) {
    result.push(node.id);
    for child in &node.children {
        traverse_tree(child, result);
    }
}

fn main() {
    let root = build_tree();
    let mut result = Vec::new();
    traverse_tree(&root, &mut result);
    println!("{:?}", result);
}

