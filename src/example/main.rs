#[ desc = "Calm data structure library showcase" ];
#[ license = "MIT" ];

#[ crate_type = "bin" ];

extern mod calm;

use calm::tree::binary::MutableTree;
use calm::tree::binary::BinarySearchTree;


fn main() {
    let mut a = BinarySearchTree::<int,int>::init();

    a.insert(5,0);
    a.insert(3,0);
    a.insert(7,0);
    a.insert(1,0);
    a.insert(6,0);
    a.insert(20,0);
    a.root.unwrap().print();
    println("Hello World");
}
