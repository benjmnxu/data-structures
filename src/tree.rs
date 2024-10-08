#[allow(unused_imports)]
use std::{cmp::Ord, mem};

#[derive(Clone, Debug)]
pub enum TreeNode<T: Ord> {
    Leaf,
    Node(T, Box<TreeNode<T>>, Box<TreeNode<T>>),
}

// Provided functions
impl<T: Ord> TreeNode<T> {
    pub fn height(&self) -> usize {
        match self {
            TreeNode::Leaf => 0,
            TreeNode::Node(_, left, right) => 1 + std::cmp::max(left.height(), right.height()),
        }
    }

    /// Verifies that the tree is a binary search tree
    fn is_bst(&self) -> bool {
        fn is_bst_helper<T: Ord>(tree: &TreeNode<T>, min: Option<&T>, max: Option<&T>) -> bool {
            match tree {
                TreeNode::Leaf => true,
                TreeNode::Node(value, left, right) => {
                    match min {
                        Some(min) => {
                            if value <= min {
                                return false;
                            }
                        }
                        _ => {}
                    }
                    match max {
                        Some(max) => {
                            if value >= max {
                                return false;
                            }
                        }
                        _ => {}
                    }
                    is_bst_helper(left, min, Some(value)) && is_bst_helper(right, Some(value), max)
                }
            }
        }
        is_bst_helper(self, None, None)
    }

    /// Verifies that the tree is balanced
    pub fn is_balanced(&self) -> bool {
        match self {
            TreeNode::Leaf => true,
            TreeNode::Node(_, left, right) => {
                let left_height = left.height();
                let right_height = right.height();
                let diff = (left_height as i32 - right_height as i32).abs();
                diff <= 1 && left.is_balanced() && right.is_balanced()
            }
        }
    }

    /// Verifies that the tree is a valid balanced binary search tree
    pub fn validate(&self) -> bool {
        self.is_bst() && self.is_balanced()
    }
}

// Required functions
impl<T: Ord> TreeNode<T> {
    /// Creates a new `TreeNode<T>` with value `value` and children `left` and `right`
    pub fn node(value: T, left: TreeNode<T>, right: TreeNode<T>) -> TreeNode<T> {
        TreeNode::Node(value, Box::new(left), Box::new(right))
    }

    /// Creates a new `TreeNode<T>` with no children
    pub fn new() -> TreeNode<T> {
        TreeNode::default()
    }

    /// Inserts a new node with value `value` into the tree. If the value already exists in the tree,
    /// the function does nothing.
    ///
    /// After insertion, the tree is rebalanced if necessary
    pub fn insert(&mut self, value: T) {
        match self {
            TreeNode::Leaf => {
                *self = TreeNode::Node(value, Box::new(TreeNode::Leaf), Box::new(TreeNode::Leaf));
            }
            TreeNode::Node(current, left, right) => {
                if *current < value {
                    right.insert(value);
                } else if *current > value {
                    left.insert(value);
                }
            }
        }
        if !self.is_balanced() {
            self.rebalance();
        }
    }

    /// Computes the balance factor of the tree (the difference between the height of the left and right subtrees)
    fn balance_factor(&self) -> i32 {
        match self {
            TreeNode::Leaf => 0,
            TreeNode::Node(_, left, right) => left.height() as i32 - right.height() as i32,
        }
    }

    /// Performs a left rotation on the tree
    pub fn left_rotate(&mut self) {
        if let TreeNode::Node(value, left, right) = self {
            if let TreeNode::Node(rvalue, rleft, rright) = *mem::take(right) {
                **left = TreeNode::Node(mem::replace(value, rvalue), mem::take(left), rleft);
                *right = rright;
            }
        }
    }
    /// Performs a right rotation on the tree
    pub fn right_rotate(&mut self) {
        if let TreeNode::Node(value, left, right) = self {
            if let TreeNode::Node(lvalue, lleft, lright) = *mem::take(left) {
                **right = TreeNode::Node(mem::replace(value, lvalue), lright, mem::take(right));
                *left = lleft;
            }
        }
    }

    /// Rebalances the tree using either a single or double rotation, as specified in the AVL tree
    /// rebalancing algorithm.
    fn rebalance(&mut self) {
        let bf = self.balance_factor();
        if bf > 1 {
            if let TreeNode::Node(v, l, r) = self {
                if l.balance_factor() < 0 {
                    l.left_rotate();
                }
            }
            self.right_rotate();
        } else if bf < -1 {
            if let TreeNode::Node(v, l, r) = self {
                if r.balance_factor() > 0 {
                    r.right_rotate();
                }
            }
            self.left_rotate();
        }
    }
}

// Implement `Default` for `TreeNode<T>`
impl<T: Ord> Default for TreeNode<T> {
    fn default() -> Self {
        TreeNode::Leaf
    }
}

// Implement `PartialEq` for `TreeNode<T>`
impl<T: Ord + PartialEq> PartialEq for TreeNode<T> {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (TreeNode::Leaf, TreeNode::Leaf) => true,
            (TreeNode::Node(v1, n1, n2), TreeNode::Node(v2, n3, n4)) => {
                v1 == v2 && n1 == n3 && n2 == n4
            }
            _ => false,
        }
    }
}

// Implement `Eq` for `TreeNode<T>`
impl<T: Ord + Eq> Eq for TreeNode<T> {}

// Implement `From<Vec<T>>` for `TreeNode<T>`
impl<T: Ord> From<Vec<T>> for TreeNode<T> {
    fn from(vec: Vec<T>) -> TreeNode<T> {
        let mut tree = TreeNode::new();
        for elem in vec {
            tree.insert(elem);
        }
        tree
    }
}

// Implement `From<TreeNode<T>>` for `Vec<T>`
impl<T: Ord> From<TreeNode<T>> for Vec<T> {
    fn from(tree_node: TreeNode<T>) -> Vec<T> {
        match tree_node {
            TreeNode::Leaf => Vec::new(),
            TreeNode::Node(value, left, right) => {
                let mut vec = Self::from(*left);
                vec.push(value);
                vec.append(&mut Self::from(*right));
                vec
            }
        }
    }
}
