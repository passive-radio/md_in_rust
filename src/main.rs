// mani.rs
mod variables;
mod observer;
mod system;
mod md;
mod utils;

use variables::{Atom, Variables, VariablesMD};
use observer::{ObserverMD};
use md::{MD};


use crate::{observer::Observer, md::MD_blueprint};


fn main() {
    
    let mut vars = VariablesMD {time: 0.0, atoms: Vec::<Atom>::new()};
    vars.init_time();
    // vars.add_atoms(1.0, 0.0, 0.0, 0.0, 0.0, 0.0);
    // vars.add_atoms(0.0, 2.0, 3.0, 1.0, 2.0, 3.0);

    // vars.set_initial_velocity(1.0);
    
    let observer = ObserverMD {
        CUTOFF: 2.0,
        L: 10,
        // dt: 0.01,
    };

    let mut md = MD {
        dt: 0.001,
        observer: observer,
        state: false,
        time: 0.0,
        vars: vars,
        k: 0.0,
        v: 0.0,
        STEPS: 10000,
        OBSERVE: 100,
        cor_file: "temp/format.pdb".to_string(),
        potential_traj_file: "out7.csv".to_string(),
        cor_results_dir: "outputs/0".to_string(),
        margin_length: 0.5,
        pairs: Vec::new(),
    };

    // md.makeconf("fcc".to_string());
    md.run();
    let num = md.number_of_atoms();

    println!("kinetic energy: {:?}", md.k,);
    println!("potential energy: {:?}", md.v);

}
