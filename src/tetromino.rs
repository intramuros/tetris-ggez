mod utils;

use self::utils::body_generators;
use ggez::{graphics, Context, GameResult};

use rand::Rng;

pub(crate) const GRID_SIZE: (i16, i16) = (10, 20);
pub(crate) const GRID_CELL_SIZE: (i16, i16) = (26, 26);

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

/// Represents motion of a piece
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum Motion {
    Left,
    Right,
    RotateLeft,
}

/// Represents piece's shape, 7 classic tetromino shapes are used
#[derive(Debug, Copy, Clone)]
pub(crate) enum Shape {
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
pub(crate) struct Segment {
    pub(crate) x: i16,
    pub(crate) y: i16,
    pub(crate) color: ColorTuple,
}

/// Default shape is I
impl Default for Shape {
    fn default() -> Self {
        Shape::I
    }
}

impl Segment {
    /// Create new segment from position coordinates and add color
    pub(crate) fn new(pos: (i16, i16), color: ColorTuple) -> Self {
        Self {
            x: pos.0,
            y: pos.1,
            color,
        }
    }

    /// Add an extra segment as a ghost cell
    pub(crate) fn add_ghost_layer(&self) -> Self {
        Self {
            x: self.x,
            y: self.y - 1,
            color: (1, 1, 0, 255),
        }
    }
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
#[derive(Clone, Debug)]
pub(crate) struct Tetromino {
    shape: Shape,
    pub(crate) body: Vec<Segment>,
}

impl From<Shape> for Tetromino {
    /// Generate body at the starting position from a shape
    fn from(shape: Shape) -> Self {
        Self {
            body: Self::generate_body(&shape),
            shape,
        }
    }
}

impl Tetromino {
    /// Create new piece with a random shape
    pub(crate) fn new() -> Self {
        let shape: Shape = Tetromino::generate_shape().unwrap();
        Self::from(shape)
    }

    /// Translate piece
    pub(crate) fn translate(&mut self, x: i16, y: i16) {
        for seg in self.body.iter_mut() {
            seg.x += x;
            seg.y += y;
        }
    }

    /// Move by a single step or rotate the piece
    pub(crate) fn move_to(&mut self, dir: Motion, base: &Vec<Segment>) {
        match dir {
            Motion::Left => {
                if self.body.iter().all(|elem| {
                    elem.x > 0
                        && elem.y > -1
                        && !base
                            .iter()
                            .any(|b_elem| b_elem.x == (elem.x - 1) && b_elem.y == elem.y)
                }) {
                    self.body.iter_mut().for_each(|seg| seg.x -= 1);
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
                    self.body.iter_mut().for_each(|seg| seg.x += 1);
                }
            }
            Motion::RotateLeft => {
                self.rotate_left(base);
            }
        }
    }

    /// Implements the downward motion
    pub(crate) fn update(&mut self) {
        for seg in self.body.iter_mut() {
            seg.y += 1;
        }
    }

    /// Draw the figure using simple rectangles
    pub(crate) fn draw(&self, ctx: &mut Context) -> GameResult<()> {
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
    pub(crate) fn clone_body(&self) -> Vec<Segment> {
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
            6 => Some(Shape::J),
            _ => unreachable!(),
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

    /// Return the central segment around which a piece rotates
    fn get_central_segment(&self) -> Option<Segment> {
        match self.shape {
            Shape::O => None,
            _ => Some(self.body[1]),
        }
    }

    /// Implements kick from a wall if a piece segment left the well as a result
    /// of rotation
    fn kickback(&mut self) {
        if self.body.iter().any(|seg| seg.x < 0) {
            let kick = self.body.iter().min_by_key(|seg| seg.x).unwrap().x;
            self.body
                .iter_mut()
                .for_each(|seg| *seg = Segment::new((seg.x - kick, seg.y), seg.color));
        } else if self.body.iter().any(|seg| seg.x >= GRID_SIZE.0) {
            let kick = self.body.iter().max_by_key(|seg| seg.x).unwrap().x;
            self.body.iter_mut().for_each(|seg| {
                *seg = Segment::new((seg.x - (kick - GRID_SIZE.0 + 1), seg.y), seg.color)
            });
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
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]

    /// Test whether piece is translated appropriately
    fn test_translate() {
        let mut piece = Tetromino::new();
        let orig_piece = piece.clone();
        piece.translate(2, -1);
        for (seg_translated, seg_orig) in piece.body.iter().zip(orig_piece.body.iter()) {
            assert_eq!(seg_translated.x, seg_orig.x + 2);
            assert_eq!(seg_translated.y, seg_orig.y - 1);
        }
    }

    #[test]
    fn test_kickback() {
        let base = Vec::new();
        let shape = Shape::I;
        let mut piece = Tetromino::from(shape);
        let body: Vec<Segment> = vec![(0, -1), (0, 0), (0, 1), (0, 2)]
            .into_iter()
            .map(|v| Segment::new(v, (0, 0, 0, 0)))
            .collect();
        piece.body = body;
        piece.rotate_left(&base);
    }

}
