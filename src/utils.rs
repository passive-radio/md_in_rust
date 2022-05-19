use std::fs::File;
use std::io::{self, BufRead, BufReader};

// fn remove_whitespace(s: &mut String) {
//     s.retain(|c| !c.is_whitespace());
// }

pub fn remove_whitespaces(s: &str) -> String {
    s.chars().filter(|c| !c.is_whitespace()).collect()
}