use std::fmt;
use std::num::max;
use std::cast;

#[deriving(Eq)]
struct NoAttr;

/** 
 * Binary Tree Nodes 
 *
 * Generalized node structure for binary trees
 */
pub struct Node<K, V, T, TAttr> {
    left:  Option<~Node<K, V, T, TAttr>>,
    right: Option<~Node<K, V, T, TAttr>>,
    // This is unsafe, but currently better than a complicated Rc with new_unchecked
    // Lifetime is actually given as the parent object always owns the child
    parent: Option<* mut Node<K, V, T, TAttr>>,
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

impl<K:fmt::Default, V:fmt::Default, T, TAttr:fmt::Default> Node<K, V, T, TAttr> {
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

                let loffset = head_offset - cur_offset + l;
                let roffset = r;

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

impl<K:fmt::Default, V:fmt::Default, T, TAttr:fmt::Default> fmt::Default for Node<K, V, T, TAttr> {
    fn fmt(obj: &Node<K, V, T, TAttr>, f: &mut fmt::Formatter) {
        write!(f.buf, "({},{}{})", obj.key, obj.value, obj.attribute)
    }
}

impl fmt::Default for NoAttr {
    fn fmt(obj: &NoAttr, f: &mut fmt::Formatter) {
        write!(f.buf, "")
    }
}

pub trait PrintableTree<K, V> {
    fn print(&self);
}

/** 
 * Search Tree
 *
 * Methods implemented by any binary search tree (binary search tree as in searchable binary tree)
 */
pub trait SearchTree<K, V, T, TAttr> {
    fn contains(&self, key: K) -> bool {
        !self.find_node(key).is_none()
    }

    fn find_value<'r>(&'r self, key: K) -> Option<&'r V> {
        self.find_node(key).map(|n| &n.value)
    }

    fn find_node<'r>(&'r self, key: K) -> Option<&'r Node<K, V, T, TAttr>>;
    fn traverse<'r>(&'r self, f: |&K,&V|);
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
    fn init(parent: Option<*mut Node<K,V,T,TAttr>>, key: K, value: V) -> Self;
    fn insert_in_node(&mut self, key: K, value: V);
}


pub struct BinarySearchTree<K, V> {
    root: Option<~Node<K, V, BinarySearchTree<K,V>,NoAttr>>
}

impl<K: Ord, V> BinarySearchTree<K, V> {
    pub fn init() -> BinarySearchTree<K, V> {
        return BinarySearchTree { root: None };
    }
}

impl<K:fmt::Default,V:fmt::Default> PrintableTree<K,V> for BinarySearchTree<K, V> {
    fn print(&self) {
        match self.root {
            Some(ref node) => node.print(),
            None => println("Empty tree")
        }
    }
}

impl<K: Ord + Eq, V> MutableTree<K, V> for BinarySearchTree<K, V> {
    fn insert(&mut self, key: K, value: V) -> () {
        match self.root {
            Some(ref mut node) => node.insert_in_node(key, value),
            None => { 
                self.root = Some(~(TreeNode::init(None, key, value)));
            }
        }
    }

    fn delete(&mut self, key: K) -> () {
        let mut node = self.find_node(key);
        match node {
            Some(ref mut node) => {
                if node.left.is_some() && node.right.is_some() {
                }
                else if node.left.is_some() {
                }
                else if node.right.is_some() {
                }
                else {
                    match node.parent {
                        Some(mut p) => {
                            unsafe {
                                (*p).left = None;
                            }
                        },
                        None => {}
                    }
                }
            },
            None => {}
        }
    }
}

impl<K: Ord + Eq, V> SearchTree<K, V, BinarySearchTree<K, V>, NoAttr> 
for BinarySearchTree<K, V> {
    fn find_node<'r>(&'r self, key: K) -> Option<&'r Node<K, V, BinarySearchTree<K, V>, NoAttr>> {
        let mut cur_node = &self.root;
        while !cur_node.is_none() {
            match cur_node {
                &Some(~ref node) => {
                    if node.key == key {
                        return Some(node);
                    } else if key < node.key {
                        cur_node = &node.left;
                    } else {
                        cur_node = &node.right;
                    }
                },            
                &None => {}
            }
        }
        return None;
    }
    
    fn traverse<'r>(&'r self, f: |&K,&V|) {
        let mut stack: ~[Option<&'r Node<K, V, BinarySearchTree<K, V>, NoAttr>>] = ~[];
        let mut cur_node;

        cur_node = self.root.as_borrowed();

        while !(stack.is_empty() && cur_node.is_none()) {
            match cur_node {
                Some(node) => {
                    stack.push(Some(node));
                    cur_node = node.left.as_borrowed();
                }
                None => {
                    cur_node = stack.pop();
                    f(&(*cur_node.unwrap()).key, &(*cur_node.unwrap()).value);
                    cur_node = (*cur_node.unwrap()).right.as_borrowed();
                }
            }
        }
    }
}

trait BorrowedOption<T> {
    fn as_borrowed<'r>(&'r self) -> Option<&'r T>;
}

impl<T> BorrowedOption<T> for Option<~T> {
    fn as_borrowed<'r>(&'r self) -> Option<&'r T> {
        match self {
            &Some(~ref o) => Some(o),
            &None => None
        }
    }
}

impl<K: Ord, V> TreeNode<K, V, BinarySearchTree<K, V>,NoAttr> 
for Node<K, V, BinarySearchTree<K, V>,NoAttr> {
    fn init(parent: Option<*mut Node<K, V, BinarySearchTree<K, V>,NoAttr>>, key: K, value: V) -> Node<K, V, BinarySearchTree<K,V>,NoAttr> {
        return Node::<K, V, BinarySearchTree<K, V>,NoAttr> { 
            left: None, 
            right: None,
            parent: parent,
            key: key,            
            value: value,
            attribute: NoAttr
        };
    }

    fn insert_in_node(&mut self,  key: K, value: V) {
        if key < self.key {
            match self.left {
                Some(ref mut node) => node.insert_in_node(key, value),
                None => unsafe { 
                    self.left = Some(~TreeNode::init(
                            Some(self as *mut Node<K, V, BinarySearchTree<K, V>, NoAttr>), key, value)) 
                }
            }
        } else {
            match self.right {
                Some(ref mut node) => node.insert_in_node(key, value),
                None => unsafe { 
                    self.right = Some(~TreeNode::init(
                            Some(self as *mut Node<K, V, BinarySearchTree<K, V>,NoAttr>), key, value)) 
                }
            }
        }
    }
}


