use HalfScramble::cube::{Cube, Piece};
use HalfScramble::pdb::build_pdb;

use std::fs::File;
use std::io::Write;
use std::path::Path;

use std::env;

fn main() -> std::io::Result<()> {
    let args: Vec<String> = env::args().collect();

    let pdb_num: i32 = args[1].trim().parse().expect("Not a number");

    std::fs::create_dir_all("data")?; // create data dir if missing

    let path: &Path;
    let range: std::ops::Range<usize>;
    let selector: fn(&Cube) -> &[Piece];
    let orientation_base: usize;

    match pdb_num {
        1 => {
            path = Path::new("data/corner_pdb.bin");
            range = 0..8;
            selector = |pdb| &pdb.corners;
            orientation_base = 3;
        }
        2 => {
            path = Path::new("data/edge_pdb_1.bin");
            range = 0..8;
            selector = |pdb| &pdb.edges;
            orientation_base = 2;
        }
        3 => {
            path = Path::new("data/edge_pdb_2.bin");
            range = 4..12;
            selector = |pdb| &pdb.edges;
            orientation_base = 2;
        }

        _ => {
            println!("Invalid PDB number");
            return Ok(());
        }
    }

    println!("Generating PDB...");

    // generate the PDB
    let pdb = build_pdb(range, selector, orientation_base);

    println!("PDB Generated; Initiating Save");

    // Save it in the project directory
    let mut file = File::create(path)?;
    file.write_all(&pdb)?;

    println!("Saved PDB to {} ({} bytes)", path.display(), pdb.len());

    Ok(())
}
