mod utils;

use self::utils::body_generators;
use ggez::event::KeyCode;
use ggez::{graphics, Context, GameResult};
use std::collections::HashSet;

use rand::Rng;

pub const GRID_SIZE: (i16, i16) = (10, 20);
const GRID_CELL_SIZE: (i16, i16) = (32, 32);

pub const SCREEN_SIZE: (f32, f32) = (
    GRID_SIZE.0 as f32 * GRID_CELL_SIZE.0 as f32,
    GRID_SIZE.1 as f32 * GRID_CELL_SIZE.1 as f32,
);

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct GridPosition {
    pub x: i16,
    pub y: i16,
}

impl GridPosition {
    pub fn new(x: i16, y: i16) -> Self {
        Self { x, y }
    }
}

impl From<GridPosition> for graphics::Rect {
    fn from(pos: GridPosition) -> Self {
        graphics::Rect::new_i32(
            pos.x as i32 * GRID_CELL_SIZE.0 as i32,
            pos.y as i32 * GRID_CELL_SIZE.1 as i32,
            GRID_CELL_SIZE.0 as i32,
            GRID_CELL_SIZE.1 as i32,
        )
    }
}

impl From<(i16, i16)> for GridPosition {
    fn from(pos: (i16, i16)) -> Self {
        GridPosition { x: pos.0, y: pos.1 }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Direction {
    Down,
    Left,
    Right,
}

impl Direction {
    pub fn from_keycode(key: KeyCode) -> Option<Direction> {
        match key {
            KeyCode::Down => Some(Direction::Down),
            KeyCode::Left => Some(Direction::Left),
            KeyCode::Right => Some(Direction::Right),
            _ => None,
        }
    }
}

#[derive(Debug, Copy, Clone)]
enum Shape {
    L,
    O,
    S,
    Z,
    I,
    T,
    J,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Segment {
    pub pos: GridPosition,
}

impl Segment {
    pub fn new(pos: GridPosition) -> Self {
        Self { pos }
    }

    pub fn add_ghost_layer(self) -> Self {
        Self {
            pos: GridPosition {
                x: self.pos.x,
                y: self.pos.y - 1,
            },
        }
    }
}

enum RotationDirection {
    Right,
    Left,
}

trait Rotation {
    fn rotate_figure(&mut self, key: KeyCode) -> Option<RotationDirection>;
}

pub struct Tetromino {
    shape: Shape,
    pub body: Vec<Segment>,
    velocity: f32,
    landed: bool,
}

impl Tetromino {
    pub fn new(velocity: f32) -> Self {
        let shape: Shape = Tetromino::generate_shape().unwrap_or(Shape::I);
        let body = Tetromino::generate_body(&shape);
        println!("generating tetromino: shape - {:?}", shape);
        Self {
            shape,
            body,
            velocity,
            landed: false,
        }
    }

    fn generate_shape() -> Option<Shape> {
        let mut rng = rand::thread_rng();
        match rng.gen_range(0, 7) {
            0 => Some(Shape::L),
            1 => Some(Shape::O),
            2 => Some(Shape::S),
            3 => Some(Shape::Z),
            4 => Some(Shape::I),
            5 => Some(Shape::T),
            _ => Some(Shape::J),
        }
    }
    fn generate_body(shape: &Shape) -> Vec<Segment> {
        match *shape {
            Shape::L => body_generators::generate_L(),
            Shape::O => body_generators::generate_O(),
            Shape::S => body_generators::generate_S(),
            Shape::Z => body_generators::generate_Z(),
            Shape::I => body_generators::generate_I(),
            Shape::T => body_generators::generate_T(),
            Shape::J => body_generators::generate_J(),
        }
        .into_iter()
        .map(|(i, j)| Segment::new(GridPosition::new(i, j)))
        .collect()
    }

    pub fn move_to(&mut self, base: &HashSet<Segment>, dir: Direction) {
        match dir {
            Direction::Left => {
                if !self
                    .body
                    .iter()
                    .any(|elem| elem.pos.x <= 0 || elem.pos.y <= 0 || base.contains(&elem))
                {
                    for seg in self.body.iter_mut() {
                        seg.pos.x -= 1;
                    }
                }
            }
            Direction::Right => {
                if !self
                    .body
                    .iter()
                    .any(|elem| elem.pos.x >= GRID_SIZE.0 - 1 || elem.pos.y <= 0)
                {
                    for seg in self.body.iter_mut() {
                        seg.pos.x += 1;
                    }
                }
            }

            Direction::Down => {
                for seg in self.body.iter_mut() {
                    seg.pos.y += 1;
                }
            }
        }
    }

    pub fn update(&mut self) {
        for seg in self.body.iter_mut() {
            seg.pos.y += 1;
        }
    }

    pub fn draw(&self, ctx: &mut Context) -> GameResult<()> {
        for seg in self.body.iter() {
            let rectangle = graphics::Mesh::new_rectangle(
                ctx,
                graphics::DrawMode::fill(),
                seg.pos.into(),
                [1.0, 0.5, 0.0, 1.0].into(),
            )?;
            graphics::draw(ctx, &rectangle, (ggez::mint::Point2 { x: 0.0, y: 0.0 },))?;
        }
        Ok(())
    }
    pub fn copy_body(&self) -> Vec<Segment> {
        self.body.clone()
    }
}

impl Rotation for Tetromino {
    fn rotate_figure(&mut self, key: KeyCode) -> Option<RotationDirection> {
        match key {
            KeyCode::Left => Some(RotationDirection::Left),
            KeyCode::Right => Some(RotationDirection::Right),
            _ => None,
        }
    }
}