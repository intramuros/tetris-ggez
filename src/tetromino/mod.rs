mod utils;

use self::utils::body_generators;
use ggez::{graphics, Context, GameResult};

use rand::Rng;

pub const GRID_SIZE: (i16, i16) = (10, 20);
pub const GRID_CELL_SIZE: (i16, i16) = (32, 32);

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

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Motion {
    Left,
    Right,
    RotateLeft,
    RotateRight,
}

// impl Motion {
//     pub fn from_keycode(key: KeyCode) -> Option<Motion> {
//         match key {
//             KeyCode::Left => Some(Motion::Left),
//             KeyCode::Right => Some(Motion::Right),
//             KeyCode::Up => Some(Motion::RotateRight),
//             KeyCode::Down => Some(Motion::RotateLeft),
//             KeyCode::D => Some(Motion::Down),
//             _ => None,
//         }
//     }
// }

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

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Segment {
    pub x: i16,
    pub y: i16,
}

impl Segment {
    pub fn new(pos: (i16, i16)) -> Self {
        Self { x: pos.0, y: pos.1 }
    }

    pub fn add_ghost_layer(&self) -> Self {
        Self {
            x: self.x,
            y: self.y - 1,
        }
    }
    fn right_neighbor(&self) -> Self {
        Self::new((self.x + 1, self.y))
    }
    pub fn left_neighbor(&self) -> Self {
        Self::new((self.x - 1, self.y))
    }
}

pub struct Tetromino {
    shape: Shape,
    pub body: Vec<Segment>,
    // velocity: f32, // the speed of fall is probably better mananged by the system
}

impl Tetromino {
    pub fn new() -> Self {
        let shape: Shape = Tetromino::generate_shape().unwrap_or(Shape::I);
        // let shape = Shape::I;
        let body = Tetromino::generate_body(&shape);
        println!("generating tetromino: shape - {:?}", shape);
        Self { shape, body }
    }

    pub fn move_to(&mut self, dir: Motion, base: &Vec<Segment>) {
        match dir {
            Motion::Left => {
                if self
                    .body
                    .iter()
                    .all(|elem| elem.x > 0 && elem.y > -1 && !base.contains(&elem.left_neighbor()))
                {
                    for seg in self.body.iter_mut() {
                        seg.x -= 1;
                    }
                }
            }
            Motion::Right => {
                if self.body.iter().all(|elem| {
                    elem.x < GRID_SIZE.0 - 1
                        && elem.y > -1
                        && !base.contains(&elem.right_neighbor())
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

    pub fn update(&mut self) {
        for seg in self.body.iter_mut() {
            seg.y += 1;
        }
    }

    pub fn draw(&self, ctx: &mut Context) -> GameResult<()> {
        for seg in self.body.iter() {
            let rectangle = graphics::Mesh::new_rectangle(
                ctx,
                graphics::DrawMode::fill(),
                seg.into(),
                [1.0, 0.5, 0.0, 1.0].into(),
            )?;
            graphics::draw(ctx, &rectangle, (ggez::mint::Point2 { x: 0.0, y: 0.0 },))?;
        }
        Ok(())
    }
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
        .map(Segment::new)
        .collect()
    }

    fn get_central_segment(&self) -> Option<Segment> {
        match self.shape {
            Shape::O => None,
            _ => Some(self.body[1]),
        }
    }

    fn kickback(&mut self) {
        if self.body.iter().any(|seg| seg.x < 0) {
            let kick = self.body.iter().min_by_key(|seg| seg.x).unwrap();
            self.body = self
                .body
                .iter()
                .map(|seg| Segment::new((seg.x - kick.x, seg.y)))
                .collect();
        } else if self.body.iter().any(|seg| seg.x >= GRID_SIZE.0) {
            let kick = self.body.iter().max_by_key(|seg| seg.x).unwrap();
            self.body = self
                .body
                .iter()
                .map(|seg| Segment::new((seg.x - (kick.x - GRID_SIZE.0 + 1), seg.y)))
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
                .map(|seg| Segment::new((-1 * (seg.y + y) - x, seg.x + x - y)))
                .collect();
            if !new_body
                .iter()
                .any(|seg| base.contains(seg) || seg.y >= GRID_SIZE.1)
            {
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
                .map(|seg| Segment::new(((seg.y + y) - x, -1 * (seg.x + x) - y)))
                .collect();
            if !new_body
                .iter()
                .any(|seg| base.contains(seg) || seg.y >= GRID_SIZE.1)
            {
                self.body = new_body;
                self.kickback();
            }
        }
    }
}
