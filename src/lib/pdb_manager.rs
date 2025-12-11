use std::{error::Error, fs, path::{Path, PathBuf}};

use pdbtbx::PDBError;

pub enum PdbManagerError {
    BreakingError(PDBError),
    ParsingErrors()
}

pub struct PdbManager {
    root: PathBuf,
}

impl PdbManager {
    /// Create a PdbManager, for dowloading and parsing pdbs
    pub fn new<P: AsRef<Path>>(root: P) -> Self {
        let root = root.as_ref().to_path_buf();
        fs::create_dir_all(&root).unwrap();
        Self {
            root: root
        }
    }


    pub async fn load(&self, id: &str) -> Result<pdbtbx::PDB, Box<dyn Error>> {
        let id = id.to_uppercase();
        let file_path = self.root.join(format!("{}.pdb", id));
        
        if file_path.exists() {
            return open_wrapper(&file_path.to_str().unwrap());
        }
        let url = format!("https://files.rcsb.org/download/{id}.pdb");
        let result = reqwest::get(url)
            .await?
            .text()
            .await?;
        fs::write(&file_path, result)?;
        open_wrapper(&file_path.to_str().unwrap())
    }
}

/// This method handle the result from open() function 
fn open_wrapper(path: &str) -> Result<pdbtbx::PDB, Box<dyn Error>> {
    match pdbtbx::ReadOptions::new().set_level(pdbtbx::StrictnessLevel::Loose).read(path) {
        Ok((pdb, errors)) => {
            if !errors.is_empty() {
                log::warn!("PDB parsing warnings (non-fatal): {} warnings found", errors.len());
                for error in &errors {
                    log::debug!("{:?}", error);
                }
            }
            Ok(pdb)
        }
        Err(errors) => {
            Err(Box::new(std::io::Error::new(
                std::io::ErrorKind::InvalidData,
                format!("Failed to parse PDB: {:?}", errors)
            )))
        }
    }
}
