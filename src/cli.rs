use std::{error::Error, path::{Path, PathBuf}};

use clap::{Args, Parser, Subcommand};
use cli_bio::secondary_structure::comparers::{SecondaryStructureComparer, bp_comparer::SecondaryStructureBpComparer, len_comparer::SecondaryStructureLenComparare};



#[derive(Parser)]
pub struct Cli {
    #[command(subcommand)]
    cmd: Command
}

#[derive(Subcommand)]
enum Command {
    #[command(name="load")]
    CountAtoms(CountAtomsArgs),
    #[command(name="ssc")]
    SSC(SscArgs)
}

#[derive(Args)]
struct CountAtomsArgs {
    pdb_id: String
}

#[derive(Args)]
struct SscArgs {
    #[command(subcommand)]
    mode: SscMode,
}

#[derive(Subcommand)]
enum SscMode {
    /// Compare two structures
    #[command(visible_alias = "c")]
    Compare(CompareArgs),
    
    /// Compare all structures in a directory (all vs all)
    #[command(visible_alias = "w")]
    Workbench(WorkbenchArgs),
}

#[derive(Args)]
struct CompareArgs {
    #[command(subcommand)]
    algorithm: SscAlgorithm,
    
    /// First structure file
    file1: PathBuf,
    
    /// Second structure file
    file2: PathBuf,
}

#[derive(Args)]
struct WorkbenchArgs {
    #[command(subcommand)]
    algorithm: SscAlgorithm,
    
    /// Directory containing structure files
    directory: PathBuf,
}

#[derive(Subcommand)]
enum SscAlgorithm {
    #[command(name="len")]
    Len,
    
    #[command(name="bp")]
    BP,
}

impl SscArgs {
    fn run(&self) -> Result<(), Box<dyn Error>> {
        match &self.mode {
            SscMode::Compare(args) => args.run(),
            SscMode::Workbench(args) => args.run(),
        }
    }
}

impl CompareArgs {
    fn run(&self) -> Result<(), Box<dyn Error>> {
        let s1 = cli_bio::secondary_structure::parser::parse(self.file1.to_str().unwrap())?;
        let s2 = cli_bio::secondary_structure::parser::parse(self.file2.to_str().unwrap())?;
        
        let distance = match &self.algorithm {
            SscAlgorithm::Len => {
                let comparer = SecondaryStructureLenComparare::new();
                comparer.compare(&s1, &s2)
            },
            SscAlgorithm::BP => {
                let comparer = SecondaryStructureBpComparer::new();
                comparer.compare(&s1, &s2)
            },
        };
        
        log::info!("Distance: {}", distance);
        Ok(())
    }
}

impl WorkbenchArgs {
    fn run(&self) -> Result<(), Box<dyn Error>> {
        let structures = load_structures_from_dir(&self.directory)?;
        log::info!("Comparing {} structures (all vs all)", structures.len());
        
        match &self.algorithm {
            SscAlgorithm::Len => {
                let comparer = SecondaryStructureLenComparare::new();
                self.run_with_comparer(&structures, &comparer)
            },
            SscAlgorithm::BP => {
                let comparer = SecondaryStructureBpComparer::new();
                self.run_with_comparer(&structures, &comparer)
            },
        }
    }
    
    fn run_with_comparer<C>(&self, structures: &[(String, cli_bio::secondary_structure::secondary_structure::SecondaryStructure)], comparer: &C) -> Result<(), Box<dyn Error>>
    where
        C: SecondaryStructureComparer
    {
        for (i, (name1, s1)) in structures.iter().enumerate() {
            for (name2, s2) in structures.iter().skip(i + 1) {
                let distance = comparer.compare(s1, s2);
                println!("{},{},{}", name1, name2, distance);
            }
        }
        Ok(())
    }
}

fn load_structures_from_dir(path: &Path) -> Result<Vec<(String, cli_bio::secondary_structure::secondary_structure::SecondaryStructure)>, Box<dyn Error>> {
    use std::fs;
    
    let mut structures = Vec::new();
    
    for entry in fs::read_dir(path)? {
        let entry = entry?;
        let path = entry.path();
        
        if path.is_file() {
            if let Some(name) = path.file_stem() {
                let result = cli_bio::secondary_structure::parser::parse(path.to_str().unwrap());
                match result {
                    Ok(structure) => {
                        structures.push((name.to_string_lossy().to_string(), structure));
                    }
                    Err(e) => {
                        log::error!("skipping {} beacuase: {}", name.to_str().unwrap(), e);
                    }
                }
            }
        }
    }
    Ok(structures)
}

impl CountAtomsArgs {
    async fn run(&self) -> Result<(), Box<dyn Error>> {
        log::info!("{}", self.pdb_id);
        let manager = cli_bio::pdb_manager::PdbManager::new(Path::new("downloads"));
        let pdb = manager.load("4plx").await.unwrap();
        log::info!("Successfully loaded PDB with {} atoms", pdb.atom_count());
        Ok(())
    } 
}

impl Cli {
    pub async fn execute(&self) -> Result<(), Box<dyn Error>>{
        match &self.cmd {
            Command::CountAtoms(c) => c.run().await,
            Command::SSC(s) => s.run(),
        }
    } 
}