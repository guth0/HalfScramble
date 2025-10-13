use HalfScramble::cube::{Cube, Move};
use HalfScramble::solver::solve;
use HalfScramble::pdb::load_pdb;
use HalfScramble::scramble::{generate_scramble, invert_move, invert_path, print_path};

fn main() {

    let pdb_path = "data/corner_pdb.bin";

    let pdb = load_pdb(pdb_path).expect("Error: Could not load PDB, please verify \
        the pdb is at 'data/corner_pdb.bin' or rebuild it with `cargo run --release --bin build_pdb`");

    println!("Loaded PDB from {}", pdb_path);
                           
    let mut cube = Cube::new();

    let scramble_len = 7;
    let scramble = generate_scramble(scramble_len);

    // This is to prevent the solution from being the inverse of the scramble
    let last_move_inv: Move = invert_move(scramble[(scramble_len - 1) as usize]);

    for mv in scramble.iter()
    {
        cube.make_move(*mv);
    }

    let path = solve(&cube, last_move_inv, &pdb, scramble_len).expect("No path found :(");

    let long_scramble = invert_path(&path);
    let solution = invert_path(&scramble);

    print!("Scramble: ");
    print_path(&long_scramble);

    print!("Solution: ");
    print_path(&solution);
}
