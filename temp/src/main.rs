use std::fs::File;
use std::io::{self, BufRead, BufReader};

// fn remove_whitespace(s: &mut String) {
//     s.retain(|c| !c.is_whitespace());
// }

fn remove_whitespaces(s: &str) -> String {
    s.chars().filter(|c| !c.is_whitespace()).collect()
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    for result in BufReader::new(File::open("format.pdb")?).lines() {
        let l = result?;

        let atom = &remove_whitespaces(&l[0..4]);
        let atom_serial_number = &remove_whitespaces(&l[6..11]);
        let atom_name = &remove_whitespaces(&l[12..16]);
        let alternate_location_indicator = &remove_whitespaces(&l[16..17]);
        let residue_name = &remove_whitespaces(&l[17..20]);
        let chain_identifier = &remove_whitespaces(&l[21..22]);
        let residue_sequence_number = &remove_whitespaces(&l[22..26]);
        let code_for_insertions_of_residues = &remove_whitespaces(&l[26..27]);
        let x = &remove_whitespaces(&l[30..38]);
        let y = &remove_whitespaces(&l[39..46]);
        let z = &remove_whitespaces(&l[47..54]);
        let occupancy = &remove_whitespaces(&l[54..62]);
        let temperature_factor = &remove_whitespaces(&l[63..70]);
        let segment_identifier = &remove_whitespaces(&l[72..76]);   // is obsolute
        let element_symbol = &remove_whitespaces(&l[76..78]);
        
        println!("atom: {}", &atom);
        println!("atom serial number: {}", &atom_serial_number);
        println!("atom name: {}", &atom_name);
        println!("alternate_location_indicator: {}", &alternate_location_indicator);
        println!("residue_name: {}", &residue_name);
        println!("chain identifier: {}", &chain_identifier);
        println!("residue sequence number: {}", &residue_sequence_number);
        println!("code for insertions of residues: {}", &code_for_insertions_of_residues);
        println!("x: {}", &x);
        println!("y: {}", &y);
        println!("z: {}", &z);
        println!("occupancy: {}", &occupancy);
        println!("temperature factor: {}", &temperature_factor);
        // println!("segment identifier: {}", &segment_identifier);  // is obsolute 
        println!("element symbol: {}", &element_symbol);
        

        println!("{}, {}, {}", x, y, z);
        
    }
    Ok(())
}