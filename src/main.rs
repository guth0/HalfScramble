mod cube;

use cube::Cube;
use cube::Face;

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
