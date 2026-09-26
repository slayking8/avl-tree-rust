use std::cmp;
use std::mem;

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
        self.balance(key);
    }

    fn delete_two_children_parrent(&mut self) {
        let Some(root) = self.root.as_mut() else {
            return;
        };
        let Some(right_tree) = root.right.as_mut() else {
            return;
        };
        // optional: returning early avoids unnecessary comparisons.
        if right_tree.root.is_none() {
            return;
        };

        let mut successor = right_tree.in_order_successor();
        if successor.is_none() {
            let mut temp = root.right.take();
            if let Some(temp_tree) = temp.as_mut() {
                if let Some(temp_tree_root) = temp_tree.root.as_mut() {
                    temp_tree_root.left = root.left.take();
                    mem::swap(root, temp_tree_root);
                }
            }
        } else {
            if let Some(successor_tree) = successor.as_mut() {
                if let Some(successor_tree_root) = successor_tree.root.as_mut() {
                    successor_tree_root.left = root.left.take();
                    successor_tree_root.right = root.right.take();
                    mem::swap(root, successor_tree_root);
                }
            }
        }
    }

    fn in_order_successor(&mut self) -> Option<Box<Tree>> {
        let Some(root) = self.root.as_mut() else {
            return None;
        };
        let Some(left_tree) = root.left.as_mut() else {
            return None;
        };

        if let Some(left_tree_root) = left_tree.root.as_mut() {
            if left_tree_root.left.is_some() {
                return left_tree.in_order_successor();
            }

            let mut successor = root.left.take();
            if let Some(tree) = successor.as_mut() {
                if let Some(tree_root) = tree.root.as_mut() {
                    root.left = tree_root.right.take();
                }
            }
            return successor;
        }

        return None;
    }

    fn delete(&mut self, key: i32) {
        let Some(root) = self.root.as_mut() else {
            return;
        };

        if key < root.data {
            let Some(left_node) = root.left.as_mut() else {
                return;
            };
            let Some(left_node_root) = left_node.root.as_mut() else {
                return;
            };

            if left_node_root.data == key {
                match (left_node_root.left.as_ref(), left_node_root.right.as_ref()) {
                    (None, None) => root.left = None,
                    (_, None) => root.left = left_node_root.left.take(),
                    (None, _) => root.left = left_node_root.right.take(),
                    (_, _) => {
                        left_node.delete_two_children_parrent();
                    }
                }
            } else {
                left_node.delete(key);
            }
        } else if key > root.data {
            let Some(right_node) = root.right.as_mut() else {
                return;
            };
            let Some(right_node_root) = right_node.root.as_mut() else {
                return;
            };

            if right_node_root.data == key {
                match (
                    right_node_root.left.as_ref(),
                    right_node_root.right.as_ref(),
                ) {
                    (None, None) => root.right = None,
                    (_, None) => root.right = right_node_root.left.take(),
                    (None, _) => root.right = right_node_root.right.take(),
                    (_, _) => right_node.delete_two_children_parrent(),
                }
            } else {
                right_node.delete(key);
            }
        } else {
            self.delete_two_children_parrent();
        }
        self.balance(key);
    }

    fn balance(&mut self, key: i32) {
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

        let mut rrc = root.right.take(); // rrc == root right child
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

        let mut rlc = root.left.take(); // rlc == root left child
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
    tree.insert("Ronaldo");
    tree.insert("Miral");
    tree.insert("Paulo");
    /*
        tree.insert(9);
        tree.insert(15);
        tree.insert(12);
        tree.insert(20);
    */

    tree.delete(9);
    /*
        println!("{:?}", tree);
        println!();
        println!();
        tree.delete(9);
        println!();
        println!();
        println!("{:?}", tree);
    */
}
