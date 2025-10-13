use crate::pdb::encode_corners;

use crate::cube::{Cube, Face, Piece, Move};

pub fn solve(cube: &Cube, last_move_inv: Move, pdb: &[u8], scramble_len: i32) -> Option<Vec<Move>> {
    let mut threshold = heuristic(cube, pdb).max(scramble_len);

    println!("Heuristic = {}", heuristic(&cube, pdb));

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
        if t == -1 {
            return Some(path);
        }
        if t == i32::MAX {
            return None;
        }
        threshold = t;
        println!("Threshold: {}", threshold);
    }
}

const FACES: [Face; 6] = [Face::U, Face::R, Face::F, Face::L, Face::B, Face::D];

pub const OPPOSITE_FACES: [Face; 6] = [Face::D, Face::L, Face::B, Face::R, Face::F, Face::U];

fn search(
    node: &Cube,
    g: i32,
    threshold: i32,
    path: &mut Vec<Move>,
    last_move_inv: &Move,
    pdb: &[u8],
    scramble_len: i32,
) -> i32 {
    let h = heuristic(&node, pdb);

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
                pdb,
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

fn heuristic(cube: &Cube, pdb: &[u8]) -> i32 {
    (pdb[encode_corners(&cube.corners)] as i32).max(computational_heuristic(cube))
}

// very basic heuristic

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

