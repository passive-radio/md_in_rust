use crate::variables::{Variables, VariablesMD};


pub trait Observer {
    pub fn kinetic_energy(vars: Variables) -> f64;
    pub fn potential_energy(vars: Variables) -> f64;
    pub fn tempareture(vars: Variables) -> f64;
    pub fn total_energy(vars: Variables) -> f64;
}

impl Observer {
    pub fn kinetic_energy(vars: Variables) -> f64;
    pub fn potential_energy(vars: Variables) -> f64;
    pub fn tempareture(vars: Variables) -> f64 {
        kinetic_energy(vars) / 1.5
    }
    pub fn total_energy(vars: Variables) -> f64 {
        kinetic_energy(vars) + potential_energy(vars)
    }
}

pub struct ObserverMD {
    /// how to use
    /// let observer = ObserverMD {
    ///     CUTOFF: 2.0,
    ///     L: 10,
    ///     dt: 0.01,
    /// }
    /// observer.kinetic_energy(vars)
    
    CUTOFF: f64,
    L: i32,
    dt: f32,
    
}


impl Observer for ObserverMD {
    fn kinetic_energy(vars: Variables) -> f64 {
        let k: i32 = 0;

        for n in 0..(vars.atoms.len() - 1 as usize) {
            k += vars.atoms[n].px * vars.atoms[n].px;
            k += vars.atoms[n].py * vars.atoms[n].py;
            k += vars.atoms[n].pz * vars.atoms[n].pz;
        }
        k /= vars.number_of_atoms;
        k
    } 

    fn potential_energy(&self, vars: Variables) -> f64 {
        let v: f64 = 0.0;
        const CL2: f64 = self.CUTOFF*self.CUTOFF;
        const RC2: f64 = 
        
        // for n in 0..(vars.atoms.len() -1 as usize) {
        //     for m in n..(vars.atoms.len() as usize) {
        //         // dx: f64 = 
        //     }
        // }

        for (i, atom_i) in vars.atoms[0..].iter_mut().enumerate() {
            for (j, atom_j) in vars.atoms[i..].iter_mut().enuerate() {
                let dx: f64 = atom_j.qx - atom_i.qx;
                let dy: f64 = atom_j.qy - atom_i.qy;
                let dz: f64 = atom_j.qz - atom_i.qz;

                let r2 = (dx*dx + dy*dy + dz*dz);
                if &r2 > &CL2 {

                }

                
                
            }
        }

        v
    }
}