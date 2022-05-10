use crate::variables::{Variables, VariablesMD};
use crate::system::adjust_periodic;

// #[derive(Debug)]
pub trait Observer {
    fn kinetic_energy(&self, vars: &VariablesMD) -> f64;
    fn potential_energy(&self, vars: &VariablesMD) -> f64;
    fn tempareture(&self, vars: &VariablesMD) -> f64;
    fn total_energy(&self, vars: &mut VariablesMD) -> f64;
}

// impl Observer {
//     pub fn kinetic_energy(vars: Variables) -> f64;
//     pub fn potential_energy(vars: Variables) -> f64;
//     pub fn tempareture(vars: Variables) -> f64 {
//         kinetic_energy(vars) / 1.5
//     }
//     pub fn total_energy(vars: Variables) -> f64 {
//         kinetic_energy(vars) + potential_energy(vars)
//     }
// }

pub struct ObserverMD {
    /// how to use
    /// let observer = ObserverMD {
    ///     CUTOFF: 2.0,
    ///     L: 10,
    ///     dt: 0.01,
    /// }
    /// observer.kinetic_energy(vars)
    
    pub CUTOFF: f64,
    pub L: i32,
    pub dt: f32,
    
}


impl Observer for ObserverMD {
    //This function calculates and returns the kinetic energy of each atoms.
    fn kinetic_energy(&self, vars: &VariablesMD) -> f64 {
        let mut k: f64 = 0.0;

        for n in 0..(vars.atoms.len() as usize) {
            k += vars.atoms[n].px * vars.atoms[n].px;
            k += vars.atoms[n].py * vars.atoms[n].py;
            k += vars.atoms[n].pz * vars.atoms[n].pz;
        }
        k /= vars.number_of_atoms() as f64;
        k
    } 

    //This function calculates and returns the potential energy of each atoms.
    fn potential_energy(&self, vars: &VariablesMD) -> f64 {
        let mut v: f64 = 0.0;
        let CL2: f64 = self.CUTOFF*self.CUTOFF;
        let RC2: f64 = 1.0 / CL2;
        let RC6: f64 = RC2 * RC2 * RC2;
        let RC12: f64 = RC6 * RC6;
        let C0: f64 = -4.0 * (RC12 - RC6);

        let pn: i32 = vars.number_of_atoms();
        let atoms = &vars.atoms;

        for i in 0..5 {
            print!("{:?}", atoms[i]);
        }

        for (i, atom_i) in atoms[0..].iter().enumerate() {
            for (j, atom_j) in atoms[i..].iter().enumerate() {
                let mut dx: f64 = atom_j.qx - atom_i.qx;
                let mut dy: f64 = atom_j.qy - atom_i.qy;
                let mut dz: f64 = atom_j.qz - atom_i.qz;

                
                (dx, dy, dz) = adjust_periodic(dx, dy, dz);

                // print!("{:?}, {:?}, {:?}", &dx, &dy, &dz);

                let r2 = (dx*dx + dy*dy + dz*dz);

                // print!("r2: {:?}", &r2);
                if r2 > CL2 {
                    continue;
                } else if r2 == 0.0 {
                    continue;
                }
                let r6: f64 = r2 * r2 * r2;
                let r12: f64 = r6 * r6;
                v += 4.0 * (1.0/ r12 - 1.0/ r6) + C0;
            }
        }
        v /= pn as f64;
        v
    }

    fn tempareture(&self, vars: &VariablesMD) -> f64 {
        ObserverMD::kinetic_energy(&self, vars) / 1.5
    }
    fn total_energy(&self, vars: &mut VariablesMD) -> f64 {
        ObserverMD::kinetic_energy(&self, vars) + ObserverMD::potential_energy(self, vars)
    }
}