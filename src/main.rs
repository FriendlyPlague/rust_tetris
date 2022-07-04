const WIDTH: usize = 10;
const HEIGHT: usize = 20;

#[derive(Copy, Clone)]
struct Color(u8, u8, u8);

fn main() {
    let mut grid = tetris_grid {grid: [None;200]};
    grid.draw_blocks();
}

struct tetris_grid {
    grid: [Option<Color>; 200],
}

impl tetris_grid {
    fn delete_rows(&mut self, start_r: usize, end_r: usize) {
        /* Clears rows then shifts everything above cleared rows down*/
        for i in WIDTH * start_r..WIDTH * end_r {
            self.grid[i] = None;
        }
        self.grid[0..WIDTH * end_r].rotate_right(WIDTH * (end_r - start_r));
    }

    fn draw_blocks(&self) {
        for y in 0..HEIGHT {
            for x in 0..WIDTH {
                tetris_grid::draw_block(self.grid[x * y], x, y)
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
    