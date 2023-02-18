use nannou::image::Rgb;
use nannou::prelude::*;
use rand::Rng;

const MAX_ROWS: u8 = 10;
const MAX_COLS: u8 = 10;
const BOMB_COUNT: u8 = 10;

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
struct Field(Vec<Vec<Cell>>);

impl Field {
    /// Create an empty [`Field`] without any bombs.
    pub fn empty(rows: u8, cols: u8) -> Self {
        let field = vec![vec![Cell::new(false); cols as usize]; rows as usize];

        Self(field)
    }

    fn get(&self, pos: Point2) -> Cell {
        self.0[pos.y as usize][pos.x as usize]
    }

    pub fn place_bombs(&mut self, bomb_count: u8) {
        let mut rand_y;
        let mut rand_x;
        let mut cell;
        for _ in 0..bomb_count {
            loop {
                rand_y = rand::thread_rng().gen_range(0..self.0.len());
                rand_x = rand::thread_rng().gen_range(0..self.0[0].len());
                cell = &mut self.0[rand_y][rand_x];
                if !cell.is_bomb { break; }
            }
            cell.is_bomb = true;
        }
    }

    /// Reveals the given [`Point2`] in the [`Field`].
    ///
    /// # Returns
    ///
    /// if the [`Cell`] is a bomb
    pub fn reveal(&mut self, pos: Point2) -> bool {
        let mut cell = self.get(pos);
        cell.is_revealed = true;

        cell.is_bomb
    }

    fn count_surrounding_bombs(&self, pos: Point2) -> u8 {
        let mut bombs = 0;
        if pos.x as u8 > 0 {
            let top = Point2::new(pos.x-1.0, pos.y);
            bombs += self.get(top).is_bomb as usize;

            if pos.y as u8 > 0 {
                let top_left = Point2::new(pos.x-1.0, pos.y-1.0);
                let left  = Point2::new(pos.x, pos.y-1.0);

                bombs += self.get(top_left).is_bomb as usize;
                bombs += self.get(left).is_bomb as usize;
            }

            if (pos.y as u8) < MAX_ROWS - 1 {
                let top_right = Point2::new(pos.x-1.0, pos.y+1.0);
                let right = Point2::new(pos.x, pos.y+1.0);

                bombs += self.get(top_right).is_bomb as usize;
                bombs += self.get(right).is_bomb as usize;
            }
        }

        if (pos.x as u8) < MAX_COLS {
            let bottom = Point2::new(pos.x+1.0, pos.y);
            bombs += self.get(bottom).is_bomb as usize;
            if pos.y as u8 > 0 {
                let bottom_left = Point2::new(pos.x+1.0, pos.y-1.0);

                bombs += self.get(bottom_left).is_bomb as usize;
            }

            if (pos.y as u8) < MAX_ROWS - 1 {
                let bottom_right = Point2::new(pos.x+1.0, pos.y+1.0);

                bombs += self.get(bottom_right).is_bomb as usize;
            }
        }

        bombs as u8
    }

    fn set_bomb_counts(&mut self) {
        let mut field = self.clone();
        for (y, row) in field.0.iter_mut().enumerate() {
            for (x, cell) in row.iter_mut().enumerate() {
                cell.bomb_count = self.count_surrounding_bombs(Point2::new(x as f32, y as f32));
            }
        }
    }

    /// Draw the [`Field`] in the middle of the `draw`.
    pub fn draw(&self, window_rect: &Rect, draw: &Draw) {
        let cell_width = window_rect.w() / (self.0.len() as f32 * 2.0);
        let cell_height = window_rect.h() / (self.0.len() as f32 * 2.0);
        let padding_x = cell_width / 4.0;
        let padding_y = cell_height / 4.0;
        let field_width = cell_width * self.0.len() as f32 + padding_x * (self.0.len() as f32 - 1.0);
        let field_height = cell_height * self.0.len() as f32 + padding_y * (self.0.len() as f32 - 1.0);
        let remaining_window_width = window_rect.w() - field_width;
        let remaining_window_height = window_rect.h() - field_height;

        for (y, row) in self.0.iter().enumerate() {
            for (x, cell) in row.iter().enumerate() {
                let cell_x_pos = remaining_window_width / 2.0 + cell_width * x as f32 + padding_x * x as f32;
                let cell_y_pos = remaining_window_height / 2.0 + cell_height * y as f32 + padding_y * y as f32;

                let (mut r, g, b) = (0.0, 1.0, 0.0);
                if cell.is_bomb {
                    r = 1.0;
                }

                draw.rect()
                    .x_y(cell_x_pos, cell_y_pos)
                    .w_h(cell_width, cell_height)
                    .rgb(r, g, b);
            }
        }
    }
}

struct Model {
    field: Field,
}

fn main() {
    nannou::app(model).update(update).run();
}

/// Creates the window and sets up the [`Model`].
fn model(app: &App) -> Model {
    let _window_id = app
        .new_window()
        .title("minesweeper_rs")
        .size(800, 800)
        .view(view)
        .build()
        .unwrap();

    let mut field = Field::empty(MAX_ROWS, MAX_COLS);
    field.place_bombs(BOMB_COUNT);

    Model {
        field,
    }
}

fn update(_app: &App, model: &mut Model, _update: Update) {}

/// Draws once a frame to the window.
fn view(app: &App, model: &Model, frame: Frame) {
    let mut draw = app.draw();
    // Change Origin Point to bottom left
    draw = draw.x_y(-app.window_rect().w() * 0.5, -app.window_rect().h() * 0.5);
    draw.background().color(CORNFLOWERBLUE);

    model.field.draw(&app.window_rect(), &draw);

    draw.to_frame(app, &frame).unwrap();
}
