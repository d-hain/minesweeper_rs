use nannou::prelude::*;

#[derive(Default, Clone, Copy, Debug)]
struct Cell {
    has_flag: bool,
    is_bomb: bool,
    bomb_count: u8,
    is_revealed: bool,
}

impl Cell {
    pub fn new(is_bomb: bool) -> Self {
        Self {
            is_bomb,
            ..Default::default()
        }
    }
}

#[derive(Debug, Clone)]
struct Field (Vec<Vec<Cell>>);

impl Field {
    pub fn empty(rows: u8, cols: u8) -> Self {
        let field = vec![vec![Cell::new(false); cols as usize];rows as usize];

        Self(field)
    }

    /// Reveals the given points Field
    /// @return If cells is a bomb
    pub fn reveal(&mut self, pos: Point2) -> bool{
        let mut cell = self.0[pos.y as usize][pos.x as usize];
        cell.is_revealed = true;

        cell.is_bomb
    }



    fn count_surrounding_bombs(&self, pos: Point2) -> u8 {
        let mut bombs = 0;
        if pos.x as u32 > 0 {
            bombs += self.0[pos.y as usize][pos.x as usize-1].is_bomb as usize;
        }

        bombs as u8
    }

    fn set_bomb_counts(&mut self) {
        let mut field = self.clone();
        for (y, row) in field.0.iter_mut().enumerate() {
            for (x,  cell) in row.iter_mut().enumerate() {
                cell.bomb_count = self.count_surrounding_bombs(Point2::new(x as f32, y as f32));
            }
        }
    }
}

fn main() {
    nannou::sketch(view).run()
}

fn view(app: &App, frame: Frame) {
    let draw = app.draw();

    draw.background().color(CORNFLOWERBLUE);

    draw.rect()
        .x_y(app.mouse.y, app.mouse.x)
        .w(app.mouse.x * 0.25)
        .hsv(1.0, 1.0, 1.0);

    draw.to_frame(app, &frame).unwrap();
}
