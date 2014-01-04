#[desc = "Calm data structure library showcase"];
#[license = "MIT"];

#[crate_type = "bin"];
#[feature(globs)];

extern mod calm;


use calm::tree::binary::*;


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
    println(format!("Contains 20? {}", *a.find_node(20).unwrap()));
    a.traverse(|k,v| print(format!("({},{})", *k, *v)));
    let b = a.find_node(20);
    unsafe {
        println(format!("{}", (*(b.unwrap().parent.unwrap())).key));
    }    
    println("Hello World");
}
