//use connect_gene::Connect_gene;
use node::Node;

#[test]
fn test_node_output_0(){
    assert_eq!(0_f64, Node::produce_output(0_f64));
}

#[test]
fn test_node_output_positive(){
    assert!(Node::produce_output(5_f64) > 0_f64);
}

#[test]
fn test_node_output_negative(){
    assert!(Node::produce_output(-5_f64) < 0_f64);
}