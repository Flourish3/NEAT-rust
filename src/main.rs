extern crate genome;
use genome::connect_gene::Connect_gene;
use genome::node::Node;
fn main() {
    println!("Hello, world!");
    let out = Node::new(String::from("Output"));
    let inp = Node::new(String::from("Input"));
    let cg = Connect_gene::new(inp,out,1);

    println!("{}", cg );    
}
