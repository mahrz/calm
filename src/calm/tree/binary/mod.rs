use std::fmt;
use std::num::max;

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

impl<K, V, T, TAttr> Node<K, V, T, TAttr> {
    pub fn max_depth(&self) -> uint {
        max(match self.left  { Some(ref left) => left.max_depth() + 1, None => 0},
            match self.right { Some(ref right) => right.max_depth() + 1, None => 0})
    }
}

impl<K:fmt::Default, V:fmt::Default, T, TAttr> Node<K, V, T, TAttr> {
    pub fn print(&self) {
        let mut nds:~[&Node<K, V, T, TAttr>] = ~[self];
        let mut new_nds:~[&Node<K, V, T, TAttr>] = ~[];
        let mut offsets = ~[0];
        let mut new_offsets:~[int] = ~[];
        while !nds.is_empty() {
            let mut cur_offset = 0;
            while !nds.is_empty() {
                let head = nds.shift();
                let head_offset:int = offsets.shift();
                let l:int = head.left_width() as int;
                let h:int = head.node_width() as int;
                let r:int = head.right_width() as int;

                let mut lbal:int = match head.left {
                    Some(~ref left) => left.right_width() + left.node_width()/2,
                    None => 0
                } as int;

                let mut rbal:int = match head.right {
                    Some(~ref right) => right.left_width() + right.node_width()/2,
                    None => 0
                } as int;

                let mut loffset = head_offset - cur_offset + l;
                let mut roffset = r;

                for i in range(0,loffset) {
                    if i > loffset - lbal {
                        print("-")
                    } else if i == loffset - lbal {
                        print("+")
                    } else {
                        print(" ")
                    }
                }
                
                head.print_node_only();
                
                for i in range(0, roffset) {
                    if i < rbal-1 {
                        print("-")
                    } else if i == rbal-1 {
                        print("+")
                    } else {
                        print(" ")
                    }
                }
                cur_offset +=  loffset + h + roffset;

                match head.left {
                    Some(~ref left) => { 
                        new_nds.push(left);
                        new_offsets.push(head_offset);
                    },
                    None => {}
                }
                match head.right {
                    Some(~ref right) => { 
                        new_nds.push(right);
                        new_offsets.push(cur_offset - r);
                    },
                    None => {}
                }
            }
            println("");
            nds = new_nds.clone();
            offsets = new_offsets.clone();
            new_nds.clear();
            new_offsets.clear();
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
            Some(ref left) => left.node_width() + left.right_width() + left.left_width(),
            None => 0
        }
    }

    pub fn right_width(&self) -> uint {
        match self.right {
            Some(ref right) => right.node_width() + right.right_width() + right.left_width(),
            None => 0
        }
    }

    pub fn node_str(&self) -> ~str {
        format!("{}", *self)
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


