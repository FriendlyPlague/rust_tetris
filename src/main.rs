const WIDTH: usize = 10;
const HEIGHT: usize = 20;

#[derive(Copy, Clone)]
struct Color(u8, u8, u8);

fn main() {
    let the_grid = TetrisGrid { grid: [None; 200] };
    let test = Piece::S;
    let mut grid = test.new();
    
    for i in 0..5 {
        draw_piece_test(&grid);
        rotate_right(&mut grid);
    }
    // draw_blocks(&the_grid.grid);
}

enum Piece {
    O,
    I,
    S,
    Z,
    L,
    J,
    T,
}

impl Piece {
    pub fn new (&self) -> ([Option<Color>; 9]) {
        (self.generate_piece())
    }

    fn color(&self) -> Color {
        match *self {
            Piece::O => Color(208, 245, 22),
            Piece::I => Color(9, 180, 214),
            Piece::S => Color(232, 12, 15),
            Piece::Z => Color(5, 153, 24),
            Piece::L => Color(245, 178, 22),
            Piece::J => Color(240, 31, 205),
            Piece::T => Color(113, 6, 158),
        }
    }

    #[rustfmt::skip]
    fn generate_piece(&self) -> [Option<Color>; 9] {
        let c = self.color();
        match self {
            Piece::I => [None, Some(c), None,
                        None, Some(c), None,
                        None, Some(c), None],
            Piece::O => [Some(c),Some(c),None,
                        Some(c),Some(c),None,
                        None, None, None],
            Piece::S => [None,  Some(c),Some(c),
                        Some(c), Some(c), None,
                        None,None,None],
            Piece::Z =>[Some(c),Some(c), None,
                        None, Some(c), Some(c),
                        None, None, None],
            Piece::L => [Some(c),None,None,
                        Some(c), None, None,
                        Some(c),Some(c),None],
            Piece::J => [None, None,Some(c),
                        None, None, Some(c),
                        None,Some(c),Some(c)],
            Piece::T => [Some(c),Some(c),Some(c),
                        None, Some(c), None,
                        None, None, None],
        }
    }
}

fn rotate_right(area: &mut [Option<Color>; 9]) {
    let mut new_area: [Option<Color>; 9] = [None; 9]; //think array of 3 by 3 quandrant 4
    for x in 0..3 {
        new_area[x * 3 + 2] = area[x]; // sets right 3 to top 3
    }
    for y in 0..3 {
        new_area[8 - y] = area[y * 3 + 2]; // sets bottom 3 to right 3
    }
    for x in 0..3 {
        new_area[x * 3] = area[6 + x]; // sets left 3 to bottom 3
    }
    for y in 0..3 {
        new_area[2 - y] = area[y * 3]; // sets top 3 right 3
    }
    new_area[4] = area[4]; // sets middle to middle
    *area = new_area;
}



struct TetrisGrid {
    grid: [Option<Color>; 200],
}

impl TetrisGrid {
    fn delete_rows(&mut self, start_r: usize, end_r: usize) {
        /* Clears rows then shifts everything above cleared rows down*/
        for i in WIDTH * start_r..WIDTH * end_r {
            self.grid[i] = None;
        }
        self.grid[0..WIDTH * end_r].rotate_right(WIDTH * (end_r - start_r));
    }

    fn check_lines(&self) -> Option<(usize, usize)> {
        /* if there is a row to be deleted, it returns  the ammount
        of rows detected and the starting row location*/
        let mut tetris_count = 0;
        for y in 0..HEIGHT {
            let mut full_row = true;
            for x in 0..WIDTH {
                if self.grid[x * y].is_none() {
                    full_row = false;
                    break;
                }
            }
            if full_row {
                tetris_count += 1;
            } else if tetris_count > 0 {
                return Some((tetris_count, y - tetris_count));
            }
        }
        if tetris_count > 0 {
            Some((tetris_count, HEIGHT - tetris_count))
        } else {
            None
        }
    }
}
fn draw_blocks(grid: &[Option<Color>; WIDTH * HEIGHT]) {
    for y in 0..HEIGHT {
        for x in 0..WIDTH {
            draw_block(grid[x * y], x, y)
        }
        println!();
    }
}

fn draw_block(color: Option<Color>, x: usize, y: usize) {
    match color {
        Some(_) => print!("X"),
        None => print!("O"),
    }
}

fn draw_piece_test(grid: &[Option<Color>; 9]) {
    for y in 0..3 {
        for x in 0..3 {
            draw_block(grid[y*3 + x], x, y);
        }
        println!();
    }
}