#![allow(dead_code)]

use std::fs::{File, OpenOptions};
use std::io::{self, BufRead, BufReader};
use std::io::prelude::*;

use crate::variables::{Atom, Pair};

// fn remove_whitespace(s: &mut String) {
//     s.retain(|c| !c.is_whitespace());
// }

pub fn remove_whitespaces(s: &str) -> String {
    s.chars().filter(|c| !c.is_whitespace()).collect()
}

pub fn write_cor(x: &Vec<Atom>, outputfile: String) {
    let mut f = OpenOptions::new()
                    .write(true)
                    .append(true)
                    .create(true)
                    .open(outputfile)
                    .unwrap();
    {
        write!(f, "{}\n", x.len()).unwrap();
        write!(f, "# {}\n", "much data").unwrap();
        for (i, atom) in x.iter().enumerate() {
            write!(f, "{:?},{:?},{:?},{:?}\n",i, atom.qx, atom.qy, atom.qz).unwrap();
        }
    }
}