use std::path::Path;
#[tokio::main]
async fn main() {
    env_logger::init();
    let manager = cli_bio::pdb_manager::PdbManager::new(Path::new("downloads"));
    let pdb = manager.load("4plx").await.unwrap();
    log::info!("Successfully loaded PDB with {} atoms", pdb.atom_count());
}
