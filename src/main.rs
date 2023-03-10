use std::time::SystemTime;
use nannou::prelude::*;
use nannou::winit::event::VirtualKeyCode;
use nannou_egui::{self, Egui, egui};
use rand::Rng;

const DEFAULT_FIELD_ROWS: u32 = 10;
const DEFAULT_FIELD_COLS: u32 = 10;
const CELL_COLOR: CellColor = CellColor::new(0.0, 1.0, 0.0);
const BOMB_COLOR: CellColor = CellColor::new(1.0, 0.0, 0.0);
const REVEALED_COLOR: CellColor = CellColor::new(0.69, 0.69, 0.69);

#[derive(Default, Clone, Copy, Debug)]
struct Cell {
    has_flag: bool,
    is_bomb: bool,
    bomb_count: u32,
    is_revealed: bool,
}

impl Cell {
    /// Constructs a new [`Cell`].
    pub fn new(is_bomb: bool) -> Self {
        Self {
            is_bomb,
            ..Default::default()
        }
    }
}

/// The color of a [`Cell`].
struct CellColor {
    r: f32,
    g: f32,
    b: f32,
}

impl CellColor {
    /// Constructs a new [`CellColor`] using the given RGB values.
    const fn new(r: f32, g: f32, b: f32) -> Self {
        Self { r, g, b }
    }
}

impl From<CellColor> for (f32, f32, f32) {
    fn from(value: CellColor) -> Self {
        (value.r, value.g, value.b)
    }
}

/// The [`Field`] of the game containing all [`Cell`]s.
#[derive(Debug, Clone)]
struct Field(Vec<Vec<Cell>>);

impl Field {
    /// Create an empty [`Field`] without any bombs.
    pub fn empty(rows: u32, cols: u32) -> Self {
        let field = vec![vec![Cell::new(false); cols as usize]; rows as usize];

        Self(field)
    }

    /// Get the [`Cell`] at the given `position`.
    fn get(&self, position: Point2) -> Cell {
        self.0[position.y as usize][position.x as usize]
    }

    /// Get a mutable reference to the [`Cell`] at the given `position`.
    fn get_mut(&mut self, position: Point2) -> &mut Cell {
        &mut self.0[position.y as usize][position.x as usize]
    }

    /// # Returns 
    ///
    /// the amount of columns in the [`Field`].
    fn cols(&self) -> u32 {
        self.0[0].len() as u32
    }

    /// # Returns 
    ///
    /// the amount of rows in the [`Field`].
    fn rows(&self) -> u32 {
        self.0.len() as u32
    }

    /// # Returns
    ///
    /// the bomb count calculated by rows and columns
    fn bomb_count(&self) -> u32 {
        (self.rows() * self.cols()) / 10
    }

    /// Place the given `bomb_amount` at random points in the [`Field`].
    pub fn place_bombs(&mut self) {
        let bomb_count = self.bomb_count();

        let mut rand_y;
        let mut rand_x;
        let mut cell;
        for _ in 0..bomb_count {
            loop {
                rand_y = rand::thread_rng().gen_range(0..self.rows()) as usize;
                rand_x = rand::thread_rng().gen_range(0..self.cols()) as usize;
                cell = &mut self.0[rand_y][rand_x];
                if !cell.is_bomb {
                    break;
                }
            }
            cell.is_bomb = true;
        }

        self.set_bomb_counts();
    }

    /// # Returns
    ///
    /// if the given `position` is in the [`Field`].
    fn in_field(&self, position: Point2) -> bool {
        if position.x < 0.0 || position.y < 0.0 {
            return false;
        }
        (position.x as u32) < self.cols() && (position.y as u32) < self.rows()
    }

    /// Get the positions of the neighbors of the [`Cell`] at the given `position`.
    fn get_neighbor_positions(&self, position: &Point2) -> Vec<Point2> {
        let mut neighbor_positions = vec![];
        for offset_y in -1..2 {
            for offset_x in -1..2 {
                let neighbor = *position + Point2::new(offset_x as f32, offset_y as f32);
                if self.in_field(neighbor) && neighbor != *position {
                    neighbor_positions.push(neighbor);
                }
            }
        }
        neighbor_positions
    }

    /// Reveals the given [`Point2`] in the [`Field`].
    ///
    /// # Returns
    ///
    /// if the [`Cell`] is a bomb.
    pub fn reveal(&mut self, pos: &Point2) -> bool {
        let mut cell = self.get_mut(*pos);
        cell.is_revealed = true;
        if cell.is_bomb {
            return true;
        }
        if cell.bomb_count > 0 {
            return false;
        } else {
            for neighbor_pos in self
                .get_neighbor_positions(pos)
                .iter()
                .filter(|e| !self.get(**e).is_revealed)
                .collect::<Vec<&Point2>>()
            {
                self.reveal(neighbor_pos);
            }
        }
        false
    }

    /// Reveals all neighbors of the [`Cell`] at the given `position`
    ///
    /// # Returns
    ///
    /// true if the player checked with incorrect flags.
    fn reveal_neighbors(&mut self, position: Point2) -> bool {
        let surrounding_flags = self.count_surrounding_flags(&position);
        if surrounding_flags == 0 {
            return false;
        }

        let neighbors = self.get_neighbor_positions(&position);
        if self.get(position).bomb_count == self.count_surrounding_flags(&position)
            && !neighbors.iter().filter(|point| self.get(**point).is_bomb && !self.get(**point).has_flag).collect::<Vec<&Point2>>().is_empty() {
            return true;
        }

        for neighbor_pos in neighbors {
            let neighbor = self.get(neighbor_pos);
            if !neighbor.is_revealed && !neighbor.has_flag {
                self.reveal(&neighbor_pos);
            }
        }

        false
    }

    /// Reveals all [`Cell`]s in the [`Field`].
    fn reveal_all(&mut self) {
        for mut cell in self.0.iter_mut().flatten().collect::<Vec<&mut Cell>>() {
            cell.is_revealed = true;
        }
    }

    /// # Returns
    ///
    /// the count of the surrounding flags of the [`Cell`] at the given `position`.
    fn count_surrounding_flags(&self, position: &Point2) -> u32 {
        self.get_neighbor_positions(position)
            .iter()
            .map(|point| self.get(*point).has_flag as u32)
            .sum::<u32>()
    }

    /// # Returns
    ///
    /// the count of the surrounding bombs of the [`Cell`] at the given `position`.
    fn count_surrounding_bombs(&self, position: Point2) -> u32 {
        self.get_neighbor_positions(&position)
            .iter()
            .map(|pos| self.get(*pos).is_bomb as u32)
            .sum::<u32>()
    }

    /// # Returns
    ///
    /// if the game has been won.
    fn check_win(&self) -> bool {
        let flattened = self.0.iter().flatten().collect::<Vec<&Cell>>();
        flattened.iter().map(|e| !e.is_revealed as u32).sum::<u32>() == self.bomb_count()
    }

    /// Sets the bomb_count property of all [`Cell`]s that are no bombs.
    fn set_bomb_counts(&mut self) {
        let field = self.clone();
        for (y, row) in self.0.iter_mut().enumerate() {
            for (x, cell) in row.iter_mut().enumerate() {
                if !cell.is_bomb {
                    cell.bomb_count =
                        field.count_surrounding_bombs(Point2::new(x as f32, y as f32));
                }
            }
        }
    }

    /// Sets the flag of the [`Cell`] at the given `position`.
    pub fn toggle_flag(&mut self, position: &Point2) {
        self.get_mut(*position).has_flag = !self.get_mut(*position).has_flag;
    }

    /// Draw the [`Field`] in the middle of the `draw`.
    pub fn draw(&self, model: &Model, draw: &Draw) {
        for (y, row) in self.0.iter().enumerate() {
            for (x, cell) in row.iter().enumerate() {
                // Construct Cell Rect
                let cell_x_pos = model.field_margin_x + model.cell_width * x as f32;
                let cell_y_pos = model.field_margin_y + model.cell_height * y as f32;

                // Determine Cell color
                let (mut r, mut g, mut b) = CELL_COLOR.into();
                if cell.is_bomb && cell.is_revealed {
                    (r, g, b) = BOMB_COLOR.into();
                } else if cell.is_revealed {
                    (r, g, b) = REVEALED_COLOR.into();
                }

                // Draw the Cell
                draw.rect()
                    .x_y(cell_x_pos, cell_y_pos)
                    .w_h(model.cell_width, model.cell_height)
                    .stroke(BLACK)
                    .stroke_weight(1.0)
                    .rgb(r, g, b);

                if cell.bomb_count > 0 && cell.is_revealed {
                    draw.text(&cell.bomb_count.to_string())
                        .x_y(cell_x_pos, cell_y_pos)
                        .w_h(model.cell_width, model.cell_height)
                        .font_size((model.cell_width / 2.0) as u32)
                        .align_text_middle_y()
                        .color(BLACK);
                }

                if cell.has_flag {
                    draw.tri().color(STEELBLUE).points(
                        Point2::new(
                            cell_x_pos - model.cell_width / 2.0,
                            cell_y_pos - model.cell_height / 2.0,
                        ),
                        Point2::new(
                            cell_x_pos - model.cell_width / 2.0,
                            cell_y_pos + model.cell_height / 2.0,
                        ),
                        Point2::new(cell_x_pos + model.cell_width / 2.0, cell_y_pos),
                    );
                }
            }
        }
        if model.won || model.lost {
            let message = if model.won {
                "Wow! You won OMG MLG"
            } else {
                "looser"
            };
            draw.text(message)
                .x_y(model.field_margin_x + model.field_width / 2.0, model.field_height + model.field_margin_y * 1.5)
                .w_h(model.field_width, model.field_margin_y)
                .font_size(model.cell_width as u32)
                .align_text_middle_y()
                .color(BLACK);
        }
    }
}

struct Model {
    egui: Egui,
    field: Field,
    won: bool,
    lost: bool,
    settings_ready: bool,
    field_rows: u32,
    field_cols: u32,
    cell_width: f32,
    cell_height: f32,
    field_width: f32,
    field_height: f32,
    field_margin_x: f32,
    field_margin_y: f32,
    last_left_click: u128,
    last_right_click: u128,
}

fn main() {
    nannou::app(model).update(update).run();
}

/// Creates the window and sets up the [`Model`].
fn model(app: &App) -> Model {
    let window_id = app
        .new_window()
        .title("minesweeper_rs")
        .size(800, 800)
        .view(view)
        .raw_event(raw_window_event)
        .build()
        .unwrap();
    let window = app.window(window_id).unwrap();

    let mut field = Field::empty(DEFAULT_FIELD_ROWS, DEFAULT_FIELD_COLS);
    field.place_bombs();

    let egui = Egui::from_window(&window);

    Model {
        egui,
        field,
        won: false,
        lost: false,
        settings_ready: false,
        field_rows: DEFAULT_FIELD_ROWS,
        field_cols: DEFAULT_FIELD_COLS,
        cell_width: 0.0,
        cell_height: 0.0,
        field_width: 0.0,
        field_height: 0.0,
        field_margin_x: 0.0,
        field_margin_y: 0.0,
        last_left_click: 0,
        last_right_click: 0,
    }
}

fn update(app: &App, model: &mut Model, _update: Update) {
    for key in app.keys.down.iter() {
        if key == &VirtualKeyCode::R {
            model.lost = false;
            model.won = false;
            model.settings_ready = false
        }
    }
    if model.lost || model.won { return; }

    let window_rect = app.window_rect();

    if !model.settings_ready {
        let egui = &mut model.egui;
        let ctx = egui.begin_frame();
        egui::Window::new("Settings").show(&ctx, |ui| {
            ui.add(egui::Slider::new(&mut model.field_rows, 4..=42).text("Rows"));
            ui.add(egui::Slider::new(&mut model.field_cols, 4..=42).text("Columns"));

            if ui.button("Play").clicked() {
                model.field = Field::empty(model.field_rows, model.field_cols);
                model.field.place_bombs();
                model.settings_ready = true;
            }
        });
    } else {
        for button in app.mouse.buttons.pressed() {
            match button {
                (MouseButton::Left, position) => {
                    let time_now = SystemTime::now().duration_since(SystemTime::UNIX_EPOCH).expect("WHAT THE FUCK?").as_millis();
                    if time_now - model.last_left_click < 150 { break; }

                    model.last_left_click = time_now;
                    if let Some(position) = mouse_pos_to_field_pos(&position, model, &app.window_rect()) {
                        if model.field.get(position).has_flag { break; }

                        if model.field.get(position).is_revealed {
                            model.lost = model.field.reveal_neighbors(position);
                        } else {
                            model.lost = model.field.reveal(&position);
                        }
                    }
                    model.won = model.field.check_win();

                    if model.won || model.lost {
                        model.field.reveal_all();
                    }
                }
                (MouseButton::Right, position) => {
                    let time_now = SystemTime::now().duration_since(SystemTime::UNIX_EPOCH).expect("WHAT THE FUCK?").as_millis();
                    if time_now - model.last_right_click < 150 { break; }

                    model.last_right_click = time_now;
                    if let Some(position) = mouse_pos_to_field_pos(&position, model, &app.window_rect()) {
                        if model.field.get(position).is_revealed { break; }
                        model.field.toggle_flag(&position);
                    }
                }
                (_, _) => {}
            }
        }
    }

    // Calculate Cell and Field sizes and save them
    let cell_width = (window_rect.w() * 0.8) / model.field.0.len() as f32;
    let cell_height = (window_rect.h() * 0.8) / model.field.0.len() as f32;
    let field_width = cell_width * (model.field.0.len() as f32 - 1.0);
    let field_height = cell_height * (model.field.0.len() as f32 - 1.0);
    let remaining_window_width = window_rect.w() - field_width;
    let remaining_window_height = window_rect.h() - field_height;

    model.cell_width = cell_width;
    model.cell_height = cell_height;
    model.field_width = field_width;
    model.field_height = field_height;
    model.field_margin_x = remaining_window_width / 2.0;
    model.field_margin_y = remaining_window_height / 2.0;
}

fn raw_window_event(_app: &App, model: &mut Model, event: &nannou::winit::event::WindowEvent) {
    // Let egui handle things like keyboard and mouse input.
    model.egui.handle_raw_event(event);
}

/// Draws once a frame to the window.
fn view(app: &App, model: &Model, frame: Frame) {
    let mut draw = app.draw();
    // Change Origin Point to bottom left
    draw = draw.x_y(-app.window_rect().w() * 0.5, -app.window_rect().h() * 0.5);
    draw.background().color(CORNFLOWERBLUE);

    if model.settings_ready {
        model.field.draw(model, &draw);
    }

    draw.to_frame(app, &frame).unwrap();
    if !model.settings_ready {
        model.egui.draw_to_frame(&frame).unwrap();
    }
}

/// Converts the position of the mouse to the corresponding field position.
///
/// # Returns
///
/// [`None`] if the `mouse_pos` is outside of the [`Field`].
fn mouse_pos_to_field_pos(mouse_pos: &Point2, model: &Model, window_rect: &Rect) -> Option<Point2> {
    // Convert mouse_pos Origin Point to bottom left
    let mouse_pos = Point2::new(mouse_pos.x + window_rect.w() / 2.0 - model.field_margin_x + model.cell_width / 2.0, mouse_pos.y + window_rect.h() / 2.0 - model.field_margin_y + model.cell_height / 2.0);
    let cell_x = (mouse_pos.x / model.cell_width) as i32;
    let cell_y = (mouse_pos.y / model.cell_height) as i32;

    if cell_x >= model.field.cols() as i32 || cell_x < 0 || cell_y < 0 || cell_y >= model.field.rows() as i32 {
        None
    } else {
        Some(Point2::new(cell_x as f32, cell_y as f32))
    }
}
