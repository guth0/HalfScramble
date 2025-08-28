use crate::cube;

use cube::Cube;
use cube::Face;
use cube::Piece;

const INF: i32 = 2 ^ 31;

#[derive(Debug)]
pub struct Move {
    face: Face,
    coeff: i8,
}

pub fn solve(cube: &Cube) -> Option<Vec<Move>> {
    let mut threshold = heuristic(&cube);
    let mut path: Vec<Move> = Vec::new();
    loop {
        let tmp = search(&cube, 0, threshold, &mut path);
        if tmp == -1 {
            return Some(path);
        }
        if tmp == INF{
            return None;
        }
        threshold += 1;
        println!("Threshold = {}", threshold);
    }
}

const FACES: [Face; 6] = [Face::U, Face::R, Face::F, Face::L, Face::B, Face::D];

fn search(node: &Cube, g: i32, threshold: i32, path: &mut Vec<Move>) -> i32 {
    let f = g + heuristic(&node);

    // Prune nodes with f-scores that are too high
    if f > threshold {
        return f;
    }

    // If solved, return to the top
    if node.is_solved() {
        return -1;
    }

    let mut min_cost: i32 = INF;

    // check all moves
    for face in FACES {
        // prevents backtracking and moving twice in the same face
        if let Some(mv) = path.last() {
            if face == mv.face {
                continue;
            }
        }

        for coeff in [-1, 1, 2] {
            // do the move
            let mut new_node: Cube = node.clone();
            new_node.make_move(face, coeff);

            // add it to the path
            let mv: Move = Move {
                face: face,
                coeff: coeff as i8,
            };
            path.push(mv);

            let tmp = search(&new_node, g + 1, threshold, path);

            if tmp == -1 {
                return -1;
            }

            if tmp < min_cost {
                min_cost = tmp;
            }

            path.pop();
        }
    }

    return min_cost;
}

// very basic heuristic
fn heuristic(cube: &Cube) -> i32 {
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
