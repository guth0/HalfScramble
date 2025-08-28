mod cube;
mod solver;

use cube::Cube;
use cube::Face;
use solver::solve;

fn main() {
    println!("--------------------------");

    let mut cube = Cube::new();

    cube.print();

    cube.make_move(Face::F, 2);

    cube.make_move(Face::U, 1);
    cube.make_move(Face::L, 2);


    cube.print();


    match solve(&cube)
    {
      Some(path) => println!("PATH: {:?}", path),
      None => println!("No path found :(")
    }
}
