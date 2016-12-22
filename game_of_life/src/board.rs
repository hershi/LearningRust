use std::iter;

#[derive(Debug)]
pub struct Board {
    grid : Vec<Vec<bool>>,
    cols : usize,
    rows : usize,
}

#[derive(Debug)]
pub struct NeighborsIterator<'a> {
    board : &'a Board,
    points : Vec<(usize,usize)>,
    offset : usize,
}

impl <'a> NeighborsIterator<'a> {
    fn new(board : &Board, x : usize, y : usize) -> NeighborsIterator {
        let tx = x as isize;
        let ty = y as isize;
        let points = (-1..2).flat_map(|i| iter::repeat(i).take(3))
               .zip((-1..2).cycle())
               .filter(|&(i,j)| (i != 0 || j != 0))
               .map(|(i,j)| (tx + i, ty + j))
               .filter(|&(i,j)| i >= 0 && j >= 0)
               .filter(|&(i,j)| i < board.cols as isize && j < board.rows as isize) 
               .map(|(i,j)| (i as usize, j as usize))
               .inspect(|p| println!("D: {:?},", p))
               .collect();

        NeighborsIterator {
            board : board,
            points : points,
            offset : 0,
        }
    }
}

impl <'a> Iterator for NeighborsIterator<'a> {
    type Item = bool;
    fn next(&mut self) -> Option<bool> {
        if self.offset >= self.points.len() { 
            None 
        } else { 
            let point = self.points[self.offset];
            self.offset += 1; 
            Some(self.board.grid[point.0][point.1])
        }
    }
}

pub struct ForwardingNeighborsIterator <'a, ItType> {
    board : &'a Board,
    internal_iterator : ItType,    
}

impl <'a, ItType> ForwardingNeighborsIterator<'a, ItType> {
    fn new(board : &Board, x : usize, y : usize) -> ForwardingNeighborsIterator<'a, ItType> {
        let tx = x as isize;
        let ty = y as isize;        
        ForwardingNeighborsIterator { 
            board : board, 
            internal_iterator :
                (-1..2).flat_map(|i| iter::repeat(i).take(3))
                .zip((-1..2).cycle())
                .filter(|&(i,j)| (i != 0 || j != 0))
                .map(|(i,j)| (tx + i, ty + j))
                .filter(|&(i,j)| i >= 0 && j >= 0)
                .filter(|&(i,j)| i < board.cols as isize && j < board.rows as isize) 
                .map(|(i,j)| board.grid[i as usize][j as usize]) as ItType
        }
    }
}

impl <'a, ItType : Iterator<Item=bool>> Iterator for ForwardingNeighborsIterator<'a, ItType> {
    type Item = bool;
    fn next(&mut self) -> Option<bool> {
        self.internal_iterator.next()
    }
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