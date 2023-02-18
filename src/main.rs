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

#[derive(Debug)]
struct Field (Vec<Vec<Cell>>);

impl Field {
    /// Create an empty [`Field`] without any bombs
    pub fn empty(rows: u8, cols: u8) -> Self {
        let field = vec![vec![Cell::new(false); cols as usize];rows as usize];

        Self(field)
    }
    
    /// Draw the [`Field`] in the middle of the `draw`
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
                
                draw.rect()
                    .x_y(cell_x_pos, cell_y_pos)
                    .w_h(cell_width, cell_height)
                    .rgb(0.0, 1.0, 0.0);
                
                if cell.is_bomb {
                    draw.rect()
                        .x_y(cell_x_pos, cell_y_pos)
                        .w_h(cell_width, cell_height)
                        .rgb(1.0, 1.0, 0.0);
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

/// Creates the window and sets up the [`Model`]
fn model(app: &App) -> Model {
    let _window_id = app
        .new_window()
        .title("minesweeper_rs")
        .size(800, 800)
        .view(view)
        .build()
        .unwrap();

    let mut field = Field::empty(10, 10);
    
    Model {
        field,
    }
}

fn update(_app: &App, model: &mut Model, _update: Update) {
    
}

/// Draws once a frame to the window
fn view(app: &App, model: &Model, frame: Frame) {
    let mut draw = app.draw();
    // Change Origin Point to bottom left
    draw = draw.x_y(-app.window_rect().w() * 0.5, -app.window_rect().h() * 0.5);
    draw.background().color(CORNFLOWERBLUE);
    
    model.field.draw(&app.window_rect(), &draw);

    draw.to_frame(app, &frame).unwrap();
}
