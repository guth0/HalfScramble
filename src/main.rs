use HalfScramble::cube::{Cube, Move};
use HalfScramble::pdb::PDB;
use HalfScramble::scramble::{generate_scramble, invert_move, invert_path, print_path};
use HalfScramble::solver::solve;

use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();

    let scramble_len: i32 = args[1].trim().parse().expect("Not a number");

    let corner_pdb_path = "data/corner_pdb.bin";
    let edge1_pdb_path = "data/edge_pdb_1.bin";
    let edge2_pdb_path = "data/edge_pdb_2.bin";

    let pdb_array: [PDB; 3] = [
        PDB::new(corner_pdb_path, 0..8, |c| &c.corners, 3).expect("Error: Could not load corner PDB, please \
        verify the pdb is at 'data/corner_pdb.bin' or rebuild it with `cargo run --release --bin build_pdb 1`"),
        PDB::new(edge1_pdb_path, 0..8, |c| &c.edges, 2).expect("Error: Could not load edge PDB #1, please \
        verify the pdb is at 'data/edge_pdb_1.bin' or rebuild it with `cargo run --release --bin build_pdb 2`"),
        PDB::new(edge2_pdb_path, 4..12, |c| &c.edges, 2).expect("Error: Could not load edge PDB #2, please \
        verify the pdb is at 'data/edge_pdb_2.bin' or rebuild it with `cargo run --release --bin build_pdb 3`"),
    ];

    println!("Loaded corner PDB from {}", corner_pdb_path);
    println!("Loaded edge PDB #1 from {}", edge1_pdb_path);
    println!("Loaded edge PDB #2 from {}", edge2_pdb_path);

    let mut cube = Cube::new();

    let scramble = generate_scramble(scramble_len);

    // This is to prevent the solution from being the inverse of the scramble
    let last_move_inv: Move = invert_move(scramble[(scramble_len - 1) as usize]);

    for mv in scramble.iter() {
        cube.make_move(*mv);
    }

    let path = solve(&cube, last_move_inv, &pdb_array, scramble_len).expect("No path found :(");

    let long_scramble = invert_path(&path);
    let solution = invert_path(&scramble);

    print!("Scramble: ");
    print_path(&long_scramble);

    print!("Solution: ");
    print_path(&solution);
}
