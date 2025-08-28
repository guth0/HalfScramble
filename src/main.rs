mod cube;
mod solver;

use cube::Cube;
use cube::Face;
use solver::solve;
use solver::Move;

fn main() {
    println!("--------------------------");

    let mut cube = Cube::new();

    cube.print();

    cube.make_move(Face::F, 2);

    cube.make_move(Face::U, 1);
    cube.make_move(Face::L, 2);
    cube.make_move(Face::B, 2);


    cube.print();

    // This is to prevent the solution from being the inverse of the scramble
    let last_move_inv: Move = Move {face: Face::B, coeff: 2};

    match solve(&cube, last_move_inv)
    {
      Some(path) => println!("PATH: {:?}", path),
      None => println!("No path found :(")
    }
}
