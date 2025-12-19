use std::{error::Error, path::Path};

use clap::{Args, Parser, Subcommand};

#[derive(Parser)]
struct Cli {
    #[command(subcommand)]
    cmd: Command
}

#[derive(Subcommand)]
enum Command {
    #[command(name="load")]
    CountAtoms(CountAtomsArgs),

}

#[derive(Args)]
struct CountAtomsArgs {
    pdb_id: String
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


#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>>{
    dotenv::dotenv().ok();
    env_logger::init();
    let args = Cli::parse();
    match args.cmd {
        Command::CountAtoms(c) => c.run().await
    }
}
