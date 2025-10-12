use HalfScramble::cube::{Cube, Face, Move};
use HalfScramble::solver::solve;
use HalfScramble::pdb::load_pdb;

fn main() {

    let pdb_path = "data/corner_pdb.bin";

    let pdb = load_pdb(pdb_path).expect("Error: Could not load PDB, please verify the pdb is at 'data/corner_pdb.bin' or rebuild it with `cargo run --release --bin build_pdb`");

    println!("Loaded PDB from {}", pdb_path);

    let mut cube = Cube::new();

    cube.print();

    cube.make_move(Move {face: Face::F, coeff: 2});
    cube.make_move(Move {face: Face::U, coeff: 1});
    cube.make_move(Move {face: Face::L, coeff: 2});
    cube.make_move(Move {face: Face::B, coeff: 2});

    cube.print();

    // This is to prevent the solution from being the inverse of the scramble
    let last_move_inv: Move = Move {
        face: Face::B,
        coeff: 2,
    };

    match solve(&cube, last_move_inv, &pdb) {
        Some(path) => println!("PATH: {:?}", path),
        None => println!("No path found :("),
    }
}
