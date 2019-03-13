use ggez::event::{KeyCode, KeyMods};

use ggez::{event, graphics, Context, GameResult};
use rand::Rng;
use std::collections::{HashSet, VecDeque};

use crate::tetromino::*;
use std::time::{Duration, Instant};

pub(crate) struct GameState {
    base: Vec<Segment>,
    ghost_layer: HashSet<Segment>,
    bag: VecDeque<Shape>,
    cur_fig: Tetromino,
    game_over: bool,
    fall_update: Instant,
    points: u64,
    updates_per_second: f32,
    updates_fast: f32,
    update_slow: f32,
    paused: bool,
}

/// Represents main part of the game where most of the logic is implemented
impl GameState {
    /// Create a new game with default settings
    pub(crate) fn new() -> Self {
        let mut rng = rand::thread_rng();
        // make a bag of pieces that are replenished on the fly
        let bag: VecDeque<Shape> = (0..10).map(|_| rng.gen_range(0, 7).into()).collect();
        Self {
            base: Vec::new(),
            ghost_layer: HashSet::new(),
            bag,
            cur_fig: Tetromino::new(),
            game_over: false,
            fall_update: Instant::now(),
            points: 0,
            updates_per_second: 2.0,
            updates_fast: 40.0,
            update_slow: 1.5,
            paused: false,
        }
    }

    fn cur_fig_landed(&self) -> bool {
        if self.cur_fig.body.iter().any(|elem| {
            elem.y == GRID_SIZE.1 - 1
                || self
                    .ghost_layer
                    .iter()
                    .any(|g_elem| g_elem.x == elem.x && g_elem.y == elem.y)
        }) {
            return true;
        }
        false
    }

    fn add_shape_to_bag(&mut self) {
        let mut rng = rand::thread_rng();
        self.bag.push_back(rng.gen_range(0, 7).into());
    }

    fn update_ghost_layer(&mut self) {
        for seg in self.cur_fig.body.iter() {
            self.ghost_layer.insert(seg.add_ghost_layer());
        }
    }

    fn hit_ceiling(&self) -> bool {
        self.cur_fig.body.iter().any(|seg| seg.y == 0)
            && self.ghost_layer.iter().any(|g_elem| {
                self.cur_fig
                    .body
                    .iter()
                    .any(|seg| g_elem.x == seg.x && g_elem.y == seg.y)
            })
    }

    /// Check if any rows are full and burn them. Add points based on how many
    /// rows were burnt, extra rows give bonus points.
    fn burn_full_rows(&mut self) {
        let mut burned = 0;
        for y_coord in 0..GRID_SIZE.1 {
            if self.base.iter().filter(|seg| seg.y == y_coord).count() == 10 {
                burned += 1;
                self.base.retain(|seg| seg.y != y_coord);
                self.base = self
                    .base
                    .iter()
                    .map(|seg| {
                        // dbg!(&seg);
                        if seg.y < y_coord {
                            Segment::new((seg.x, seg.y + 1), seg.color)
                        } else {
                            seg.clone()
                        }
                    })
                    .collect();
            }
        }

        if burned > 0 {
            let bonus = match burned {
                1 => 0,
                2 => 5,
                3 => 10,
                4 => 15,
                5 => 20,
                _ => 25,
            };
            self.points += burned * 10 + bonus;
            self.ghost_layer = (0..GRID_SIZE.0)
                .filter_map(|x| {
                    match self
                        .base
                        .iter()
                        .filter(|seg| seg.x == x)
                        .min_by_key(|elem| elem.y)
                    {
                        Some(c) => Some(Segment::new((c.x, c.y - 1), c.color)),
                        None => None,
                    }
                })
                .collect();
        }
    }

    fn accelerate(&mut self) {
        if self.cur_fig.body.iter().any(|seg| seg.y > 1) && !self.cur_fig_landed() {
            self.updates_per_second = self.updates_fast;
        }
    }
}

/// Implementation of the EventHandler for out GameState
///
/// Two mandatory methods (update() and draw()) are implemented along with
/// handling key presses to rotate and move pieces.
impl event::EventHandler for GameState {
    fn update(&mut self, _ctx: &mut Context) -> GameResult {
        self.update_slow = match self.points {
            0...100 => 1.2,
            100...200 => 1.6,
            200...300 => 2.0,
            300...400 => 3.0,
            400...500 => 4.0,
            500...600 => 5.0,
            _ => 6.0,
        };
        let millis_per_update: u64 = (1.0 / self.updates_per_second * 1000.0) as u64;
        if Instant::now() - self.fall_update >= Duration::from_millis(millis_per_update)
            && !self.game_over
            && !self.paused
        {
            if self.hit_ceiling() {
                println!("Hit ceiling");
                self.game_over = true;
            } else if self.cur_fig_landed() {
                self.update_ghost_layer();
                self.base.extend(self.cur_fig.clone_body());
                self.burn_full_rows();
                println!("{}", self.points);
                self.updates_per_second = self.update_slow;
                self.cur_fig = Tetromino::from(self.bag.pop_front().unwrap_or_default());
                self.add_shape_to_bag();
            } else {
                self.cur_fig.update();
                self.fall_update = Instant::now();
                self.updates_per_second = self.update_slow;
            }
            // self.move_update = Instant::now();
        }
        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult {
        graphics::clear(ctx, [0.0, 0.0, 0.0, 1.0].into());
        let main_field = graphics::Rect::new_i32(
            0,
            0,
            (GRID_CELL_SIZE.0 * GRID_SIZE.0) as i32,
            (GRID_CELL_SIZE.1 * GRID_SIZE.1) as i32,
        );
        let rectangle = graphics::Mesh::new_rectangle(
            ctx,
            graphics::DrawMode::fill(),
            main_field,
            (32, 32, 32, 255).into(),
        )?;
        graphics::draw(ctx, &rectangle, (ggez::mint::Point2 { x: 0.0, y: 0.0 },))?;
        // draw a grid
        for j in 0..GRID_SIZE.1 {
            if j < GRID_SIZE.0 {
                let points_vert = [
                    ggez::mint::Point2 {
                        x: (j * GRID_CELL_SIZE.0) as f32,
                        y: 0.0,
                    },
                    ggez::mint::Point2 {
                        x: (j * GRID_CELL_SIZE.0) as f32,
                        y: (GRID_CELL_SIZE.1 * GRID_SIZE.1) as f32,
                    },
                ];
                let line =
                    graphics::Mesh::new_line(ctx, &points_vert, 1.0, [0.3, 0.3, 0.3, 0.7].into())?;
                graphics::draw(ctx, &line, (ggez::mint::Point2 { x: 0.0, y: 0.0 },))?;
            }
            let points_hor = [
                ggez::mint::Point2 {
                    y: (j * GRID_CELL_SIZE.1) as f32,
                    x: 0.0,
                },
                ggez::mint::Point2 {
                    y: (j * GRID_CELL_SIZE.1) as f32,
                    x: (GRID_CELL_SIZE.0 * GRID_SIZE.0) as f32,
                },
            ];
            let line =
                graphics::Mesh::new_line(ctx, &points_hor, 1.0, [0.3, 0.3, 0.3, 0.7].into())?;
            graphics::draw(ctx, &line, (ggez::mint::Point2 { x: 0.0, y: 0.0 },))?;
        }

        self.cur_fig.draw(ctx)?;

        // draw the base
        for seg in self.base.iter() {
            let rectangle = graphics::Mesh::new_rectangle(
                ctx,
                graphics::DrawMode::fill(),
                seg.into(),
                seg.color.into(),
            )?;
            graphics::draw(ctx, &rectangle, (ggez::mint::Point2 { x: 0.0, y: 0.0 },))?;
        }

        // draw Score
        let title_position = ggez::mint::Point2 {
            x: (GRID_SIZE.0 * GRID_CELL_SIZE.0 + (7 * GRID_CELL_SIZE.0) / 3) as f32,
            y: GRID_SIZE.1 as f32 + 2.,
        };
        let point_position = ggez::mint::Point2 {
            x: title_position.x + 16.,
            y: title_position.y + 32.,
        };

        let points_text = graphics::Text::new("Score");
        let points = graphics::Text::new(self.points.to_string());
        graphics::draw(ctx, &points_text, (title_position,))?;
        graphics::draw(ctx, &points, (point_position,))?;

        // draw next figure
        let next_fig_text_pos = ggez::mint::Point2 {
            x: title_position.x + 10.0,
            y: point_position.y + 70.0,
        };
        let next_text = graphics::Text::new("Next");
        graphics::draw(ctx, &next_text, (next_fig_text_pos,))?;
        let next_shape = **self.bag.iter().peekable().peek().unwrap();
        let mut next_fig = Tetromino::from(next_shape);
        next_fig.translate(8, 8);
        for seg in next_fig.body.iter() {
            let rectangle = graphics::Mesh::new_rectangle(
                ctx,
                graphics::DrawMode::fill(),
                seg.into(),
                seg.color.into(),
            )?;
            graphics::draw(ctx, &rectangle, (ggez::mint::Point2 { x: 0.0, y: 0.0 },))?;
        }

        graphics::present(ctx)?;

        ggez::timer::yield_now();

        Ok(())
    }

    /// Listen to key events, if certain keys are pressed perform prescribed motions
    fn key_down_event(
        &mut self,
        _ctx: &mut Context,
        keycode: KeyCode,
        _keymod: KeyMods,
        _repeat: bool,
    ) {
        match keycode {
            KeyCode::Left => self.cur_fig.move_to(Motion::Left, &self.base),
            KeyCode::Right => self.cur_fig.move_to(Motion::Right, &self.base),
            KeyCode::Up => self.cur_fig.move_to(Motion::RotateLeft, &self.base),
            // KeyCode::Down => self.cur_fig.move_to(Motion::RotateRight, &self.base),
            KeyCode::Down => self.accelerate(), //self.updates_per_second = self.updates_fast,
            KeyCode::Space => self.paused = !self.paused,
            KeyCode::Escape => ggez::quit(_ctx),
            _ => (),
        };
    }
}
