mod cube;
mod pdb;
mod solver;

use std::fs::File;
use std::io::Read;

use cube::Cube;
use cube::Face;
use cube::Move;
use solver::solve;

fn main() {

    let pdb_path = "data/corner_pdb.bin";
    let mut f = File::open(pdb_path).expect("Error: No PDB file found, please run ");

    let mut pdb = Vec::new();

    f.read_to_end(&mut pdb)?;

    println!("Loaded PDB from {}", path);

    let mut cube = Cube::new();

    cube.print();

    cube.make_move(Face::F, 2);

    cube.make_move(Face::U, 1);
    cube.make_move(Face::L, 2);
    cube.make_move(Face::B, 2);

    cube.print();

    // This is to prevent the solution from being the inverse of the scramble
    let last_move_inv: Move = Move {
        face: Face::B,
        coeff: 2,
    };

    match solve(&cube, last_move_inv) {
        Some(path) => println!("PATH: {:?}", path),
        None => println!("No path found :("),
    }
}
