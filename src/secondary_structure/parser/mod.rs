use std::{fmt::Debug, fs, io::Error};

use reqwest::blocking::get;

use crate::secondary_structure::{SecondaryStructureBuilder, secondary_structure::Bond};

use super::secondary_structure::SecondaryStructure;


pub fn parse(path: &str) -> Result<SecondaryStructure, Error> {
    todo!()
}

pub fn parse_aas(path: &str) -> Result<SecondaryStructure, Error> {
    let text = fs::read_to_string(path)?;
    parse_aas_text(path, text)
}

pub fn parse_aas_text(name: &str, text: String) -> Result<SecondaryStructure, Error> {
    let lines: Vec<&str> = text.lines()
        .filter(|l| l.is_empty())
        .filter(|l| l.trim().starts_with("#")) //Comments
        .collect();

    if lines.iter().count() > 2 {
        return Err(Error::new(std::io::ErrorKind::InvalidInput, "format non valid"));
    }
    //TODO: parse bonds
    Ok(SecondaryStructureBuilder::new(false)
            .add_name(name.to_string())
            .add_sequence(lines[0].to_string())
            .build())
}



