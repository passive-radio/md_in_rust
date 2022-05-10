mod class;
mod variables;

use std::sync::Arc;

use crate::class::MD;
use crate::class::SimpleMD;
use variables::{Atom, Variables, VariablesMD};


fn main() {
    
    let mut vars = VariablesMD {time: 0.0, atoms: vec![Atom::init()]};
    vars.init_time();
    vars.add_atoms(0.0, 0.0, 0.0);
    

    println!("{:?}", vars.atoms);

    println!("Hello, world!");
}
