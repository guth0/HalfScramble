use crate::cube::{Cube, Face, Move, Piece};

use std::collections::VecDeque;

use std::fs::File;
use std::io::{self, Read};
use std::path::Path;

// Compute the actual PDB and return it as a Vec<u8> to be saved
//  piece_range: to select which piecies in the piece array to use for the PDB
//  selector: to select between edge and corner PDBs
//  orientation_base: (the # of possible orientation states a piece can be in)
//      to calcualte the state code
pub fn build_pdb(
    piece_range: std::ops::Range<usize>,
    selector: fn(&Cube) -> &[Piece],
    orientation_base: usize,
) -> Vec<u8> {
    // calculate the total size of the PDB
    let num_pieces = piece_range.end - piece_range.start;
    let size = factorial_recursive(num_pieces) * orientation_base.pow(7);

    // set default to max to more easily identify missed indices
    let mut pdb = vec![u8::MAX; size];

    // start with solved cube
    let solved = Cube::new();
    let start_index = encode_pieces(selector(&solved), orientation_base, piece_range.clone());
    pdb[start_index] = 0;

    let mut queue = VecDeque::new();
    queue.push_back(solved);

    while let Some(node) = queue.pop_front() {
        // get depth
        let depth = pdb[encode_pieces(selector(&node), orientation_base, piece_range.clone())];

        // create all child nodes
        for mv in ALL_MOVES {
            let mut new_node = node.clone();
            new_node.make_move(mv);

            // get the index of the new node
            let index = encode_pieces(selector(&new_node), orientation_base, piece_range.clone());

            // if index is untouched, change it and add the node to the queue
            if pdb[index] == u8::MAX {
                pdb[index] = depth + 1;
                queue.push_back(new_node);
            }
        }
    }

    pdb
}

// Encode a state into an index to be used in the PDB
//  pieces: the state of which to be encoded (could be corners or edges)
//  orientation_base: the # of possible orientations a piece can be in
//  range: which pieces in the array to encode
fn encode_pieces(
    pieces: &[Piece],
    orientation_base: usize,
    range: std::ops::Range<usize>,
) -> usize {
    let mut orient_code: usize = 0;

    // The (range.end - 1) ignores the last orientation since it is implied
    for i in (range.start)..(range.end - 1) {
        // use base 2 since each piece has 2 orientatinos
        orient_code = (orient_code * orientation_base) + pieces[i].ori as usize;
    }

    // Lehmer Code
    let num_pieces = range.end - range.start;
    let perm: Vec<u8> = pieces[range].iter().map(|c| c.pos as u8).collect();
    let mut perm_code: usize = 0;

    for i in 0..num_pieces {
        // counts the number of items smaller and to the right of perm[i]
        // that is the Lehmer value for that number
        let num_smaller = perm[(i + 1)..].iter().filter(|&x| *x < perm[i]).count();
        perm_code = perm_code * (num_pieces - i) + num_smaller;
    }

    // combine permutation and orientation codes for final code
    perm_code * orientation_base.pow(7) + orient_code
}

pub struct PDB {
    data: Box<[u8]>,
    range: std::ops::Range<usize>,
    selector: fn(&Cube) -> &[Piece],
    orientation_base: usize,
}

impl PDB {
    // Get the heuristic value for a certain cube state
    pub fn get_heuristic(&self, cube: &Cube) -> i32 {
        // collect the slice of Pieces using the selector
        //      Either corners or edges
        let pieces: &[Piece] = (self.selector)(cube);

        let index: usize = encode_pieces(pieces, self.orientation_base, self.range.clone());

        self.data[index] as i32
    }

    // Initialize a new PDB
    pub fn new<P: AsRef<Path>>(
        in_path: P,
        in_range: std::ops::Range<usize>,
        in_selector: fn(&Cube) -> &[Piece],
        in_base: usize,
    ) -> io::Result<PDB> {
        let pdb = PDB {
            data: load_pdb(in_path, in_base, in_range.end - in_range.start)?.into_boxed_slice(),
            range: in_range,
            selector: in_selector,
            orientation_base: in_base,
        };

        Ok(pdb)
    }
}

// Gets the largest heuristic from a slice of PDBs for a given state
pub fn get_max_heuristic(cube: &Cube, pdbs: &[PDB]) -> i32 {
    pdbs.iter()
        .map(|pdb| pdb.get_heuristic(cube))
        .max()
        .expect("Unable to retreave heuristic")
}

// Load PDB from a given path
fn load_pdb<P: AsRef<Path>>(
    path: P,
    orientation_base: usize,
    num_pieces: usize,
) -> io::Result<Vec<u8>> {
    let mut f = File::open(path)?;

    let mut pdb =
        vec![0u8; factorial_recursive(num_pieces) * orientation_base.pow((num_pieces - 1) as u32)];

    f.read_exact(&mut pdb)?;

    Ok(pdb)
}

// returns (number!)
fn factorial_recursive(number: usize) -> usize {
    // Base Case
    if number <= 1 {
        return 1;
    }

    // Recursive Case
    return number * factorial_recursive(number - 1);
}

// array of all possible moves
pub const ALL_MOVES: [Move; 18] = [
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

/*
// decode the orientaions from the code
fn rev_corner_orientations(mut code: usize) -> [u8; 8] {
    let mut orientations: [u8; 8] = [0; 8];

    for i in (0..7).rev() {
        orientations[i] = (code % 3) as u8;
        code /= 3;
    }

    // Solve for the last orientation
    // For any valid position, the following equality is true
    //      0 = orientation_sum % 3
    //  Where orientation_sum is the sum of the orientations of the pieces

    let sum: u8 = orientations.iter().copied().sum();
    orientations[7] = (3 - (sum % 3)) % 3;

    orientations
}

// decode the positions from the code
fn rev_corner_lehmer(mut code: usize) -> [u8; 8] {
    let mut positions: [u8; 8] = [0; 8];

    // the lehmer number for the 8th piece is always 0
    for i in (0..7).rev() {
        let num_smaller = (code % (8 - i)) as u8;

        for j in &mut positions[i + 1..] {
            if *j >= num_smaller {
                *j += 1;
            }
        }

        positions[i] = num_smaller;

        code = code / (8 - i);
    }

    positions
}

fn decode_corners(index: usize) -> [Piece; 8] {
    // split code
    let orient_code = index % 2187;
    let perm_code = index / 2187; // this truncates the orient_code part off

    // decode everything
    let orientations: [u8; 8] = rev_corner_orientations(orient_code);
    let positions: [u8; 8] = rev_corner_lehmer(perm_code);

    // Complete the pieces
    let mut pieces: [Piece; 8] = [Piece { pos: 0, ori: 0 }; 8];

    for i in 0..8 {
        pieces[i].ori = orientations[i] as i32;
        pieces[i].pos = positions[i] as i32;
    }

    pieces
}

pub fn test_encode_decode(test_len: i32) -> bool {
    let mut rng = rand::rng();

    let mut cube: Cube = Cube::new();

    for _ in 0..test_len {
        let mv = ALL_MOVES[rng.random_range(0..18)];
        cube.make_move(mv);

        let code = encode_corners(&cube.corners);

        let decoded_corners = decode_corners(code);

        if are_equal_corners(cube.corners, decoded_corners) == false {
            return false;
        }
    }

    true
}

fn are_equal_corners(c1: [Piece; 8], c2: [Piece; 8]) -> bool {
    for i in 0..8 {
        if c1[i].pos != c2[i].pos || c1[i].ori != c2[i].ori {
            return false;
        }
    }

    true
}
*/
