
pub struct SecondaryStructure {
    ///Name of the structure
    pub name: String,
    ///The description of the structure
    pub description: Option<String>,
    ///Bonds
    pub bonds: Vec<Bond>,
    ///The sequence
    pub seq: Vec<char>,
}


pub struct Bond {
    // start of the bond
    pub start: u16,
    // end of the bond
    pub end: u16,
    // if it cross with other bonds (is optional because can only be postcomputed)
    pub cross: Option<bool>
}








