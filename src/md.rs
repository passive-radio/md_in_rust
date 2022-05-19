//module to execute MD
use std::time::{Instant};
use crate::variables::{VariablesMD, Variables};
use crate::observer::{ObserverMD, Observer};
use crate::system::adjust_periodic;
use std::fs::File;
use std::io::{self, BufRead, BufReader};
use std::io::prelude::*;

use log::{error, info, warn};
use log4rs;

use crate::utils::remove_whitespaces;

pub trait MD_blueprint {
    fn makeconf(&mut self, conf_type: String);
    fn makeinitcor(&mut self, file: &str) -> io::Result<()>;
    fn update_position(&mut self);
    fn calculate_force(&mut self);
    fn periodic(&mut self);
    fn run(&mut self);

    fn number_of_atoms(&self) -> i32;
    fn calculate(&mut self);
}

pub struct MD {
    pub state: bool,
    pub time: f64,
    pub dt: f64,
    pub vars: VariablesMD,
    pub observer: ObserverMD,
    pub k: f64,
    pub v: f64,
    pub STEPS: i32,
    pub OBSERVE: i32,
    pub cor_file: String
}

impl MD_blueprint for MD{

    fn makeconf(&mut self, conf_type: String) {

        if conf_type == "fcc" {
            const DENSITY: f64 = 0.50;
            let s: f64 = 1.0 / (DENSITY*0.25).powf(1.0/3.0);
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

    fn makeinitcor(&mut self, filepath: &str) -> io::Result<()> {
        const DENSITY: f64 = 0.50;
        let s: f64 = 1.0 / (DENSITY*0.25).powf(1.0/3.0);
        
        // for result in BufReader::new(File::open(file)?).lines() {

        let f = File::open(filepath)?;
        let f = BufReader::new(f);
    
        for line in f.lines() {

            let l = line.unwrap();
    
            let atom = &remove_whitespaces(&l[0..4]);
            let atom_name = &remove_whitespaces(&l[12..16]);
            
            let x = remove_whitespaces(&l[30..38]);
            let y = remove_whitespaces(&l[39..46]);
            let z = remove_whitespaces(&l[47..54]);

            print!("{}, {}, {}\n", x, y, z);

            let x: f64 = x.parse().unwrap();
            let y: f64 = y.parse().unwrap();
            let z: f64 = z.parse().unwrap();

            self.vars.add_atoms(x *s,y *s,z *s, 0.0, 0.0, 0.0);
        }

        self.vars.set_initial_velocity(1.0);
    
        Ok(())
    }

    fn update_position(&mut self) {
        let dt2 = self.dt * 0.5;
        for atom in self.vars.atoms.iter_mut() {
            atom.qx += atom.px * dt2;
            atom.qy += atom.py * dt2;
            atom.qz += atom.pz * dt2;
        }
    }

    fn calculate_force(&mut self) {

        let pn: i32 = self.vars.number_of_atoms();

        let atoms = self.vars.atoms.to_vec();

        for (i, atom_i) in atoms[..(pn-1) as usize].iter().enumerate() {
            for (j, atom_j) in atoms[i+1..].iter().enumerate() {
                let mut dx: f64 = atom_j.qx - atom_i.qx;
                let mut dy: f64 = atom_j.qy - atom_i.qy;
                let mut dz: f64 = atom_j.qz - atom_i.qz;

                let CL2: f64 = self.observer.CUTOFF * self.observer.CUTOFF;
                
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
                let df: f64 = (24.0 * r6 - 48.0) / (r6 * r6 * r2) * self.dt;
                
                self.vars.atoms[i].px += df*dx;
                self.vars.atoms[i].py += df*dy;
                self.vars.atoms[i].pz += df*dz;
                self.vars.atoms[j].px -= df*dx;
                self.vars.atoms[j].py -= df*dy;
                self.vars.atoms[j].pz -= df*dz;
            }
        }
    }

    fn periodic(&mut self) {
        for atom in self.vars.atoms.iter_mut() {
            if atom.qx < 0.0 {
                atom.qx += self.observer.L as f64;
            }
            if atom.qy < 0.0 {
                atom.qy += self.observer.L as f64;
            }
            if atom.qz < 0.0 {
                atom.qz += self.observer.L as f64;
            }
            if atom.qx > self.observer.L as f64{
                atom.qy -= self.observer.L as f64;
            }
            if atom.qy > self.observer.L as f64 {
                atom.qy -= self.observer.L as f64;
            }
            if atom.qz > self.observer.L as f64 {
                atom.qz -= self.observer.L as f64;
            }
        }
    }

    fn calculate(&mut self) {
        self.update_position();
        self.calculate_force();
        self.update_position();
        self.periodic();
        self.time += self.dt;
    }

    fn run(&mut self) {

        log4rs::init_file("config/log4rs.yaml", Default::default()).unwrap();

        log::info!("md calculation started!");

        // self.makeconf("fcc".to_string());
        let cor_file: String = self.cor_file.clone();
        let _ = self.makeinitcor(&cor_file);

        let num = self.vars.number_of_atoms();

        print!("num of atoms: {} \n", &num);

        print!("MD calculation started! \n");
        let start = Instant::now();
        for i in 0..self.STEPS {
            self.calculate();
        }
        let end = start.elapsed();
        self.k = self.observer.kinetic_energy(&self.vars);
        self.v = self.observer.potential_energy(&mut self.vars);
        
        log::info!("MD calculation successfully ended!");
        log::info!("Elapsed time: {}.{:01} s", end.as_secs(), end.subsec_nanos() / 1_000_000);
        print!("MD calculation ended! (elapsed time: {}.{:01} s)\n", end.as_secs(), end.subsec_nanos() / 1_000_000);
        
        // MD::makeconf(self, "fcc".to_string());
    }

    fn number_of_atoms(&self) -> i32{
        self.vars.number_of_atoms()
    }
}