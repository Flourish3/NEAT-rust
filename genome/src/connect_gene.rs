use node::Node;
use std::fmt;

pub struct ConnectGene{
    input       : Node,
    output      : Node,
    weight      : f64,
    innovation  : u32,
    enabled     : bool, 
}

impl ConnectGene{
    pub fn new(input : Node, output : Node, innovation : u32) -> ConnectGene{
        ConnectGene{
            input,
            output,
            weight : 1.0,
            innovation,
            enabled : true,
        }
    }
}

impl fmt::Display for ConnectGene{
    fn fmt(&self, f : &mut fmt::Formatter) -> fmt::Result{
        write!(f, "Input Node: {}, Output Node: {}, weight: {}, enabled: {} & innovation: {}", self.input, self.output, self.weight, self.enabled, self.innovation)
    }
}

