use crate::pdb::{get_max_heuristic, PDB};

use crate::cube::{Cube, Face, Move};

// Finds a different path to the solved cube from the scrambled state
pub fn solve(
    cube: &Cube,
    last_move_inv: Move,
    pdb: &[PDB; 3],
    scramble_len: i32,
) -> Option<Vec<Move>> {
    // The threshold is the minimum number of moves a solution will take (estimate)
    //  all paths with an expected path shorter than this are discarded
    let mut threshold = heuristic(cube, pdb).max(scramble_len);

    println!("Heuristic = {}", heuristic(&cube, pdb));

    // Start the recursion
    let mut path: Vec<Move> = Vec::new();
    loop {
        let t = search(
            &cube,
            0,
            threshold,
            &mut path,
            &last_move_inv,
            pdb,
            scramble_len,
        );

        // if t = -1, path was found, if t = i32::MAX, there is no solution
        if t == -1 {
            return Some(path);
        }
        if t == i32::MAX {
            return None;
        }

        // increase threshold to t if path is not found
        threshold = t;
        println!("Threshold: {}", threshold);
    }
}

const FACES: [Face; 6] = [Face::U, Face::R, Face::F, Face::L, Face::B, Face::D];

pub const OPPOSITE_FACES: [Face; 6] = [Face::D, Face::L, Face::B, Face::R, Face::F, Face::U];

// helper function for the solver (IDA*)
fn search(
    node: &Cube,
    g: i32,
    threshold: i32,
    path: &mut Vec<Move>,
    last_move_inv: &Move,
    pdbs: &[PDB],
    scramble_len: i32,
) -> i32 {
    // calculate the heuristic using the PDBs
    let h = heuristic(&node, pdbs);

    // Total estimated cost (guaranteed not be less than scramble length)
    let f = (g + h).max(scramble_len);

    // If the estimate exceeds the threshold then prune
    if f > threshold {
        return f;
    }

    // If solved, return to top
    if node.is_solved() {
        return -1;
    }

    // initalize the min cost as "infinity"
    let mut min_cost: i32 = i32::MAX;

    // Check all moves
    for &face in FACES.iter() {
        // Prevent redundant moves
        if let Some(prev) = path.last() {
            if face == prev.face || face == OPPOSITE_FACES[prev.face as usize] {
                continue;
            }
        }

        // Iterate over coefficients (-1 = CW, 1 = CCW, 2 = Double Turn)
        for coeff in [-1, 1, 2] {
            // Prevent first move being inverse of scramble
            if path.is_empty() && face == last_move_inv.face && coeff == last_move_inv.coeff {
                continue;
            }

            let mv = Move { face, coeff };

            // Apply move
            let mut new_node = node.clone();
            new_node.make_move(mv);

            // Push to path
            path.push(mv);

            // Recursive search
            let t = search(
                &new_node,
                g + 1,
                threshold,
                path,
                last_move_inv,
                pdbs,
                scramble_len,
            );

            // If found, go to top
            if t == -1 {
                return -1;
            }

            // Track minimum cutoff cost
            if t < min_cost {
                min_cost = t;
            }

            // Backtrack
            path.pop();
        }
    }

    min_cost
}

// calculate the heurstic
fn heuristic(cube: &Cube, pdbs: &[PDB]) -> i32 {
    get_max_heuristic(cube, pdbs)
}

// very basic heuristic
/*
fn computational_heuristic(cube: &Cube) -> i32 {
    let mut misplaced: i32 = 0;
    // count misplaced corners
    for i in 0..8 {
        let piece: &Piece = &cube.corners[i];
        if piece.pos != i as i32 || piece.ori != 0 {
            misplaced += 1;
        }
    }
    for i in 0..12 {
        let piece: &Piece = &cube.edges[i];
        if piece.pos != i as i32 || piece.ori != 0 {
            misplaced += 1;
        }
    }

    misplaced / 4
}
*/
