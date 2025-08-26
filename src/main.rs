use std::array;

fn main() {
    println!("--------------------------");

    let mut cube = Cube::new();

    cube.print();

    println!("--------------------------");

    cube.make_move(Face::D, 1);

    cube.print();

    println!("--------------------------");
    cube.make_move(Face::F, -1);

    cube.print();

    println!("--------------------------");
}

// for this explanation, move notation will be used to describe the sides of the cube:
//  U -> up/top     |    D -> down/bottom
//  F -> front      |    B -> back
//  R -> right      |    L -> left
//
// for the order of the pieces in the Cube arrays:
//  orient the solved cube with white on top and green in front
//  For corners:
//      The are ordered the same as quadrents in a 3D plane
//          Green-Blue -> Z Axis
//          White-Yellow -> Y Axis
//          Red-Orange -> X Axis
//      So, the corner 0 would be quadrent I, so the Front Top Right/UFR/White-Green-Red
//      Then the UFL would be quadrent 2 and so on.
//
//      So the order is (starting at 0):
//          UFR, UFL, DFL, DFR, UBR, UBL, DBL, DBR
//
//  For Edges:
//      The order will just be:
//          UF, UR, UB, UL, FR, BR, BL, FL, DF, DR, DB, DL

#[derive(Clone, Copy, Debug)]
struct Piece {
    position: i32,
    orientation: i32,
    // Edges will have 12 possible positions and 2 possible orientations  (Total : 2*12=24)
    // Corners will have 8 possible positions and 3 possible orientations (Total : 3*8=24)
}

impl Piece {}

// This is an array of 6 3x3 faces
// 0=U, 1=R, 2=F, 3=D, 4=L, 5=B
type CubeState = [[[char; 3]; 3]; 6];

fn fill_state(cube: &Cube) -> CubeState {
    let mut state: CubeState = [[[' '; 3]; 3]; 6];

    // fill in centers
    // NOTE: A premade state with the centers already done would be faster
    //       but this function isn't used enough for that to be worth it.

    state[Face::U as usize][1][1] = 'W';
    state[Face::R as usize][1][1] = 'R';
    state[Face::F as usize][1][1] = 'G';
    state[Face::D as usize][1][1] = 'Y';
    state[Face::L as usize][1][1] = 'O';
    state[Face::B as usize][1][1] = 'B';

    // Corners

    for pos in 0..8 {
        // find which piece is in the 'pos' position
        let index = cube
            .corners
            .iter()
            .position(|piece| piece.position == pos)
            .expect("Corner Position #{pos} Empty"); // Panics if no peice is found

        // fill in the sticker on the 3 sides that share the corner
        //      this uses the 'CORNER_TABLE' which is a lookup table
        //      for where to put the stickers based on 'pos'
        for i in 0..3 {
            // get the lookup table info
            let sticker_pos: &(Face, u8, u8) = &CORNER_TABLE[pos as usize][i];

            // set the color in the state
            state[sticker_pos.0 as usize][sticker_pos.1 as usize][sticker_pos.2 as usize] =
                CORNER_COLORS[index]
                    .chars()
                    .nth((i + (cube.corners[index].orientation as usize)) % 3)
                    .expect("No Color Found");
            // NOTE: the (i + orientation) % 3 makes it so the colors in the piece actually rotate
        }
    }

    // Edges
    for pos in 0..12 {
        // find which piece is in the 'pos' position
        let index = cube
            .edges
            .iter()
            .position(|piece| piece.position == pos)
            .expect("Edge Position #{pos} Empty"); // Panics if no peice is found

        // fill in the sticker on the 2 sides that share the edge
        //      Uses the 'EDGE_TABLE' lookup table to see where to place them
        for i in 0..2 {
            // lookup table info
            let sticker_pos: &(Face, u8, u8) = &EDGE_TABLE[pos as usize][i];

            // set the color in the state
            state[sticker_pos.0 as usize][sticker_pos.1 as usize][sticker_pos.2 as usize] =
                EDGE_COLORS[index]
                    .chars()
                    .nth((i + (cube.edges[index].orientation as usize)) % 2)
                    .expect("No Color Found");
        }
    }


    return state;
}

// could try returning a &[char; 3] for a little more performance
fn print_chars(state: &CubeState, f: Face, i: usize) {
    for c in state[f as usize][i] {
        print!("{}", c);
    }
}

// this is ugly as fuck, please ignore
fn print_state(state: &CubeState) {
    print!("   ");
    print_chars(state, Face::U, 0);
    print!("\n   ");
    print_chars(state, Face::U, 1);
    print!("\n   ");
    print_chars(state, Face::U, 2);
    println!();

    print_chars(state, Face::L, 0);
    print_chars(state, Face::F, 0);
    print_chars(state, Face::R, 0);
    print_chars(state, Face::B, 0);
    println!();
    print_chars(state, Face::L, 1);
    print_chars(state, Face::F, 1);
    print_chars(state, Face::R, 1);
    print_chars(state, Face::B, 1);
    println!();
    print_chars(state, Face::L, 2);
    print_chars(state, Face::F, 2);
    print_chars(state, Face::R, 2);
    print_chars(state, Face::B, 2);

    print!("\n   ");
    print_chars(state, Face::D, 0);
    print!("\n   ");
    print_chars(state, Face::D, 1);
    print!("\n   ");
    print_chars(state, Face::D, 2);

    println!();
}

const CORNER_COLORS: [&str; 8] = ["WRG", "WGO", "YOG", "YGR", "WBR", "WOB", "YBO", "YRB"];
const EDGE_COLORS: [&str; 12] = [
    "WG", "WR", "WB", "WO", "GR", "BR", "BO", "GO", "YG", "YR", "YB", "YO",
];

#[repr(u8)]
#[derive(Clone, Copy, PartialEq)]
enum Face {
    U,
    R,
    F,
    L,
    B,
    D,
}

// could combine these into a single table
const CORNER_MOVE_TABLE: [[u8; 4]; 6] = [
    [1, 5, 4, 0],
    [0, 4, 7, 3],
    [0, 3, 2, 1],
    [1, 2, 6, 5],
    [4, 5, 6, 7],
    [2, 3, 7, 6],
];

const EDGE_MOVE_TABLE: [[u8; 4]; 6] = [
    [3, 2, 1, 0],
    [1, 5, 9, 4],
    [0, 4, 8, 7],
    [3, 7, 11, 6],
    [2, 6, 10, 5],
    [8, 9, 10, 11],
];

const CORNER_TABLE: [[(Face, u8, u8); 3]; 8] = [
    // The tuples are (Face, row, column)
    [(Face::U, 2, 2), (Face::R, 0, 0), (Face::F, 0, 2)],
    [(Face::U, 2, 0), (Face::F, 0, 0), (Face::L, 0, 2)],
    [(Face::D, 0, 0), (Face::L, 2, 2), (Face::F, 2, 0)],
    [(Face::D, 0, 2), (Face::F, 2, 2), (Face::R, 2, 0)],
    [(Face::U, 0, 2), (Face::B, 0, 0), (Face::R, 0, 2)],
    [(Face::U, 0, 0), (Face::L, 0, 0), (Face::B, 0, 2)],
    [(Face::D, 2, 0), (Face::B, 2, 2), (Face::L, 2, 0)],
    [(Face::D, 2, 2), (Face::R, 2, 2), (Face::B, 2, 0)],
];


const EDGE_TABLE: [[(Face, u8, u8); 2]; 12] = [
    [(Face::U, 2, 1), (Face::F, 0, 1)],
    [(Face::U, 1, 2), (Face::R, 0, 1)],
    [(Face::U, 0, 1), (Face::B, 0, 1)],
    [(Face::U, 1, 0), (Face::L, 0, 1)],
    [(Face::F, 1, 2), (Face::R, 1, 0)],
    [(Face::B, 1, 0), (Face::R, 1, 2)],
    [(Face::B, 1, 2), (Face::L, 1, 0)],
    [(Face::F, 1, 0), (Face::L, 1, 2)],
    [(Face::D, 0, 1), (Face::F, 2, 1)],
    [(Face::D, 1, 2), (Face::R, 2, 1)],
    [(Face::D, 2, 1), (Face::B, 2, 1)],
    [(Face::D, 1, 0), (Face::L, 2, 1)],
];

#[derive(Clone, Copy)]
struct Cube {
    // 2 arrays of pieces
    corners: [Piece; 8],
    edges: [Piece; 12],
}

impl Cube {
    fn new() -> Self {
        // create the pieces and put them in the correct position and orientation
        //      When solved, the orientations are all 0
        Cube {
            corners: array::from_fn(|i| Piece {
                position: i as i32,
                orientation: 0,
            }),
            edges: array::from_fn(|i| Piece {
                position: i as i32,
                orientation: 0,
            }),
        }
    }

    fn print(self) {
        let state: CubeState = fill_state(&self);

        print_state(&state);
    }

    fn make_move(&mut self, face: Face, coeff: i32) {
        // Get the cycles from the move tables
        // if coeff is negative, flip the cycle around
        let corner_pos_cycle: [u8; 4] = {
            if coeff == -1 {
                let mut tmp = CORNER_MOVE_TABLE[face as usize];
                tmp.reverse();
                tmp
            } else {
                CORNER_MOVE_TABLE[face as usize]
            }
        };

        let edge_pos_cycle: [u8; 4] = {
            if coeff == -1 {
                let mut tmp = EDGE_MOVE_TABLE[face as usize];
                tmp.reverse();
                tmp
            } else {
                EDGE_MOVE_TABLE[face as usize]
            }
        };

        // Corners
        {
            // cycle pieces
            let corner_cycle = cycle_pieces(&mut self.corners, &corner_pos_cycle);

            // orient pieces
            for i in 0..4 {
                // Not all 4 corners are to be rotated the same amount
                // for a CW rotation, they should be rotated by
                //  [2, 1, 2, 1], but for CCW, [1, 2, 1, 2]
                
                let rotation: i32;
                match face
                {
                    Face::F | Face::L => rotation = get_rotation(coeff, i),
                    Face::B | Face::R => rotation = get_rotation(-coeff, i),
                    Face::U | Face::D => rotation = 0,
                }

                self.corners[corner_cycle[i]].orientation =
                    (self.corners[corner_cycle[i]].orientation + rotation)  % 3;
            }
        }

        // Edges
        {
            // cycle pieces
            let edge_cycle = cycle_pieces(&mut self.edges, &edge_pos_cycle);

            // orient pieces (Only changes on F or B move)

            if face == Face::F || face == Face::B {
                for i in 0..4 {
                    self.edges[edge_cycle[i]].orientation =
                        (self.edges[edge_cycle[i]].orientation + 1) % 2;
                }
            }
        }
    }
}

fn cycle_pieces<const N: usize>(pieces: &mut [Piece; N], pos_cycle: &[u8; 4]) -> [usize; 4] {
    // This will contain which pieces will swap places
    let mut piece_cycle: [usize; 4] = [0; 4];

    // fill in piece_cycle
    for i in 0..4 {
        piece_cycle[i] = pieces
            .iter()
            .position(|piece| piece.position == pos_cycle[i] as i32)
            .expect("Piece Positon #{pos_cycle[i]} Empty");
    }

    // actually cycle the pieces
    for i in 0..4 {
        pieces[piece_cycle[i]].position = pos_cycle[(i + 1) % 4] as i32;
    }

    // return for orientation changes
    return piece_cycle;
}

fn get_rotation(coeff: i32, i: usize) -> i32 {
    if coeff == -1 {
        2 - (i as i32 % 2)
    } else {
        1 + (i as i32 % 2)
    }
}
