use std::vec;

struct Cell {
    value: i32,
    row: i32,
    col: i32,
    square: i32
}

impl Cell {
    fn new(value: i32, row: i32, col:i32, square:i32) -> Self { 
      Self { value, row, col, square }                       
    }
}

struct SudokuBoard {
    board: Vec<Cell>
}

impl SudokuBoard {
    fn new(s:&str) -> Self {
        let mut board:Vec<Cell> = vec![];
        for (i , srow) in s.split(';').enumerate() {
            for (j , value) in srow.split(' ').enumerate() {
                let square_number:i32 = (3*(i/3) + j/3) as i32;
                board.push(Cell::new(value.parse::<i32>().unwrap(),i as i32,j as i32, square_number));
            }
        }
        Self {
            board: board
        }
    }

    fn print(&self) {
        for cell in self.board.iter(){ 
            if cell.row % 3 == 0 && cell.col == 0 {
                println!("----------------------");
            }                      
            if cell.col % 3 == 0{
                print!("|");
            }
            match cell.value {
                0 => print!("  "),
                _ => print!("{} ", cell.value)
            }  
            if cell.col % 9 == 8{
                println!("|");
            }
        }
        println!("----------------------");
    }

    fn get_all_values_in_row(&self, c:&Cell) -> Vec<i32> {
        self.board.iter().filter(|x| x.row == c.row && x.value > 0).map(|x| x.value).collect::<Vec<i32>>()
    }

    fn get_all_values_in_col(&self, c:&Cell) -> Vec<i32> {
        self.board.iter().filter(|x| x.col == c.col && x.value > 0).map(|x| x.value).collect::<Vec<i32>>()
    }

    fn get_all_values_in_square(&self, c:&Cell) -> Vec<i32> {
        self.board.iter().filter(|x| x.square == c.square && x.value > 0).map(|x| x.value).collect::<Vec<i32>>()
    }
    
    fn calculate_cell_options(&mut self, i: usize) -> Vec<i32> {       
        let mut options = vec![];        
        for v in 1..10 {
            // if the value doesn't exist in the row, column or square - then it is an option
            if self.get_all_values_in_row(&self.board[i]).iter().any(|x| *x == v) || 
                self.get_all_values_in_col(&self.board[i]).iter().any(|x| *x == v) ||
                self.get_all_values_in_square(&self.board[i]).iter().any(|x| *x == v) {
                continue;
            } 
            options.push(v);
        }         
        options   
    }

    fn rec_solve(&mut self, i: usize) -> bool{
        // all cells contain a valid value
        if i == 81 {
            return true;
        }

        // cell already has a value - solve the next one
        if self.board[i].value > 0{
            return self.rec_solve(i+1);
        }

        let options = self.calculate_cell_options(i);
        for o in options.iter(){
            self.board[i].value = *o;
            if self.rec_solve(i+1) {
                return true;
            }
            self.board[i].value = 0; // if we didn't solve we must restore cell value - since this is an in-place solution.
        }
        return false;
    }
}

fn main() {
    let _debug_easy_board = "0 5 0 2 0 0 0 0 0;0 0 0 0 0 6 7 9 3;4 0 7 8 0 9 0 5 6;3 7 0 0 6 2 4 8 5;6 9 5 0 0 0 1 2 7;8 0 4 5 7 0 0 0 0;0 6 0 0 8 4 0 3 1;7 8 0 0 0 0 0 4 0;1 0 9 0 0 3 5 0 8";
    let _debug_medium_board = "0 0 0 0 4 9 0 7 6;0 0 0 7 0 0 0 4 8;0 7 3 0 0 0 0 0 2;0 6 0 8 9 2 0 0 3;3 8 4 1 5 0 6 2 0;5 0 2 6 0 0 1 8 0;7 4 0 3 6 0 0 0 0;6 1 9 0 0 0 0 0 0;0 0 0 9 7 0 0 0 4";
    let _debug_hard_board = "1 0 7 8 5 0 4 0 0;0 0 0 0 0 6 0 0 5;0 6 0 0 0 0 0 0 8;0 0 0 3 9 7 0 0 0;0 5 6 0 0 0 2 9 0;0 0 0 6 2 5 0 0 0;8 0 0 0 0 0 0 3 0;4 0 0 7 0 0 0 0 0;0 0 2 0 4 8 5 0 7";
    let mut board = SudokuBoard::new(_debug_hard_board);
    println!("Initial Board:");
    board.print();

    if board.rec_solve(0) {
        println!("Solved Board:");
        board.print();
    }
}
