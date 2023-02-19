use nannou::image::Rgb;
use nannou::prelude::*;
use rand::Rng;

const MAX_ROWS: u8 = 10;
const MAX_COLS: u8 = 10;
const BOMB_COUNT: u8 = 10;
const CELL_COLOR: CellColor = CellColor::new(0.0, 1.0, 0.0);
const BOMB_COLOR: CellColor = CellColor::new(1.0, 0.0, 0.0);

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

struct CellColor {
    r: f32,
    g: f32,
    b: f32,
}

impl CellColor {
    const fn new(r: f32, g: f32, b: f32) -> Self {
        Self {
            r,
            g,
            b,
        }
    }
}

impl From<CellColor> for (f32, f32, f32) {
    fn from(value: CellColor) -> Self {
        (value.r, value.g, value.g)
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

        self.set_bomb_counts();
    }

    fn in_field(&self, pos: Point2) -> bool {
        if pos.x < 0.0 || pos.y < 0.0 {
            return false;
        }
        (pos.x as u8) < MAX_COLS && (pos.y as u8) < MAX_ROWS
    }

    fn get_neighbor_positions(&self, pos: &Point2) -> Vec<Point2> {
        let mut neighbor_positions = vec![];
        for offset_y in -1..2 {
            for offset_x in -1..2 {
                let neighbor = *pos + Point2::new(offset_x as f32, offset_y as f32);
                if self.in_field(neighbor) && neighbor != *pos {
                    neighbor_positions.push(neighbor);
                }
            }
        }
        neighbor_positions
    }

    /// Reveals the given points Field
    /// @return If cells is a bomb
    pub fn reveal(&mut self, pos: &Point2) -> bool{
        let mut cell = self.get(*pos);
        cell.is_revealed = true;
        if cell.bomb_count > 0 {
            return cell.is_bomb;
        } else {
            for neighbor_pos in self.get_neighbor_positions(&pos).iter() {
                self.reveal(neighbor_pos);
            }
        }
        false
    }

    fn check_win(self) -> bool {
        let flattened = self.0.into_iter().flatten().collect::<Vec<Cell>>();
        flattened.iter().map(|e| e.is_bomb as u8).sum::<u8>() == BOMB_COUNT
    }

    fn count_surrounding_bombs(&self, pos: Point2) -> u8 {
        self.get_neighbor_positions(&pos).iter().map(|pos| self.get(*pos).is_bomb as u8).sum::<u8>()
    }

    fn set_bomb_counts(&mut self) {
        let field = self.clone();
        for (y, row) in self.0.iter_mut().enumerate() {
            for (x, cell) in row.iter_mut().enumerate() {
                if !cell.is_bomb {
                    cell.bomb_count = field.count_surrounding_bombs(Point2::new(x as f32, y as f32));
                }
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

                let (mut r, mut g, mut b) = CELL_COLOR.into();
                if cell.is_bomb {
                    (r, g, b) = BOMB_COLOR.into();
                }

                draw.rect()
                    .x_y(cell_x_pos, cell_y_pos)
                    .w_h(cell_width, cell_height)
                    .rgb(r, g, b);
                if cell.bomb_count > 0 {
                    draw.text(&cell.bomb_count.to_string())
                        .x_y(cell_x_pos, cell_y_pos)
                        .w_h(cell_width, cell_height)
                        .font_size((cell_width/3.0) as u32 )
                        .align_text_middle_y()
                        .color(BLACK);
                }
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
