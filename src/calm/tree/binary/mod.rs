use std::fmt;

/** 
 * Binary Tree Nodes 
 *
 * Generalized node structure for binary trees
 */
pub struct Node<K, V, T, TAttr> {
    left:  Option<~Node<K, V, T, TAttr>>,
    right: Option<~Node<K, V, T, TAttr>>,
    key: K,
    value: V,
    attribute: TAttr
}

impl<K:fmt::Default, V:fmt::Default, T, TAttr> Node<K, V, T, TAttr> {
    pub fn print(&self) {
        let mut nds:~[&Node<K, V, T, TAttr>] = ~[self];
        let mut new_nds:~[&Node<K, V, T, TAttr>] = ~[];
        while !nds.is_empty() {
            while !nds.is_empty() {
                let head = nds.shift();
                let l = head.left_width();
                for i in range(0, l) {
                    print(" ")
                }
                head.print_node_only();
                let r = self.right_width();
                for i in range(0, l) {
                    print(" ")
                }
                match head.left {
                    Some(ref left) => new_nds.push(left),
                    None => {}
                }
                match head.right {
                    Some(ref right) => new_nds.push(right),
                    None => {}
                }

            }
            nds = new_nds.clone();
            new_nds.clear();
        }
    }

    pub fn print_node_only(&self) {
        print(self.node_str())
    }

    pub fn node_width(&self) -> uint {
        self.node_str().len()
    }

    pub fn left_width(&self) -> uint {
        match self.left {
            Some(ref left) => left.node_width() + left.left_width(),
            None => 0
        }
    }

    pub fn right_width(&self) -> uint {
        match self.right {
            Some(ref right) => right.node_width() + right.left_width(),
            None => 0
        }
    }


    pub fn node_str(&self) -> ~str {
        format!(" {} ", *self)
    } 
}

impl<K:fmt::Default, V:fmt::Default, T, TAttr> fmt::Default for Node<K, V, T, TAttr> {
    fn fmt(obj: &Node<K, V, T, TAttr>, f: &mut fmt::Formatter) {
        write!(f.buf, "({},{})", obj.key, obj.value)
    }
}

/** 
 * Search Tree
 *
 * Methods implemented by any binary search tree (binary search tree as in searchable binary tree)
 */
pub trait SearchTree<K, V, T, TAttr> {
    fn contains(&self, key: K) -> bool;
    fn find_value<'r>(&'r self, key: K) -> Option<&'r V>;
    fn find_node<'r>(&'r self, key: K) -> Option<&'r Node<K, V, T, TAttr>>;
    fn traverse<'r>(&'r self, f: |K,V|);
}

/** 
 * Mutable Tree
 * 
 * Methods implemented by a mutable tree
 */
pub trait MutableTree<K, V> {
    fn insert(&mut self, key: K, value: V) -> ();
    fn delete(&mut self, key: K) -> ();
}

/** 
 * Tree Node
 * 
 * Trait that is implemented by specific node implementations depending on the tree
 */
trait TreeNode<K, V, T, TAttr> {
    fn init(key: K, value: V) -> Self;
    fn insert_in_node(&mut self, key: K, value: V);
}

pub struct BinarySearchTree<K, V> {
    root: Option<~Node<K, V, BinarySearchTree<K,V>, ()>>
}

impl<K: Orderable, V> BinarySearchTree<K, V> {
    pub fn init() -> BinarySearchTree<K, V> {
        return BinarySearchTree { root: None };
    }
}

impl<K: Orderable, V> MutableTree<K, V> for BinarySearchTree<K, V> {

    fn insert(&mut self, key: K, value: V) -> () {
        match self.root {
            Some(ref mut node) => node.insert_in_node(key, value),
            None => { 
                self.root = Some(~(TreeNode::init(key,value)));
            }
        }
    }

    fn delete(&mut self, key: K) -> () {
    }
}


impl<K: Orderable, V> TreeNode<K, V, BinarySearchTree<K, V>, ()> for Node<K, V, BinarySearchTree<K, V>, ()> {
    fn init(key: K, value: V) -> Node<K, V, BinarySearchTree<K,V>, ()> {
        return Node::<K, V, BinarySearchTree<K, V>, ()> { 
            left: None, 
            right: None,
            key: key,            
            value: value,
            attribute: ()
        };
    }

    fn insert_in_node(&mut self,  key: K, value: V) {
        if key < self.key {
            match self.left {
                Some(ref mut node) => node.insert_in_node(key, value),
                None => { self.left =  Some(~TreeNode::init(key, value)) }
            }
        } else {
            match self.right {
                Some(ref mut node) => node.insert_in_node(key, value),
                None => { self.right =  Some(~TreeNode::init(key, value)) }
            }
        }
    }
}


