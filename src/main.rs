use std::array;

fn main() {
    println!("LETS GOOO");

    let cube = Cube::new();

    cube.print();
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

    state[0][1][1] = 'W';
    state[1][1][1] = 'R';
    state[2][1][1] = 'G';
    state[3][1][1] = 'Y';
    state[4][1][1] = 'O';
    state[5][1][1] = 'B';

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
        let index = cube
            .edges
            .iter()
            .position(|piece| piece.position == pos)
            .expect("Edge Position #{pos} Empty"); // Panics if no peice is found

        for i in 0..2 {
            let sticker_pos: &(Face, u8, u8) = &EDGE_TABLE[pos as usize][i];


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

const CORNER_COLORS: [&str; 8] = ["WRG", "WOG", "YOG", "YRG", "WRB", "WOB", "YOB", "YRB"];
const EDGE_COLORS: [&str; 12] = ["WG", "WR", "WB", "WO", "GR", "BR", "BO", "GO", "YG", "YR", "YB", "YO"];

#[repr(u8)]
#[derive(Clone, Copy)]
enum Face {
    U,
    R,
    F,
    D,
    L,
    B,
}

const CORNER_TABLE: [[(Face, u8, u8); 3]; 8] = [
    // Order is always going to be [(U/D), (R/L), (F/B)]
    //   The tuple is always (Face, row, column)
    [(Face::U, 2, 2), (Face::R, 0, 0), (Face::F, 0, 2)],
    [(Face::U, 2, 0), (Face::L, 0, 2), (Face::F, 0, 0)],
    [(Face::D, 0, 0), (Face::L, 2, 2), (Face::F, 2, 0)],
    [(Face::D, 0, 2), (Face::R, 2, 0), (Face::F, 2, 2)],
    [(Face::U, 0, 2), (Face::R, 0, 2), (Face::B, 0, 0)],
    [(Face::U, 0, 0), (Face::L, 0, 0), (Face::B, 0, 2)],
    [(Face::D, 2, 0), (Face::L, 2, 0), (Face::B, 2, 2)],
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

struct Cube {
    // 2 arrays of pieces
    corners: [Piece; 8],
    edges: [Piece; 12],
}

impl Cube {
    fn new() -> Self {
        // this creates the object and retuns it
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
}
