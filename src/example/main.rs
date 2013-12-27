#[ desc = "Calm data structure library showcase" ];
#[ license = "MIT" ];

#[ crate_type = "bin" ];

extern mod calm;

use calm::tree::binary::MutableTree;
use calm::tree::binary::PrintableTree;
use calm::tree::binary::BinarySearchTree;


fn main() {
    let mut a = BinarySearchTree::<int,int>::init();
    a.print();
    a.insert(5,0);
    a.insert(3,0);
    a.insert(10,0);
    a.insert(1,0);
    a.insert(6,0);
    a.insert(9,0);
    a.insert(8,0);
    a.insert(2,0);
    a.insert(14,0);
    a.insert(20,0);
    a.insert(15,0);
    a.insert(5,0);
    a.print();
    println("Hello World");
}
