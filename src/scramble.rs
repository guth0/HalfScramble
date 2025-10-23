use rand::Rng;

use crate::cube::{Face, Move};
use crate::pdb::ALL_MOVES;
use crate::solver::OPPOSITE_FACES;

// generates a random scramble of length = len
pub fn generate_scramble(len: i32) -> Vec<Move> {
    let mut rng = rand::rng();

    let mut scramble: Vec<Move> = Vec::new();

    for i in 0..len {
        // slect a random move from ALL_MOVES
        let mut mv = ALL_MOVES[rng.random_range(0..18)];

        // shitty if statement to make sure moves are not reversing themselves
        while (i > 0 && mv.face == scramble[(i - 1) as usize].face)
            || (i > 1
                && mv.face == OPPOSITE_FACES[scramble[(i - 1) as usize].face as usize]
                && mv.face == scramble[(i - 2) as usize].face)
        {
            mv = ALL_MOVES[rng.random_range(0..18)];
        }

        scramble.push(mv);
    }

    scramble
}

// returns the inverse of a given move
//  invert_move( U ) = U'
//  invert_move( F' ) = F
//  invert_move( 2D ) = 2D
pub fn invert_move(mv: Move) -> Move {
    Move {
        face: mv.face,
        coeff: if mv.coeff == 2 { 2 } else { -mv.coeff },
    }
}

// Takes a path P and returns an inverted path P'
//  such that if you apply P and P' to a cube, it will return to its origional state
pub fn invert_path(path: &[Move]) -> Vec<Move> {
    let mut reversed_path: Vec<Move> = Vec::new();

    for mv in path.iter().rev() {
        reversed_path.push(invert_move(*mv));
    }

    reversed_path
}

fn print_move(mv: &Move) {
    match mv.face {
        Face::U => print!("U"),
        Face::F => print!("F"),
        Face::R => print!("R"),
        Face::L => print!("L"),
        Face::B => print!("B"),
        Face::D => print!("D"),
    }

    match mv.coeff {
        -1 => print!("`"),
        2 => print!("2"),
        _ => (),
    }
}

pub fn print_path(path: &[Move]) {
    for mv in path {
        print_move(mv);
        print!(" ");
    }
    println!();
}
