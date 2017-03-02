mod avl_tree;

fn main() {
    let mut n = avl_tree::Node::new(0);
    for i in 1..9 {
        n.insert_unbalanced(i);
    }
    println!("Tree:\n{}", n);
    println!("Node balance: {}", n.vertex_balance());
}
