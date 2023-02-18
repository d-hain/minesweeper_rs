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
    pub fn empty(rows: u8, cols: u8) -> Self {
        let field = vec![vec![Cell::new(false); cols as usize];rows as usize];

        Self(field)
    }
}

fn main() {
    dbg!(Field::empty(10, 10));
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
