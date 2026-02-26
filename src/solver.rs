use crate::cube::{Cube, Face, Move};
use crate::data::ScrambleRecord;
use crate::pdb::ALL_MOVES;
use crate::scramble::invert_move;

fn analyze_scramble(
    mut record: ScrambleRecord,
    cube: &Cube,
    records_vec: &mut Vec<ScrambleRecord>,
) {
    solve(cube, &mut record);
    records_vec.push(record)
}

fn recursive_scramble(
    record: &mut ScrambleRecord,
    cube: &mut Cube,
    stop_depth: u8,
    records_vec: &mut Vec<ScrambleRecord>,
) {
    // Base Case:
    if record.n == stop_depth {
        analyze_scramble(record.clone(), cube, records_vec);
        return;
    }

    // Recursive Case:
    for mv in ALL_MOVES {
        // Put move onto scramble
        record.scramble.push(mv);

        let mut new_cube = cube.clone();
        new_cube.make_move(mv);

        // Recursive call
        recursive_scramble(record, &mut new_cube, stop_depth, records_vec);

        // Take the move off the scramble
        record.scramble.pop();
    }
}

// in this modified version, this will solve
pub fn solve(cube: &Cube, record: &mut ScrambleRecord) {
    // set the minimum alternate length (threshold) to scramble len to start
    let mut min_alternate_len = u8::MAX;

    let last_move_inv: Move = invert_move(
        *record
            .scramble
            .last()
            .expect("Cannot solve a cube that has no scramble in its record"),
    );

    let previous_face: Option<Face> = None;

    ////////// Add a previous_face: Option<face> to replace path.last() in previous
    search(
        &cube,
        0,
        previous_face,
        &mut min_alternate_len,
        &last_move_inv,
        record,
    );
}

const FACES: [Face; 6] = [Face::U, Face::R, Face::F, Face::L, Face::B, Face::D];

pub const OPPOSITE_FACES: [Face; 6] = [Face::D, Face::L, Face::B, Face::R, Face::F, Face::U];

// helper function for the solver (IDA*)
// REMOVED: "path: &mut Vec<Move>"
//  we don't need the path for the solution in this case
fn search(
    node: &Cube,
    depth: u8,
    prev_face: Option<Face>,
    min_alternate_len: &mut u8,
    last_move_inv: &Move,
    record: &mut ScrambleRecord,
) {
    //// For this modified version, f = depth
    //// this just means that we are not using heuristics, we covering every state in the tree
    //let f = depth;
    //// So, I am just using depth instead for all instances of f

    // prune anything above the min_alternate_len
    if depth > *min_alternate_len {
        return;
    }

    // Increment how many nodes have been expanded
    record.nodes_expanded += 1;

    // If solved, stop searching this path
    if node.is_solved() {
        // update min_alternate_len if not updated yet
        if *min_alternate_len != depth {
            *min_alternate_len = depth;
            record.min_length = depth;

            // Shouldn't need to do this...
            if record.count != 0 {
                panic!("Went too far somehow")
            }
        }

        record.count += 1;

        return;
    }

    // Check all moves
    for &face in FACES.iter() {
        // Prevent redundant moves
        // THIS PREVENTS TOO MANY MOVES
        if let Some(prev) = prev_face {
            if face == prev || face == OPPOSITE_FACES[prev as usize] {
                continue;
            }
        }

        // Iterate over coefficients (-1 = CW, 1 = CCW, 2 = Double Turn)
        for coeff in [-1, 1, 2] {
            // Prevent first move being inverse of scramble
            // checking "prev_face == None" makes sure that there are no moves yet 
            if prev_face == None && face == last_move_inv.face && coeff == last_move_inv.coeff {
                continue;
            }

            let mv = Move { face, coeff };

            // Apply move
            let mut new_node = node.clone();
            new_node.make_move(mv);

            // // Push to path
            // path.push(mv);

            // Recursive search
            //let t =
            search(
                &new_node,
                depth + 1,
                Some(mv.face),
                min_alternate_len,
                last_move_inv,
                record,
            );

            // // If found, go to top
            // if t == -1 {
            //     return -1;
            // }

            // // Track minimum cutoff cost
            // if t < min_cost {
            //     min_cost = t;
            // }

            // // Backtrack
            // path.pop();
        }
    }
}
