use std::{collections::VecDeque, fs, io::Error};

use regex::Regex;

use crate::secondary_structure::{SecondaryStructureBuilder, secondary_structure::Bond};

use super::secondary_structure::SecondaryStructure;

pub fn parse(path: &str) -> Result<SecondaryStructure, Error> {
    todo!("{path}")
}

pub fn parse_aas(path: &str) -> Result<SecondaryStructure, Error> {
    let text = fs::read_to_string(path)?;
    parse_aas_text(path, text)
}

pub fn parse_aas_text(name: &str, text: String) -> Result<SecondaryStructure, Error> {
    let description = try_extract_description(&text);
    let text = clean_text(text);
    
    let format_re = Regex::new(r"^\s*([A-Za-z]+)").unwrap();
    
    let caps = format_re.captures(&text)
        .ok_or_else(|| Error::new(std::io::ErrorKind::InvalidInput, "Invalid AAS format"))?;
    
    let sequence = caps.get(1)
        .ok_or_else(|| Error::new(std::io::ErrorKind::InvalidInput, "No sequence found"))?
        .as_str();
    
    let mut builder = SecondaryStructureBuilder::new(false)
        .add_name(name.to_string())
        .add_sequence(sequence.to_string());
    
    if let Some(desc) = description {
        builder = builder.add_description(desc);
    }

    let bond_re = Regex::new(r"\(\s*([0-9]+)\s*,\s*([0-9]+)\s*\)").unwrap();
    
    for cap in bond_re.captures_iter(&text) {
        let i = cap[1].parse::<u16>().unwrap();
        let j = cap[2].parse::<u16>().unwrap();
        builder = builder.add_bond(Bond::new(i, j));
    }
    
    Ok(builder.build())
}

pub fn parse_ct_text(name: &str, text: String) -> Result<SecondaryStructure, Error> {
    let description = try_extract_description(&text);
    let text = clean_text(text);
    let mut lines = text.lines();

    let _header = lines.next()
        .ok_or_else(|| Error::new(std::io::ErrorKind::InvalidInput, "Empty CT file"))?;
    
    let mut builder = SecondaryStructureBuilder::new(false).add_name(name.to_string());

    if let Some(desc) = description {
        builder = builder.add_description(desc);
    }
    
    let entries: Vec<_> = lines.collect();

    if entries.is_empty() {
        return Err(Error::new(std::io::ErrorKind::InvalidInput, "Empty string is not allowed"));
    }

    for line in &entries {
        let parts: Vec<&str> = line.split_whitespace().collect();
        if parts.len() < 6 { continue; }
        
        let base = parts[1].chars().next().unwrap();
        builder = builder.add_nucleotide(base);
    }

    for line in entries {
        let parts: Vec<&str> = line.split_whitespace().collect();
        if parts.len() < 6 { continue; }
        
        let index = parts[0].parse::<u16>()
            .map_err(|_| Error::new(std::io::ErrorKind::InvalidInput, "Invalid index"))?;
        let pair = parts[4].parse::<u16>()
            .map_err(|_| Error::new(std::io::ErrorKind::InvalidInput, "Invalid pair"))?;
        
        if pair > 0 && index < pair {
            builder = builder.add_bond(Bond::new(index, pair));
        }
    }
    
    Ok(builder.build())
}

pub fn parse_bpseq_text(name: &str, text: String) -> Result<SecondaryStructure, Error> {
    let description = try_extract_description(&text);
    let text = clean_text(text);
    
    let mut builder = SecondaryStructureBuilder::new(false).add_name(name.to_string());
    
    if let Some(desc) = description {
        builder = builder.add_description(desc);
    }
    
    let entries: Vec<_> = text.lines().collect();
    
    if entries.is_empty() {
        return Err(Error::new(std::io::ErrorKind::InvalidInput, "Empty string is not allowed"));
    }

    for line in &entries {
        let parts: Vec<&str> = line.split_whitespace().collect();
        if parts.len() < 3 {
            return Err(Error::new(std::io::ErrorKind::InvalidInput, "Invalid BPSEQ format"));
        }
        
        let base = parts[1].chars().next().unwrap();
        builder = builder.add_nucleotide(base);
    }
    
    for line in entries {
        let parts: Vec<&str> = line.split_whitespace().collect();
        let index = parts[0].parse::<u16>()
            .map_err(|_| Error::new(std::io::ErrorKind::InvalidInput, "Invalid index"))?;
        let pair = parts[2].parse::<u16>()
            .map_err(|_| Error::new(std::io::ErrorKind::InvalidInput, "Invalid pair"))?;
        
        if pair > 0 && index < pair {
            builder = builder.add_bond(Bond::new(index, pair));
        }
    }
    
    Ok(builder.build())
}

/// Represents pair of chars rapresenting the bond in DB format
const SEPRATORS_START: &'static [char] = &[
    '(',
    '[',
    '{',
    '<',
];

const SEPARATOR_END: &'static [char] = & [
    ')',
    ']',
    '}',
    '>',
];

pub fn parse_db_text(name: &str, text: String) -> Result<SecondaryStructure, Error>{
    let description = try_extract_description(&text);
    let text = clean_text(text);

    let format_re = Regex::new(r"^\s*([A-Za-z]+)").unwrap();
    
    let caps = format_re.captures(&text)
        .ok_or_else(|| Error::new(std::io::ErrorKind::InvalidInput, "Invalid AAS format"))?;
    
    let sequence = caps.get(1)
        .ok_or_else(|| Error::new(std::io::ErrorKind::InvalidInput, "No sequence found"))?
        .as_str();
    
    let mut builder = SecondaryStructureBuilder::new(false)
        .add_name(name.to_string())
        .add_sequence(sequence.to_string());

    if let Some(desc) = description {
        builder = builder.add_description(desc);
    }

    let bond_part = format_re.replace(&text, "").to_string();
    let mut cont = 0;
    let mut stack: VecDeque<(char, u16)> = VecDeque::new();
    for char in bond_part.chars() {
        if char == '.' {
            cont += 1;
        } else if SEPRATORS_START.contains(&char) {
            stack.push_front((char, cont));
            cont += 1;
        } else if SEPARATOR_END.contains(&char) {
            let char_start = stack.pop_front().ok_or_else(
                || Error::new(std::io::ErrorKind::InvalidInput, format!("found a closing('{char}') but no opening char found"))
            )?;

            if SEPRATORS_START.binary_search(&char_start.0).unwrap() == SEPARATOR_END.binary_search(&char).unwrap() {
                builder = builder.add_bond(Bond::new(char_start.1, cont))
            } else {
                return Err(Error::new(std::io::ErrorKind::InvalidInput, format!("Mismatch at {},{}. Closing char not corresponding with the opening one", char_start.0, cont)));
            }
            cont += 1;
        }
    }
    
    Ok(builder.build())
}


fn clean_text(text: String) -> String {
    text.lines()
        .filter(|l| !l.is_empty() && !l.trim().starts_with("#"))
        .collect::<Vec<_>>()
        .join("\n")
}

fn try_extract_description(text: &String) -> Option<String> {
    text.lines()
        .filter(|l| l.starts_with("#"))
        .map(|l| l.replacen("#", "", 1))
        .reduce(|a, b| format!("{a}\n{b}"))
}