// pub mod tetromino;
use ggez::event::{KeyCode, KeyMods};
use ggez::{event, graphics, Context, GameResult};
use std::collections::HashSet;

use crate::tetromino::*;
use std::time::{Duration, Instant};
const UPDATES_PER_SECOND: f32 = 8.0;

const MILLIS_PER_UPDATE: u64 = (1.0 / UPDATES_PER_SECOND * 1000.0) as u64;

pub struct GameState {
    base: HashSet<Segment>,
    ghost_layer: Vec<Segment>,
    cur_fig: Tetromino,
    game_over: bool,
    last_update: Instant,
}

impl GameState {
    pub fn new() -> Self {
        let ghost_layer = (0..=GRID_SIZE.0)
            .map(|x| Segment::new((x, GRID_SIZE.1 - 1).into()))
            .collect();
        println!("{:?}", ghost_layer);
        let cur_fig = Tetromino::new(1.0);
        Self {
            base: HashSet::new(),
            ghost_layer,
            cur_fig: cur_fig,
            game_over: false,
            last_update: Instant::now(),
        }
    }

    fn cur_fig_landed(&self) -> bool {
        if self
            .cur_fig
            .body
            .iter()
            .any(|elem| self.ghost_layer.contains(&elem))
        {
            return true;
        }
        false
    }

    fn update_ghost_layer(&mut self) {
        for seg in self.cur_fig.body.iter() {
            self.ghost_layer.push(seg.add_ghost_layer());
        }
    }

    fn hit_ceiling(&self) -> bool {
        self.cur_fig.body.iter().any(|seg| seg.pos.y == 0 as i16)
            && self
                .cur_fig
                .body
                .iter()
                .any(|seg| self.ghost_layer.contains(&seg))
    }
}

impl event::EventHandler for GameState {
    fn update(&mut self, _ctx: &mut Context) -> GameResult {
        if Instant::now() - self.last_update >= Duration::from_millis(MILLIS_PER_UPDATE) {
            if !self.game_over {
                self.cur_fig.update();
                if GameState::hit_ceiling(&self) {
                    println!("hit ceiling");
                    self.game_over = true;
                }
                if GameState::cur_fig_landed(&self) {
                    self.base.extend(self.cur_fig.copy_body());
                    GameState::update_ghost_layer(self);
                    self.cur_fig = Tetromino::new(1.0);
                }
            }
            self.last_update = Instant::now();
        }
        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult {
        graphics::clear(ctx, [0.0, 1.0, 0.0, 1.0].into());
        self.cur_fig.draw(ctx)?;

        for seg in self.base.iter() {
            let rectangle = graphics::Mesh::new_rectangle(
                ctx,
                graphics::DrawMode::fill(),
                seg.pos.into(),
                [1.0, 0.5, 0.0, 1.0].into(),
            )?;
            graphics::draw(ctx, &rectangle, (ggez::mint::Point2 { x: 0.0, y: 0.0 },))?;
        }

        // for seg in self.ghost_layer.iter() {
        //     let rectangle = graphics::Mesh::new_rectangle(
        //         ctx,
        //         graphics::DrawMode::fill(),
        //         seg.pos.into(),
        //         [1.0, 0.5, 0.2, 1.0].into(),
        //     )?;
        //     graphics::draw(ctx, &rectangle, (ggez::mint::Point2 { x: 0.0, y: 0.0 },))?;
        // }
        graphics::present(ctx)?;

        // ggez::timer::yield_now();

        Ok(())
    }

    fn key_down_event(
        &mut self,
        _ctx: &mut Context,
        keycode: KeyCode,
        _keymod: KeyMods,
        _repeat: bool,
    ) {
        if let Some(dir) = Direction::from_keycode(keycode) {
            self.cur_fig.move_to(&self.base, dir);
        }
    }
}
