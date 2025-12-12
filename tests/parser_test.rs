use std::result;

use cli_bio::secondary_structure::{parser, secondary_structure::Bond};

#[test]
pub fn parse_aas_test() {
    let input = "AUAUAU \r\n (1,2)(2,3)";
    let result = parser::parse_aas_text("test1", input.to_string());
    assert!(!result.is_err(), "should not be an error: {:?}", result.err());
    let structure = result.unwrap();
    assert_eq!(structure.name, "test1".to_string(), "the name should be test1");
    assert_eq!(structure.description, None, "the descritprion is not setted");
    assert_eq!(structure.bonds, [Bond::new(1, 2), Bond::new(2, 3)], "Bonds should be (1,2) and (2,3)");
    assert_eq!(structure.seq, ['A', 'U', 'A', 'U', 'A', 'U'])
}

#[test]
pub fn parse_aas_fail_test() {
    let input = r"
        #AAAUUU
        (1,2)(1,3)
    ";
    assert!(
        parser::parse_aas_text("test_fail", input.to_string()).is_err(),
        "Should fail to parse"
    );
}

#[test]
fn parse_aas_no_bonds() {
    let input = "ACGU";
    let result = parser::parse_aas_text("no_bonds", input.to_string());
    assert!(result.is_ok());
    let structure = result.unwrap();
    assert_eq!(structure.seq, ['A', 'C', 'G', 'U']);
    assert!(structure.bonds.is_empty(), "Should have no bonds");
}

#[test]
fn parse_aas_with_comments() {
    let input = r"
        # This is a comment
        # Another comment
        ACGU
        (1,4)(2,3)
    ";
    let result = parser::parse_aas_text("with_comments", input.to_string());
    assert!(result.is_ok());
    let structure = result.unwrap();
    assert_eq!(structure.seq, ['A', 'C', 'G', 'U']);
    assert_eq!(structure.bonds, [Bond::new(1, 4), Bond::new(2, 3)]);
}

#[test]
fn parse_aas_bonds_on_same_line() {
    let input = "ACGU(1,4)(2,3)";
    let result = parser::parse_aas_text("same_line", input.to_string());
    assert!(result.is_ok());
    let structure = result.unwrap();
    assert_eq!(structure.bonds, [Bond::new(1, 4), Bond::new(2, 3)]);
}

#[test]
fn parse_aas_with_spaces_in_bonds() {
    let input = "ACGU ( 1 , 4 ) ( 2 , 3 )";
    let result = parser::parse_aas_text("spaces", input.to_string());
    assert!(result.is_ok());
    let structure = result.unwrap();
    assert_eq!(structure.bonds, [Bond::new(1, 4), Bond::new(2, 3)]);
}

#[test]
fn parse_aas_multiline_bonds() {
    let input = "ACGU\n(1,4)\n(2,3)\n(5,8)";
    let result = parser::parse_aas_text("multiline", input.to_string());
    assert!(result.is_ok());
    let structure = result.unwrap();
    assert_eq!(structure.bonds, [Bond::new(1, 4), Bond::new(2, 3), Bond::new(5, 8)]);
}

#[test]
fn parse_aas_empty_string() {
    let input = "";
    let result = parser::parse_aas_text("empty", input.to_string());
    assert!(result.is_err(), "Empty input should fail");
}

#[test]
fn parse_aas_only_comments() {
    let input = r"
        # Only comments here
        # No actual data
    ";
    let result = parser::parse_aas_text("only_comments", input.to_string());
    assert!(result.is_err(), "Only comments should fail");
}

#[test]
fn parse_aas_long_sequence() {
    let input = "ACGUACGUACGUACGUACGUACGUACGU\n(1,27)(5,23)(10,18)";
    let result = parser::parse_aas_text("long_seq", input.to_string());
    assert!(result.is_ok());
    let structure = result.unwrap();
    assert_eq!(structure.seq.len(), 28);
    assert_eq!(structure.bonds.len(), 3);
}

#[test]
fn parse_aas_mixed_case() {
    let input = "AcGuAcGu\n(1,8)(2,7)";
    let result = parser::parse_aas_text("mixed_case", input.to_string());
    assert!(result.is_ok());
    let structure = result.unwrap();
    assert_eq!(structure.seq, ['A', 'c', 'G', 'u', 'A', 'c', 'G', 'u']);
}


// CT Format Tests

#[test]
fn parse_ct_basic() {
    let input = r"4 ENERGY = -2.3  example
1 A 0 2 4 1
2 C 1 3 3 2
3 G 2 4 2 3
4 U 3 0 1 4";
    
    let result = parser::parse_ct_text("ct_test", input.to_string());
    assert!(result.is_ok());
    let structure = result.unwrap();
    assert_eq!(structure.seq, ['A', 'C', 'G', 'U']);
    assert_eq!(structure.bonds, [Bond::new(1, 4), Bond::new(2, 3)]);
}

#[test]
fn parse_ct_no_bonds() {
    let input = r"4 No bonds
1 A 0 2 0 1
2 C 1 3 0 2
3 G 2 4 0 3
4 U 3 0 0 4";
    
    let result = parser::parse_ct_text("ct_no_bonds", input.to_string());
    assert!(result.is_ok());
    let structure = result.unwrap();
    assert_eq!(structure.seq, ['A', 'C', 'G', 'U']);
    assert!(structure.bonds.is_empty());
}

#[test]
fn parse_ct_complex() {
    let input = r"8 Complex structure
1 A 0 2 8 1
2 C 1 3 7 2
3 G 2 4 6 3
4 U 3 5 5 4
5 A 4 6 4 5
6 C 5 7 3 6
7 G 6 8 2 7
8 U 7 0 1 8";
    
    let result = parser::parse_ct_text("ct_complex", input.to_string());
    assert!(result.is_ok());
    let structure = result.unwrap();
    assert_eq!(structure.seq, ['A', 'C', 'G', 'U', 'A', 'C', 'G', 'U']);
    assert_eq!(structure.bonds.len(), 4);
    assert_eq!(structure.bonds, [
        Bond::new(1, 8),
        Bond::new(2, 7),
        Bond::new(3, 6),
        Bond::new(4, 5)
    ]);
}

#[test]
fn parse_ct_empty() {
    let input = "";
    let result = parser::parse_ct_text("ct_empty", input.to_string());
    assert!(result.is_err());
}

#[test]
fn parse_ct_invalid_format() {
    let input = r"4 Header
1 A 0 2";
    let result = parser::parse_ct_text("ct_invalid", input.to_string());
    assert!(result.is_ok()); // Should skip invalid lines
}

// BPSEQ Format Tests
#[test]
fn parse_bpseq_basic() {
    let input = r"1 A 4
2 C 3
3 G 2
4 U 1";
    
    let result = parser::parse_bpseq_text("bpseq_test", input.to_string());
    assert!(result.is_ok());
    let structure = result.unwrap();
    assert_eq!(structure.seq, ['A', 'C', 'G', 'U']);
    assert_eq!(structure.bonds, [Bond::new(1, 4), Bond::new(2, 3)]);
}

#[test]
fn parse_bpseq_no_bonds() {
    let input = r"1 A 0
2 C 0
3 G 0
4 U 0";
    
    let result = parser::parse_bpseq_text("bpseq_no_bonds", input.to_string());
    assert!(result.is_ok());
    let structure = result.unwrap();
    assert_eq!(structure.seq, ['A', 'C', 'G', 'U']);
    assert!(structure.bonds.is_empty());
}

#[test]
fn parse_bpseq_with_comments() {
    let input = r"# This is a comment
# Sequence name: test
1 A 4
2 C 3
3 G 2
4 U 1";
    
    let result = parser::parse_bpseq_text("bpseq_comments", input.to_string());
    assert!(result.is_ok());
    let structure = result.unwrap();
    assert_eq!(structure.seq, ['A', 'C', 'G', 'U']);
    assert_eq!(structure.bonds.len(), 2);
}

#[test]
fn parse_bpseq_complex() {
    let input = r"1 A 8
2 C 7
3 G 6
4 U 5
5 A 4
6 C 3
7 G 2
8 U 1";
    
    let result = parser::parse_bpseq_text("bpseq_complex", input.to_string());
    assert!(result.is_ok());
    let structure = result.unwrap();
    assert_eq!(structure.bonds.len(), 4);
}

#[test]
fn parse_bpseq_empty() {
    let input = "";
    let result = parser::parse_bpseq_text("bpseq_empty", input.to_string());
    assert!(result.is_err());
}

#[test]
fn parse_bpseq_only_comments() {
    let input = r"# Only comments
# No data";
    let result = parser::parse_bpseq_text("bpseq_only_comments", input.to_string());
    assert!(result.is_err());
}

#[test]
fn parse_bpseq_with_empty_lines() {
    let input = r"1 A 4

2 C 3

3 G 2
4 U 1";
    
    let result = parser::parse_bpseq_text("bpseq_empty_lines", input.to_string());
    assert!(result.is_ok());
    let structure = result.unwrap();
    assert_eq!(structure.seq.len(), 4);
}

//DB Parser

#[test]
fn parse_db() {
    let input = r"
        AUAUAU
        .().()
    ";//012345

    let result = parser::parse_db_text("db_test", input.to_string());

    assert!(!result.is_err(), "input should be parsed");
    let structure = result.unwrap();
    assert_eq!(structure.seq, Vec::from_iter("AUAUAU".chars()));
    assert_eq!(structure.bonds, [Bond::new(1, 2), Bond::new(4, 5)])
}