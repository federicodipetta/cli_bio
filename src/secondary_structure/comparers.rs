use crate::secondary_structure::secondary_structure::SecondaryStructure;

pub mod len_comparer;
pub mod bp_comparer;

/// This trait compare two secondary structure outputig a number, typacally the function comapre is a metric
pub trait SecondaryStructureComparer {
    /// compare two secondary structure
    fn compare(&self, s1: SecondaryStructure, s2: SecondaryStructure) -> i32 {
        Self::comapre_defualt(s1, s2)
    }
    /// comapre two secondary structure using the default configuration of the alg.
    fn comapre_defualt(s1: SecondaryStructure, s2: SecondaryStructure) -> i32;
}

///Example comparare
pub struct SecondaryStructureComparerExample {

}

/// A simple comparer not biologicaly relevant
impl SecondaryStructureComparer for SecondaryStructure  {
    fn comapre_defualt(s1: SecondaryStructure, s2: SecondaryStructure) -> i32 {
        i32::abs(s1.name.len() as i32 - s2.name.len() as i32)
    }
}
