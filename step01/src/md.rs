//module to execute MD

use crate::variables::{VariablesMD, Variables};
use crate::observer::{ObserverMD, Observer};

pub trait MD_blueprint {
    fn makeconf(&mut self, conf_type: String);
    fn update_position();
    fn calculate_force();
    fn periodic();
    fn run(&mut self);

    fn number_of_atoms(&self) -> i32;
}

pub struct MD {
    pub state: bool,
    pub time: f64,
    pub dt: f64,
    pub vars: VariablesMD,
    pub observer: ObserverMD,
    pub k: f64,
    pub v: f64,
}

impl MD_blueprint for MD{

    fn makeconf(&mut self, conf_type: String) {

        if conf_type == "fcc" {
            const density: f64 = 0.50;
            let s: f64 = 1.0 / (density*0.25).powf(1.0/3.0);
            let hs: f64 = s * 0.5;        // half s
            let is: i32 = self.observer.L;

            for iz in 0..is {
                for iy in 0..is {
                    for ix in 0..is {
                        self.vars.add_atoms(ix as f64 *s,iy as f64 *s,iz as f64*s, 0.0, 0.0, 0.0);
                        self.vars.add_atoms(ix as f64 *s + hs,iy as f64 *s,iz as f64*s, 0.0, 0.0, 0.0);
                        self.vars.add_atoms(ix as f64 *s,iy as f64 *s + hs,iz as f64*s, 0.0, 0.0, 0.0);
                        self.vars.add_atoms(ix as f64 *s,iy as f64 *s,iz as f64*s + hs, 0.0, 0.0, 0.0);
                    }
                }
            }

            self.vars.set_initial_velocity(1.0);
        }
    }

    fn update_position() {

    }

    fn calculate_force() {
        
    }

    fn periodic() {
        
    }

    fn run(&mut self) {
        self.makeconf("fcc".to_string());
        // MD::makeconf(self, "fcc".to_string());
        self.k = self.observer.kinetic_energy(&self.vars);
        self.v = self.observer.potential_energy(&mut self.vars);
    }

    fn number_of_atoms(&self) -> i32{
        self.vars.number_of_atoms()
    }
}