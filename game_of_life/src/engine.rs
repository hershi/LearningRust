use std::iter;
use board::*;

pub fn next_gen(board : &mut Board) {
    let flips = 
        (0..board.cols).flat_map(|x| iter::repeat(x).take(board.rows)).zip((0..board.rows).cycle()) // all coordinates
        .map(|(x,y)| ((x,y), board.grid[x][y], board.neighbors(x,y).filter(|c| *c).count()))          // map to (point, value, count of live neighbors)
        .filter(|&(_, v, live_neighbors)| (v && live_neighbors != 2 && live_neighbors != 3) || (!v && live_neighbors == 3))
        .map(|(p, _, _)| p).collect::<Vec<(usize, usize)>>();

    for f in flips {
        board.flip(f.0, f.1);
    }
}