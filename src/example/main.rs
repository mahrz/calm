#[ desc = "Calm data structure library showcase" ];
#[ license = "MIT" ];

#[ crate_type = "bin" ];

extern mod calm;

use calm::tree::binary::BinarySearchTree;

fn main() {
    let mut a = BinarySearchTree::<int,int>::init();
    println("Hello World");
}
