use std::collections::{LinkedList};

use crate::secondary_structure::secondary_structure::{Bond, SecondaryStructure};


pub mod comparers;
pub mod parser;
pub mod secondary_structure;



struct SecondaryStructureBuilder {
    compute_cross: bool,
    str_name: Option<String>,
    str_description: Option<String>,
    str_bonds: LinkedList<Bond>,
    str_sequence: LinkedList<char>
}

impl SecondaryStructureBuilder {
    fn new(compute_cross: bool) -> Self {
        Self { 
            compute_cross, 
            str_name: None, 
            str_description: None, 
            str_bonds: LinkedList::new(), 
            str_sequence: LinkedList::new(),
        }
    }
    pub fn add_name(mut self, name: String) -> Self {
        self.str_name = Some(name);
        self
    }

    pub fn add_description(mut self, description: String) -> Self {
        self.str_description = Some(description);
        self
    }

    pub fn add_bond(mut self, bond: Bond) -> Self {
        self.str_bonds.push_back(bond);
        self
    }

    pub fn add_sequence(mut self, sequence: String) -> Self {
        let mut list = LinkedList::from_iter(sequence.chars());
        self.str_sequence.append(&mut list);
        self
    }

    pub fn add_nucleotide(mut self, n: char) -> Self {
        self.str_sequence.push_back(n);
        self
    }

    pub fn build(self) -> SecondaryStructure {
        if !self.compute_cross {
            SecondaryStructure {
                //TODO: handle the unwrap?
                name: self.str_name.clone().unwrap(),
                description: self.str_description,
                bonds: self.str_bonds.into_iter().collect(),
                seq: self.str_sequence.into_iter().collect(),
            }
        } else {
            todo!()
        }
    }

}