mod utils;

use self::utils::body_generators;
use ggez::{graphics, Context, GameResult};

use rand::Rng;

pub const GRID_SIZE: (i16, i16) = (10, 20);
pub const GRID_CELL_SIZE: (i16, i16) = (26, 26);

type ColorTuple = (u8, u8, u8, u8);

/// Trait implementation for turning a Segment into graphics Rectangle object
impl From<&Segment> for graphics::Rect {
    fn from(seg: &Segment) -> Self {
        graphics::Rect::new_i32(
            seg.x as i32 * GRID_CELL_SIZE.0 as i32,
            seg.y as i32 * GRID_CELL_SIZE.1 as i32,
            GRID_CELL_SIZE.0 as i32,
            GRID_CELL_SIZE.1 as i32,
        )
    }
}

/// Represents a motion of a piece
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Motion {
    Left,
    Right,
    RotateLeft,
    RotateRight,
}

/// Represents piece's shape, 7 classical tetromino shapes are used
#[derive(Debug, Copy, Clone)]
pub enum Shape {
    L,
    O,
    S,
    Z,
    I,
    T,
    J,
}

/// A segment is one out of four blocks making each piece
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Segment {
    pub x: i16,
    pub y: i16,
    pub color: ColorTuple,
}

/// Default shape is I
impl Default for Shape {
    fn default() -> Self {
        Shape::I
    }
}

impl Segment {
    /// Create a new segment out of position coordinates and ascribe some color
    pub fn new(pos: (i16, i16), color: ColorTuple) -> Self {
        Self {
            x: pos.0,
            y: pos.1,
            color,
        }
    }

    /// Add extra segment of top to signify a ghost cell
    pub fn add_ghost_layer(&self) -> Self {
        Self {
            x: self.x,
            y: self.y - 1,
            color: (1, 1, 0, 255),
        }
    }

    // fn right_neighbor(&self) -> Self {
    //     Self::new((self.x + 1, self.y), self.color)
    // }

    // fn left_neighbor(&self) -> Self {
    //     Self::new((self.x - 1, self.y), self.color)
    // }
}

impl From<i32> for Shape {
    fn from(num: i32) -> Shape {
        match num {
            0 => Shape::L,
            1 => Shape::O,
            2 => Shape::S,
            3 => Shape::Z,
            4 => Shape::I,
            5 => Shape::T,
            _ => Shape::J,
        }
    }
}

impl From<&Shape> for ColorTuple {
    fn from(shape: &Shape) -> Self {
        match *shape {
            Shape::L => (255, 128, 0, 255),
            Shape::O => (255, 255, 0, 255),
            Shape::S => (205, 0, 0, 255),
            Shape::Z => (0, 205, 0, 255),
            Shape::I => (0, 255, 255, 255),
            Shape::T => (51, 0, 104, 255),
            Shape::J => (0, 0, 153, 255),
        }
    }
}

/// Represents a single piece that has a shape and body made out of segments
pub struct Tetromino {
    shape: Shape,
    pub body: Vec<Segment>,
}

impl From<Shape> for Tetromino {
    /// Generate body at the starting position from a shape
    fn from(shape: Shape) -> Self {
        Self {
            body: Tetromino::generate_body(&shape),
            shape,
        }
    }
}

impl Tetromino {
    /// Create a new piece with a random shape
    pub fn new() -> Self {
        let shape: Shape = Tetromino::generate_shape().unwrap_or(Shape::I);
        Self {
            body: Self::generate_body(&shape),
            shape,
        }
    }

    /// Translate a piece
    pub fn translate(&mut self, x: i16, y: i16) {
        for seg in self.body.iter_mut() {
            seg.x += x;
            seg.y += y;
        }
    }

    /// Move by a single step or rotate the piece
    pub fn move_to(&mut self, dir: Motion, base: &Vec<Segment>) {
        match dir {
            Motion::Left => {
                if self.body.iter().all(|elem| {
                    elem.x > 0
                        && elem.y > -1
                        && !base
                            .iter()
                            .any(|b_elem| b_elem.x == (elem.x - 1) && b_elem.y == elem.y)
                }) {
                    for seg in self.body.iter_mut() {
                        seg.x -= 1;
                    }
                }
            }
            Motion::Right => {
                if self.body.iter().all(|elem| {
                    elem.x < GRID_SIZE.0 - 1
                        && elem.y > -1
                        && !base
                            .iter()
                            .any(|b_elem| b_elem.x == (elem.x + 1) && b_elem.y == elem.y)
                }) {
                    for seg in self.body.iter_mut() {
                        seg.x += 1;
                    }
                }
            }
            // Motion::Down => {
            //     if self.body.iter().all(|elem| {
            //         elem.y > -1
            //             && elem.y < GRID_SIZE.1 - 2
            //             && !base.iter().any(|seg| (seg.y - 2) == elem.y)
            //     }) {
            //         for seg in self.body.iter_mut() {
            //             seg.y += 2;
            //         }
            //     }
            // }
            Motion::RotateLeft => {
                self.rotate_left(base);
            }
            Motion::RotateRight => {
                self.rotate_right(base);
            }
        }
    }

    /// Implements the downward motion
    pub fn update(&mut self) {
        for seg in self.body.iter_mut() {
            seg.y += 1;
        }
    }

    /// Draw the figure using simple rectangles
    pub fn draw(&self, ctx: &mut Context) -> GameResult<()> {
        for seg in self.body.iter() {
            let rectangle = graphics::Mesh::new_rectangle(
                ctx,
                graphics::DrawMode::fill(),
                seg.into(),
                seg.color.into(),
            )?;
            graphics::draw(ctx, &rectangle, (ggez::mint::Point2 { x: 0.0, y: 0.0 },))?;
        }
        Ok(())
    }

    /// Clone body of a piece
    pub fn clone_body(&self) -> Vec<Segment> {
        self.body.clone()
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
            Shape::L => body_generators::generate_l(),
            Shape::O => body_generators::generate_o(),
            Shape::S => body_generators::generate_s(),
            Shape::Z => body_generators::generate_z(),
            Shape::I => body_generators::generate_i(),
            Shape::T => body_generators::generate_t(),
            Shape::J => body_generators::generate_j(),
        }
        .into_iter()
        .map(|v| Segment::new(v, shape.into()))
        .collect()
    }

    // Get the central segment around which the piece rotates
    fn get_central_segment(&self) -> Option<Segment> {
        match self.shape {
            Shape::O => None,
            _ => Some(self.body[1]),
        }
    }

    // Implements kick from a wall if piece's segment is out of the well after rotation
    fn kickback(&mut self) {
        if self.body.iter().any(|seg| seg.x < 0) {
            let kick = self.body.iter().min_by_key(|seg| seg.x).unwrap();
            self.body = self
                .body
                .iter()
                .map(|seg| Segment::new((seg.x - kick.x, seg.y), seg.color))
                .collect();
        } else if self.body.iter().any(|seg| seg.x >= GRID_SIZE.0) {
            let kick = self.body.iter().max_by_key(|seg| seg.x).unwrap();
            self.body = self
                .body
                .iter()
                .map(|seg| Segment::new((seg.x - (kick.x - GRID_SIZE.0 + 1), seg.y), seg.color))
                .collect();
        }
    }

    fn rotate_left(&mut self, base: &Vec<Segment>) {
        if let Some(central_segment) = self.get_central_segment() {
            // get the translation
            let (x, y) = (-central_segment.x, -central_segment.y);
            // translate, rotate and translate the body back
            let new_body: Vec<Segment> = self
                .body
                .iter()
                .map(|seg| Segment::new((-1 * (seg.y + y) - x, seg.x + x - y), seg.color))
                .collect();
            if !new_body.iter().any(|seg| {
                seg.y >= GRID_SIZE.1
                    || base
                        .iter()
                        .any(|b_elem| b_elem.x == seg.x && b_elem.y == seg.y)
            }) {
                self.body = new_body;
                self.kickback();
            }
        }
    }

    fn rotate_right(&mut self, base: &Vec<Segment>) {
        if let Some(central_segment) = self.get_central_segment() {
            // get the translation
            let (x, y) = (-central_segment.x, -central_segment.y);
            // translate, rotate and translate the body back
            let new_body: Vec<Segment> = self
                .body
                .iter()
                .map(|seg| Segment::new(((seg.y + y) - x, -1 * (seg.x + x) - y), seg.color))
                .collect();
            if !new_body.iter().any(|seg| {
                seg.y >= GRID_SIZE.1
                    || base
                        .iter()
                        .any(|b_elem| b_elem.x == seg.x && b_elem.y == seg.y)
            }) {
                self.body = new_body;
                self.kickback();
            }
        }
    }
}
