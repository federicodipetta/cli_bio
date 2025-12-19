use crate::secondary_structure::comparers::SecondaryStructureComparer;
use crate::secondary_structure::SecondaryStructure;

/// Compare two secondary structure by the length of the BasePairs
pub struct SecondaryStructureBpComparer {

}

impl SecondaryStructureBpComparer {
    pub fn new() -> Self {
        SecondaryStructureBpComparer {  }
    }
}

impl SecondaryStructureComparer for SecondaryStructureBpComparer {
    
    fn comapre_defualt(s1: &SecondaryStructure, s2: &SecondaryStructure) -> i32 {
        i32::abs((s1.bonds.len() - s2.bonds.len()) as i32)
    }
}