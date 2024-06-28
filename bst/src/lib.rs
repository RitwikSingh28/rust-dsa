struct Tree {
    root: Option<Box<Node>>,
}

impl Tree {
    fn new() -> Self {
        Tree { root: None }
    }

    fn insert(&mut self, value: i32) {
        match &mut self.root {
            None => {
                self.root = Some(Node::new(value));
            },
            Some(node) => {
                Tree::recursive_insert(node, value);
            }
        }
    }

    fn recursive_insert(node: &mut Box<Node>, value: i32) {
        if node.value < value {
            match &mut node.left {
                None => {
                    node.left = Some(Node::new(value));
                }, 
                Some(node) => {
                    Tree::recursive_insert(node, value);
                }
            }
        } else {
            match &mut node.right {
                None => {
                    node.right = Some(Node::new(value));
                },
                Some(node) => {
                    Tree::recursive_insert(node, value);
                }
            }
        }
    }
}

struct Node {
    value: i32,
    left: Option<Box<Node>>,
    right: Option<Box<Node>>,
}

impl Node {
    fn new(value: i32) -> Box<Node> {
        Box::new(Node {
            value,
            left: None,
            right: None,
        })
    }
}

