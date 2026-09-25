use std::cmp;
use std::mem;
use std::mem::swap;
use std::ops::RemAssign;

#[derive(Debug)]
struct Node {
    data: i32,
    left: Option<Box<Tree>>,
    right: Option<Box<Tree>>,
    height: isize,
}

impl Node {
    fn new(data: i32) -> Self {
        return Self {
            data: data,
            left: None,
            right: None,
            height: 0,
        };
    }
}

#[derive(Debug)]
struct Tree {
    root: Option<Box<Node>>,
}

impl Tree {
    fn new(data: Option<i32>) -> Self {
        let Some(val) = data else {
            return Self { root: None };
        };
        return Self {
            root: Some(Box::new(Node::new(val))),
        };
    }

    fn insert(&mut self, key: i32) {
        let Some(root_node) = self.root.as_mut() else {
            self.root = Some(Box::new(Node::new(key)));
            return;
        };

        if key < root_node.data {
            if let Some(left_tree) = root_node.left.as_mut() {
                left_tree.insert(key);
            } else {
                root_node.left = Some(Box::new(Tree::new(Some(key))));
            }
        } else if key > root_node.data {
            if let Some(right_tree) = root_node.right.as_mut() {
                right_tree.insert(key);
            } else {
                root_node.right = Some(Box::new(Tree::new(Some(key))));
            }
        }

        self.update_height();
        let balance = self.get_balance();
        let Some(root) = self.root.as_mut() else {
            return;
        };

        if balance < -1 {
            let Some(right_tree) = root.right.as_mut() else {
                return;
            };
            let Some(rt_root) = right_tree.root.as_mut() else {
                return;
            };

            if key > rt_root.data {
                self.left_rotation();
            } else {
                right_tree.right_rotation();
                self.left_rotation();
            }
        } else if balance > 1 {
            let Some(left_tree) = root.left.as_mut() else {
                return;
            };
            let Some(lt_root) = left_tree.root.as_mut() else {
                return;
            };

            if key < lt_root.data {
                self.right_rotation();
            } else {
                left_tree.left_rotation();
                self.right_rotation();
            }
        }
    }

    fn in_order_successor(&mut self) -> Option<Box<Tree>> {
        let root = self.root.as_mut().unwrap();
        // let right_node = root.right.as_mut().unwrap();
        let Some(right_node) = root.right.as_mut() else {
            println!("CLUES");
            return None;
        };

        if let Some(right_node_root) = right_node.root.as_mut() {
            if let Some(right_node_left_child) = right_node_root.left.as_mut() {
                let mut leftmost = right_node_left_child.leftmost_child();
                if leftmost.is_none() {
                    mem::swap(&mut right_node_root.left, &mut leftmost);
                    if let Some(leftmost_tree) = leftmost.as_mut() {
                        if let Some(leftmost_tree_root) = leftmost_tree.root.as_mut() {
                            right_node_root.left = leftmost_tree_root.right.take();
                        }
                    }
                    return leftmost;
                } else {
                    if let Some(left_child_root) = right_node_left_child.root.as_mut() {
                        if let Some(leftmost_tree) = leftmost.as_mut() {
                            if let Some(leftmost_tree_root) = leftmost_tree.root.as_mut() {
                                left_child_root.left = leftmost_tree_root.right.take();
                                leftmost_tree_root.left = root.left.take();
                                leftmost_tree_root.right = root.right.take();
                                mem::swap(root, leftmost_tree_root);
                            }
                        }
                    }
                    return leftmost;
                }
            }
        }

        println!("CLUES 2");
        return root.right.take();
    }
    // It's just helper to walk the tree recursively.
    fn leftmost_child(&mut self) -> Option<Box<Tree>> {
        let Some(root) = self.root.as_mut() else {
            return None;
        };
        let Some(left_node) = root.left.as_mut() else {
            return None;
        };

        if let Some(left_node_root) = left_node.root.as_mut() {
            if left_node_root.left.is_some() {
                return left_node.leftmost_child();
            }

            let mut successor = root.left.take();
            if let Some(tree) = successor.as_mut() {
                if let Some(tree_root) = tree.root.as_mut() {
                    root.left = tree_root.right.take();
                }
            }

            return successor;
        }

        None
    }

    fn delete(&mut self, key: i32) {
        let Some(root) = self.root.as_mut() else {
            return;
        };

        if key < root.data {
            if let Some(left_node) = root.left.as_mut() {
                if let Some(left_node_root) = left_node.root.as_mut() {
                    if left_node_root.data == key {
                        println!("LEFT: {:?}", left_node_root);
                        match (left_node_root.left.as_ref(), left_node_root.right.as_ref()) {
                            (None, None) => root.left = None,
                            (_, None) => root.left = left_node_root.left.take(),
                            (None, _) => root.left = left_node_root.right.take(),
                            (_, _) => {
                                left_node.in_order_successor();
                            }
                        }
                    } else {
                        left_node.delete(key);
                    }
                }
            }
        } else if key > root.data {
            if let Some(right_node) = root.right.as_mut() {
                if let Some(right_node_root) = right_node.root.as_mut() {
                    if right_node_root.data == key {
                        // println!("RIGHT: {:?}", right_node_root);
                        match (
                            right_node_root.left.as_ref(),
                            right_node_root.right.as_ref(),
                        ) {
                            (None, None) => {
                                root.right = None;
                                println!("NEVER");
                            }
                            (_, None) => {
                                root.right = right_node_root.left.take();
                                println!("NEVER");
                            }
                            (None, _) => {
                                root.right = right_node_root.right.take();
                                println!("NEVER");
                            }
                            (_, _) => {
                                right_node.in_order_successor();
                            }
                        }
                    } else {
                        right_node.delete(key);
                    }
                }
            }
        } else {
            println!("NEVER");
            let mut successor = self.in_order_successor();
            if let Some(successor_tree) = successor.as_mut() {
                if let Some(successor_tree_root) = successor_tree.root.as_mut() {
                    let self_root = self.root.as_mut().unwrap();

                    successor_tree_root.left = self_root.left.take();
                    mem::swap(self_root, successor_tree_root);
                }
            }
        }
    }

    fn get_balance(&self) -> isize {
        let Some(root) = self.root.as_ref() else {
            return 0;
        };

        let left_h = root
            .left
            .as_ref()
            .map(|tree| tree.root.as_ref().map(|node| node.height + 1).unwrap_or(1))
            // 1 is returned by default cuz the tree itself exists it just happens that it's empty.
            .unwrap_or(0);

        let right_h = root
            .right
            .as_ref()
            .map(|tree| tree.root.as_ref().map(|node| node.height + 1).unwrap_or(1))
            // 1 is returned by default cuz the tree itself exists it just happens that it's empty.
            .unwrap_or(0);

        left_h - right_h
    }

    fn update_height(&mut self) -> isize {
        let Some(root) = self.root.as_mut() else {
            return 0;
        };

        let left_h = root
            .left
            .as_mut()
            .map(|tree| tree.update_height() + 1)
            .unwrap_or(0);

        let right_h = root
            .right
            .as_mut()
            .map(|tree| tree.update_height() + 1)
            .unwrap_or(0);

        let height = cmp::max(left_h, right_h);
        if let Some(root) = self.root.as_mut() {
            root.height = height;
        }

        height
    }

    fn left_rotation(&mut self) {
        let Some(root) = self.root.as_mut() else {
            return;
        };

        let mut rrc = root.right.take();
        let Some(rc) = rrc.as_mut() else {
            root.right = rrc;
            return;
        };

        let Some(rc_root) = rc.root.as_mut() else {
            root.right = rrc;
            return;
        };

        let rc_lc = rc_root.left.take();
        mem::swap(root, rc_root);
        rc_root.right = rc_lc;
        root.left = rrc;

        self.update_height();
    }

    fn right_rotation(&mut self) {
        let Some(root) = self.root.as_mut() else {
            return;
        };

        let mut rlc = root.left.take();
        let Some(lc) = rlc.as_mut() else {
            root.left = rlc;
            return;
        };

        let Some(lc_root) = lc.root.as_mut() else {
            root.left = rlc;
            return;
        };

        let lc_rc = lc_root.right.take();
        mem::swap(root, lc_root);
        lc_root.left = lc_rc;
        root.right = rlc;

        self.update_height();
    }
}

fn main() {
    let mut tree = Tree::new(None);

    tree.insert(9);
    tree.insert(5);
    tree.insert(10);
    tree.insert(0);
    tree.insert(6);
    tree.insert(11);
    tree.insert(-1);
    tree.insert(1);
    tree.insert(2);

    println!("{:?}", tree);
    println!();
    println!();
    tree.delete(5);
    println!();
    println!();
    println!("{:?}", tree);

    /*
        tree.insert(10);
        tree.insert(20);
        tree.insert(30);
        tree.insert(40);
        tree.insert(50);
        tree.insert(25);
    */
}
