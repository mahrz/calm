pub struct Node<K, V, T, TAttr> {
    left:  Option<~Node<K, V, T, TAttr>>,
    right: Option<~Node<K, V, T, TAttr>>,
    key: K,
    value: V,
    attribute: TAttr
}

trait SearchTree<K, V, T, TAttr> {
    fn contains(&self, key: K) -> bool;
    fn find_value<'r>(&'r self, key: K) -> Option<&'r V>;
    fn find_node<'r>(&'r self, key: K) -> Option<&'r Node<K, V, T, TAttr>>;
    fn traverse<'r>(&'r self, f: |K,V|);
}

trait MutableTree<K, V, T, TAttr> {
    fn insert(&mut self, key: K, value: V);
    fn delete(&mut self, key: K);
}

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

impl<K: Orderable, V> TreeNode<K, V, BinarySearchTree<K, V>, ()> 
for Node<K, V, BinarySearchTree<K, V>, ()> {
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
            if self.left.is_none() {
                self.left = Some(~TreeNode::init(key, value));
            }
        } else {
        }
    }
}

impl<K: Orderable, V> MutableTree<K, V, BinarySearchTree<K,V>, ()> for BinarySearchTree<K, V> {

    fn insert(&mut self, key: K, value: V) {
        match self.root {
            Some(ref mut node) => node.insert_in_node(key, value),
            None => { 
                self.root = Some(~(TreeNode::init(key,value)));
            }
        }
    }

    fn delete(&mut self, key: K) {
    }
}

