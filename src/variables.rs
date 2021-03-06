use std::{ops::{Add, Sub, Mul, Div, }, f64::consts::PI};
use rand::Rng;

#[derive(Debug, Clone)]
pub struct Atom {
    pub qx: f64,
    pub qy: f64,
    pub qz: f64,
    pub px: f64,
    pub py: f64, 
    pub pz: f64,
}

impl Atom {
    pub fn init() -> Atom {
        Atom { qx: 0.0, qy: 0.0, qz: 0.0, px: 0.0, py: 0.0, pz: 0.0}
    }
}

pub struct Pair {
    pub i: i32,
    pub j: i32,
}



impl Pair {

}

pub struct VariablesMD {
    pub time: f32,
    pub atoms: Vec<Atom>,
}

pub trait Variables {

    fn init_time(&mut self);
    fn add_atoms(&mut self, qx: f64, qy: f64, qz: f64, px: f64, py: f64, pz: f64);
    fn number_of_atoms(&self) -> i32;
    fn set_initial_velocity(&mut self, v0: f64);
    fn export_3dview(&self);
}

impl Variables for VariablesMD {

    fn init_time(&mut self) {
        self.time = 0.0;
    }

    fn add_atoms(&mut self, qx: f64, qy: f64, qz: f64, px: f64, py: f64, pz: f64) {
        let mut atom: Atom = Atom::init();
        atom.qx = qx;
        atom.qy = qy;
        atom.qz = qz;
        atom.px = px; 
        atom.py = py;
        atom.pz = pz;
        self.atoms.push(atom);
    }
    
    fn number_of_atoms(&self) -> i32 {
        self.atoms.len() as i32
    }

    fn set_initial_velocity(&mut self, v0: f64) {
        let mut avx = 0.0;
        let mut avy: f64 = 0.0;
        let mut avz: f64 = 0.0;
        
        let mut rng = rand::thread_rng();
        let rand: f64 = rng.gen();
        print!("rand: {:?}\n", rand);

        for n in 0..(self.atoms.len() as usize) {
            let i: f64 = rng.gen();
            let z = i * 2.0 - 1.0;
            let phi = 2.0 * i * PI;
            let vx: f64 = v0 * (1.0 - z*z).sqrt() * phi.cos();
            let vy: f64 = v0 * (1.0 - z*z).sqrt() * phi.sin();
            let vz = v0 * z;
            self.atoms[n].px = vx;
            self.atoms[n].py = vy;
            self.atoms[n].pz = vz;
            avx += vx;
            avy += vy;
            avz += vz;
        }
        let pn: f64 = self.atoms.len() as f64;
        avx /= pn;
        avy /= pn;
        avz /= pn;
        
        for n in 0..(self.atoms.len() -1 as usize) {
            self.atoms[n].px -= avx;
            self.atoms[n].py -= avy;
            self.atoms[n].pz -= avz;
        }
    }

    fn export_3dview(&self) {
        self.time;
    }
}