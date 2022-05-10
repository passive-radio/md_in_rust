// mani.rs
mod variables;
mod observer;
mod system;

// use crate::class::MD;
// use crate::class::SimpleMD;
use variables::{Atom, Variables, VariablesMD};
use observer::{ObserverMD};

use crate::observer::Observer;


fn main() {
    
    let mut vars = VariablesMD {time: 0.0, atoms: vec![Atom::init()]};
    vars.init_time();
    vars.add_atoms(1.0, 0.0, 0.0, 0.0, 0.0, 0.0);
    vars.add_atoms(0.0, 2.0, 3.0, 1.0, 2.0, 3.0);

    vars.set_initial_velocity(1.0);
    
    let observer = ObserverMD {
        CUTOFF: 1.0,
        L: 10,
        dt: 0.1,
    };

    let kinetic = observer.kinetic_energy(&vars);

    println!("{:?}", vars.atoms);
    println!("len of atoms (Vec<Atom>): {:?}", vars.atoms.len());
    println!("kinetic energy: {:?}", &kinetic);

}
