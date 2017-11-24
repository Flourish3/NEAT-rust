use std::fmt;


pub struct Node{
    name : String,
}

impl Node{
    pub fn new(name : String) -> Node{
        Node{name}
    }

    pub fn produce_output(input: f64) -> f64{
        (2_f64 / (1_f64 + (-1_f64 * input).exp())) - 1_f64
    }
}

impl fmt::Display for Node{
    fn fmt(&self, f : &mut fmt::Formatter ) -> fmt::Result{
        write!(f, "{}", self.name)
    }
}