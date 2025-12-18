use crate::secondary_structure::comparers::SecondaryStructureComparer;
use crate::secondary_structure::secondary_structure::SecondaryStructure;

/// Compare two secondary strcuture comparing the length of the sequence
pub struct SecondaryStructureLenComparare {

}

impl SecondaryStructureComparer for SecondaryStructureLenComparare {
    fn comapre_defualt(s1: SecondaryStructure, s2: SecondaryStructure) -> i32 {
        i32::abs((s1.seq.len() - s2.seq.len()) as i32)
    }
}
