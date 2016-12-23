use std::iter;
use iterators::NeighborsIterator;

#[derive(Debug)]
pub struct Board {
    pub grid : Vec<Vec<bool>>,
    pub cols : usize,
    pub rows : usize,
}

impl Board {
    pub fn new(width : usize, height : usize) -> Board {
        if width == 0 || height == 0 { panic!("Invalid board size: Array dimensions must be non-zero {} {}", width, height); }

        let mut res = Vec::with_capacity(width);
        for _ in 0..width {
            res.push(iter::repeat(false).take(height).collect());
        }

        Board{grid : res, cols : width, rows : height}
    }

    pub fn flip(&mut self, col : usize, row : usize){
        self.grid[col][row] = !self.grid[col][row];
    }

    pub fn neighbors(&self, x : usize, y : usize) -> NeighborsIterator {
        NeighborsIterator::new(self, x, y)
    }

    pub fn print(&self) {
        for x in 0..self.cols {
            for y in 0..self.rows {
                print!("{}", if self.grid[x][y] { "*" } else { "-" });
            }
            println!("");
        }        
    }
}

#[cfg(test)]
mod tests {
    use board::*;
    use std::iter;

    #[test]
    fn test_basic_creation() {
        let width = 30; 
        let height = 27;
        let b = Board::new(width,height);
        assert_eq!(b.cols, width);
        assert_eq!(b.rows, height);
        assert_eq!(b.grid.len(), width);
        for col in b.grid {
            assert_eq!(col.len(), height);
        } 
    }

    #[test]
    #[should_panic(expected = "Invalid board size")]
    fn test_empty_board_creation() {
        let b = Board::new(0,0);
        println!("{:?}", b);
    }

    #[test]
    fn test_one_x_one_creation() {
        let mut b = Board::new(1,1);

        b.grid[0][0] = true;
        assert_eq!(b.grid[0][0], true);
        b.grid[0][0] = false;
        assert_eq!(b.grid[0][0], false);
    }

    #[test]
    fn test_population() {
        let width = 1999;
        let height = 1000;
        let mut b = Board::new(width, height);

        for point in (0..width).flat_map(|x| iter::repeat(x).take(height)).zip((0..height).cycle()) {
            let x = point.0;
            let y = point.1; 
            b.grid[x][y] = (x + y) % 2 == 0;
        }

        for point in (0..width).flat_map(|x| iter::repeat(x).take(height)).zip((0..height).cycle()) {
            let x = point.0;
            let y = point.1; 
            assert_eq!(b.grid[x][y], (x + y) % 2 == 0); 
        }
    }

    #[test]
    fn test_flip() {
        let width = 1999;
        let height = 1000;
        let mut b = Board::new(width, height);

        for point in (0..width).flat_map(|x| iter::repeat(x).take(height)).zip((0..height).cycle()) {
            let x = point.0;
            let y = point.1; 
            b.grid[x][y] = (x + y) % 2 == 0; 
        }

        for point in (0..width).flat_map(|x| iter::repeat(x).take(height)).zip((0..height).cycle()) {
            b.flip(point.0, point.1);
        }
        
        for point in (0..width).flat_map(|x| iter::repeat(x).take(height)).zip((0..height).cycle()) {
            let x = point.0;
            let y = point.1; 
            assert_eq!(b.grid[x][y], (x + y) % 2 != 0); 
        }
    }

    #[test]
    #[should_panic]
    fn test_out_of_bounds_col_get() {
        let b = Board::new(10,10);
        println!("{:?}", b.grid[15][5]);         
    }

    #[test]
    #[should_panic]
    fn test_out_of_bounds_row_get() {
        let b = Board::new(10,10);
        println!("{:?}", b.grid[5][15]);         
    }

    #[test]
    #[should_panic(expect = "out of bounds access")]
    fn test_out_of_bounds_col_set() {
        let mut b = Board::new(10,10);
        b.grid[15][5] = true;                
    }

    #[test]
    #[should_panic(expect = "out of bounds access")]
    fn test_out_of_bounds_row_set() {
        let mut b = Board::new(10,10);
        b.grid[5][15] = true;                
    }

    #[test]
    fn test_neighbors_iterator_all_empty() {
        let b = Board::new(10,10);
        for x in b.neighbors(1,1) {
            println!("{:?}", x);
            assert_eq!(x, false);
        }
    }

    #[test]
    fn test_boundary_iterator() {
        let b = Board::new(10,10);
        assert_eq!(b.neighbors(1,1).count(), 8);
        assert_eq!(b.neighbors(0,0).count(), 3);
        assert_eq!(b.neighbors(0,9).count(), 3);
        assert_eq!(b.neighbors(9,0).count(), 3);
        assert_eq!(b.neighbors(9,9).count(), 3);
        assert_eq!(b.neighbors(0,3).count(), 5);
        assert_eq!(b.neighbors(3,0).count(), 5);
        assert_eq!(b.neighbors(8,9).count(), 5);
        assert_eq!(b.neighbors(9,8).count(), 5);
    }

    #[test]
    fn test_neighbors_iterator_all_full() {

    }

    #[test]
    fn test_neighbors_iterator_mixed() {

    }    
}