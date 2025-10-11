use HalfScramble::pdb::build_corner_pdb;

use std::fs::File;
use std::io::Write;
use std::path::Path;

fn main() -> std::io::Result<()> {
    let path = Path::new("data/corner_pdb.bin");
    std::fs::create_dir_all("data")?; // create data dir if missing

    println!("Generating corner PDB...");

    // generate the PDB
    let pdb = build_corner_pdb();

    println!("PDB Generated; Initiating Save");

    // Save it in the project directory
    let mut file = File::create(path)?;
    file.write_all(&pdb)?;

    println!("Saved PDB to {} ({} bytes)", path.display(), pdb.len());

    Ok(())
}
