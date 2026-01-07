use std::{error::Error, fs::{self, File}, path::{Path, PathBuf}};

use clap::ValueEnum;

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


    pub async fn load(&self, id: &str, format: &FileFormat) -> Result<pdbtbx::PDB, Box<dyn Error>> {
        let id = id.to_uppercase();
        let format_str = format.to_format_str();
        let file_path = self.root.join(format!("{id}.{format_str}"));
        
        if file_path.exists() {
            return open_wrapper(&file_path.to_str().unwrap());
        }

        let url = format!("https://files.rcsb.org/download/{id}.{format_str}");
        let result = reqwest::get(url)
            .await?
            .text()
            .await?;
        fs::write(&file_path, result)?;
        open_wrapper(&file_path.to_str().unwrap())
    }
}

#[derive(Debug, Clone, Copy, ValueEnum)]
pub enum FileFormat {
    /// PDB format
    Pdb,
    /// mmCIF format
    Cif,
}

impl FileFormat {
    fn to_format_str(&self) -> &str {
        match self {
            Self::Pdb => "pdb",
            Self::Cif => "cif",
        }
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
