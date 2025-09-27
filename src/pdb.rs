use crate::cube;

use cube::Cube;
use cube::Face;
use cube::Move;
use cube::Piece;

use std::collections::VecDeque;

fn encode_corners(corners: &[Piece; 8]) -> usize {
    // Each state has 8 pieces
    //   Each piece has 3 orientations
    //       Final orientation is implied
    //   8! total positions

    let mut orient_code: usize = 0;

    // Ignore the last orientation since last one is implied
    for i in 0..7 {
        // use base 3 since each piece has 3 orientatinos
        orient_code = (orient_code * 3) + corners[i].ori as usize;
    }

    // Lehmer Code
    let perm: Vec<u8> = corners.iter().map(|c| c.pos as u8).collect();
    let mut perm_code: usize = 0;

    for i in 0..8 {
        // counts the number of items smaller and to the right of perm[i]
        // that is the Lehmer value for that number
        let num_smaller = perm[(i + 1)..].iter().filter(|&x| *x < perm[i]).count();
        perm_code = perm_code * (8 - i) + num_smaller;
    }

    perm_code * (2187) + orient_code // 2187 = 3^7
}

fn decode_corners(index: usize) -> [Piece; 8] {
    let mut orient_code = index % 2187;
    let mut perm_code = index / 2187; // this truncates the orient_code part off

    // Solve for orientations:

    let mut orientations: [u8; 8] = [0; 8];

    for i in (0..7).rev() {
        orientations[i] = (orient_code % 3) as u8;
        orient_code /= 3;
    }

    // Solve for the last orientation
    // For any valid position, the following equality is true
    //      0 = orientation_sum % 3
    //  Where orientation_sum is the sum of the orientations of the pieces

    let sum: u8 = orientations.iter().copied().sum();
    orientations[7] = (3 - (sum % 3)) % 3;

    // Solve for permutations:

    let mut positions: [u8; 8] = [0; 8];

    // Reverse this:
    /*
    let perm: Vec<u8> = corner.iter().map(|c| c.pos as u8).collect();
    let mut perm_code: usize = 0;

    for i in 0..8 {
        // counts the number of items smaller and to the right of perm[i]
        // that is the Lehmer value for that number
        let num_smaller = perm[(i + 1)..].iter().filter(|&x| *x < perm[i]).count();
        perm_code = perm_code * (8 - i) + num_smaller;
    }*/

    //  0  1  1  0  2  0  0
    // [0, 2, 3, 1, 7, 5, 6]

    //perm_code = perm_code * (8 - i) + num_smaller;
    // perm_code2 = permcode1 * (8 - i) + num;

    let mut smaller_arr: [u8; 8] = [0; 8];

    for i in (0..8).rev() {
        smaller_arr[i] = (perm_code % (8 - i)) as u8;
        perm_code = perm_code / (8 - i);
    }



    // Complete the pieces
    let mut pieces: [Piece; 8];

    for i in 0..8 {
        pieces[i].ori = orientations[i] as i32;
        pieces[i].pos = positions[i] as i32;
    }

    pieces
}

// this shit will take like 10 hours to run...
pub fn build_corner_pdb() -> Vec<u8> {
    let size = 40320 * 2187; // 8! * 3^7

    // set default to max to more easily identify missed indices
    let mut pdb = vec![u8::MAX; size];

    let solved = Cube::new();
    let start_index = encode_corners(&solved.corners);
    pdb[start_index] = 0;

    let mut queue = VecDeque::new();
    queue.push_back(solved);

    while let Some(node) = queue.pop_front() {
        // get depth
        let depth = pdb[encode_corners(&node.corners)];

        // create all child nodes
        for mv in ALL_MOVES {
            let mut new_node = node.clone();
            new_node.make_move(mv.face, mv.coeff as i32);
            let index = encode_corners(&new_node.corners);

            // if index is untouched, change it and add the node to the queue
            if pdb[index] == u8::MAX {
                if depth == 6 {
                    return vec![1];
                };
                pdb[index] = depth + 1;
                queue.push_back(new_node);
            }
        }
    }

    pdb
}

const ALL_MOVES: [Move; 18] = [
    Move {
        face: Face::U,
        coeff: -1,
    },
    Move {
        face: Face::U,
        coeff: 1,
    },
    Move {
        face: Face::U,
        coeff: 2,
    },
    Move {
        face: Face::F,
        coeff: -1,
    },
    Move {
        face: Face::F,
        coeff: 1,
    },
    Move {
        face: Face::F,
        coeff: 2,
    },
    Move {
        face: Face::R,
        coeff: -1,
    },
    Move {
        face: Face::R,
        coeff: 1,
    },
    Move {
        face: Face::R,
        coeff: 2,
    },
    Move {
        face: Face::L,
        coeff: -1,
    },
    Move {
        face: Face::L,
        coeff: 1,
    },
    Move {
        face: Face::L,
        coeff: 2,
    },
    Move {
        face: Face::B,
        coeff: -1,
    },
    Move {
        face: Face::B,
        coeff: 1,
    },
    Move {
        face: Face::B,
        coeff: 2,
    },
    Move {
        face: Face::D,
        coeff: -1,
    },
    Move {
        face: Face::D,
        coeff: 1,
    },
    Move {
        face: Face::D,
        coeff: 2,
    },
];
