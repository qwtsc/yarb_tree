use std::cell::RefCell;
use std::rc::Rc;

#[derive(Debug, PartialEq, Eq)]
pub struct Node<T: PartialOrd + Clone> {
    value: T,
    left: Option<Rc<RefCell<Node<T>>>>,
    right: Option<Rc<RefCell<Node<T>>>>,
    color: Color,
}

type TreeNode<T> = Option<Rc<RefCell<Node<T>>>>;

#[derive(Debug)]
pub struct RedBlackTree<T: PartialOrd + Clone> {
    root: TreeNode<T>,
    size: u64,
}

fn is_red<T: PartialOrd + Clone>(node: TreeNode<T>) -> bool {
    if let Some(node_ptr) = node {
        return node_ptr.borrow().is_red();
    }
    false
}

impl<T: PartialOrd + Clone> RedBlackTree<T> {
    /// create a new redblack_tree
    /// examples:
    /// ```
    /// use redblack::tree::RedBlackTree;
    /// let tree: RedBlackTree<u32> = RedBlackTree::new();
    /// assert_eq!(0, tree.len());
    /// ```
    pub fn new() -> Self {
        RedBlackTree {
            root: None,
            size: 0,
        }
    }

    /// insert new element, and keep the red black tree balanced by rotating the key node
    /// examples:
    /// ```
    /// use redblack::tree::RedBlackTree;
    /// let mut tree: RedBlackTree<u32> = RedBlackTree::new();
    /// tree.insert(1);
    /// tree.insert(2);
    /// assert_eq!(2, tree.len());
    /// ```
    ///
    pub fn insert(&mut self, val: T) {
        let insert_root = self.insert_help(self.root.clone(), val);
        insert_root.clone().unwrap().borrow_mut().color = Color::Black;
        self.root = insert_root;
        self.size += 1;
    }

    fn insert_help(&mut self, node: TreeNode<T>, val: T) -> TreeNode<T> {
        if let Some(mut cnode) = node.clone() {
            let mut cmp_node = cnode.borrow_mut();
            let value = cmp_node.value.clone();
            if val < value {
                cmp_node.left = self.insert_help(cmp_node.left.clone(), val)
            } else if val > value {
                cmp_node.right = self.insert_help(cmp_node.right.clone(), val)
            } else {
                cmp_node.value = val;
            }
            let left = cmp_node.left.clone();
            let right = cmp_node.right.clone();
            drop(cmp_node);
            if is_red(right.clone()) && !is_red(left.clone()) {
                // because only left node can be red, if right node become red for some reason,
                // rotate the tree.
                cnode = self.rotate_left(Some(cnode.clone())).unwrap();
            }
            if is_red(left.clone())
                && is_red(
                    left.clone()
                        .expect("shouldn't be None because of previous is_red check")
                        .borrow()
                        .left
                        .clone(),
                )
            {
                // the double left red node should not appear either, try to rotate tree to keep
                // balance.
                cnode = self.rotate_right(Some(cnode.clone())).unwrap();
            }
            if is_red(left.clone()) && is_red(right.clone()) {
                cnode.borrow_mut().flip_color();
                left.unwrap().borrow_mut().flip_color();
                right.unwrap().borrow_mut().flip_color();
            }
            return Some(cnode);
        }
        Some(Rc::new(RefCell::new(Node::with_color(val, Color::Red))))
    }

    fn rotate_left(&self, node: TreeNode<T>) -> TreeNode<T> {
        let cnode = node.clone().unwrap();
        let mut node_ptr = cnode.borrow_mut();
        let right = node_ptr.right.clone().unwrap();
        let mut right_ptr = right.borrow_mut();
        node_ptr.right = right_ptr.left.clone();
        right_ptr.left = node.clone();
        right_ptr.color = node_ptr.color.clone();
        node_ptr.color = Color::Red;
        Some(right.clone())
    }

    fn rotate_right(&self, node: TreeNode<T>) -> TreeNode<T> {
        let cnode = node.clone().unwrap();
        let mut node_ptr = cnode.borrow_mut();
        let left = node_ptr.left.clone().unwrap();
        let mut left_ptr = left.borrow_mut();
        node_ptr.left = left_ptr.right.clone();
        left_ptr.right = node.clone();
        left_ptr.color = node_ptr.color.clone();
        node_ptr.color = Color::Red;
        Some(left.clone())
    }

    pub fn contains(&self, val: T) -> bool {
        let mut cmp = self.root.clone();
        while let Some(cmp_node) = cmp {
            let value = cmp_node.borrow().value.clone();
            if value < val {
                cmp = cmp_node.borrow().right.clone();
            } else if value > val {
                cmp = cmp_node.borrow().left.clone();
            } else {
                return true;
            }
        }
        false
    }

    pub fn clear(&mut self) {
        self.root.take();
        self.size = 0;
    }

    pub fn insert_all(&mut self, vals: Vec<T>) {
        vals.into_iter().for_each(|x| self.insert(x))
    }

    pub fn delete(&self, val: T) -> bool {
        unimplemented!("not yet")
    }

    pub fn len(&self) -> u64 {
        self.size
    }
}

fn from_recursive<T: PartialOrd + Clone>(node: TreeNode<T>, res: &mut Vec<T>) {
    if let Some(node_ptr) = node {
        from_recursive(node_ptr.clone().borrow().left.clone(), res);
        res.push(node_ptr.clone().borrow().value.clone());
        from_recursive(node_ptr.clone().borrow().right.clone(), res);
    }
}

impl<T: PartialOrd + Clone> From<RedBlackTree<T>> for Vec<T> {
    fn from(tree: RedBlackTree<T>) -> Self {
        let mut res = Vec::with_capacity(tree.len() as usize);
        from_recursive(tree.root, &mut res);
        res
    }
}

impl<T: PartialOrd + Clone> Default for RedBlackTree<T> {
    fn default() -> Self {
        RedBlackTree::new()
    }
}

#[derive(Debug, Clone, PartialEq, Ord, PartialOrd, Eq)]
pub enum Color {
    Red,
    Black,
}

impl<T: PartialOrd + Clone> Node<T> {
    pub fn new(val: T) -> Node<T> {
        Node {
            value: val,
            left: None,
            right: None,
            color: Color::Black,
        }
    }

    pub fn with_color(val: T, color: Color) -> Node<T> {
        Node {
            value: val,
            left: None,
            right: None,
            color,
        }
    }

    pub fn flip_color(&mut self) {
        match self.color {
            Color::Red => self.color = Color::Black,
            Color::Black => self.color = Color::Red,
        }
    }

    pub fn is_red(&self) -> bool {
        self.color == Color::Red
    }
}

#[cfg(test)]
mod tests {
    use super::RedBlackTree;
    #[test]
    pub fn test_create_tree() {
        let tree: RedBlackTree<u32> = RedBlackTree::new();
        println!("{:?}", tree);
        assert_eq!(0, tree.len());
    }

    #[test]
    pub fn test_insert() {
        let mut tree: RedBlackTree<u32> = RedBlackTree::new();
        tree.insert(1);
        println!("{:?}", tree);
        tree.insert(2);
        println!("{:?}", tree);
        tree.insert(23);
        assert_eq!(3, tree.len());
        println!("{:?}", tree);
        tree.insert(45);
        println!("{:?}", tree);
    }

    #[test]
    pub fn test_into() {
        let mut tree: RedBlackTree<u32> = RedBlackTree::new();
        tree.insert(1);
        tree.insert(2);
        tree.insert(23);
        tree.insert(0);
        tree.insert(15);
        tree.insert(100);
        let res: Vec<u32> = tree.into();
        assert_eq!(res, vec![0, 1, 2, 15, 23, 100]);
    }

    #[test]
    pub fn test_contains() {
        let mut tree: RedBlackTree<u32> = RedBlackTree::new();
        tree.insert(1);
        tree.insert(2);
        tree.insert(23);
        tree.insert(0);
        tree.insert(15);
        tree.insert(100);
        assert_eq!(true, tree.contains(23));
    }
}
