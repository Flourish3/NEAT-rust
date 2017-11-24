

#[cfg(test)]
mod unit_test;

pub mod connect_gene;
pub mod node;

use connect_gene::ConnectGene;
use node::Node;

struct Genome{
    nodes : Vec<Node>,
    genes : Vec<ConnectGene>,
}





