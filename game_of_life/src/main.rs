extern crate game_of_life;
use game_of_life::board::*;

fn main() {
    let b = Board::new(10,10);
    for x in b.neighbors(1,1).take(10) {
        println!("{:?}", x);
    }

    println!("{:?}", b.neighbors(1,1).count());
    println!("{:?}", b.neighbors(0,0).count());
    println!("{:?}", b.neighbors(9,9).count());
    println!("{:?}", b.neighbors(8,9).count());
    println!("{:?}", b.neighbors(9,8).count());
}
