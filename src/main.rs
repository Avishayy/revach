mod angle;
mod player;

use getrandom::getrandom;
use ggez::conf::{FullscreenType, WindowMode, WindowSetup};
use ggez::event::{self, EventHandler};
use ggez::timer;
use ggez::{graphics, Context, ContextBuilder, GameResult};
use glam::*;
use oorandom::Rand32;

fn main() {
    // Make a Context.
    let (mut ctx, event_loop) = ContextBuilder::new("Revach", "Shdodi")
        .window_mode(
            WindowMode::default()
                .dimensions(1920.0, 1080.0)
                .fullscreen_type(FullscreenType::True),
        )
        .window_setup(WindowSetup::default().vsync(true))
        .build()
        .expect("aieee, could not create ggez context!");

    // Create an instance of your event handler.
    // Usually, you should provide it with the Context object to
    // use when setting your game up.
    let my_game = MyGame::new(&mut ctx);

    // Run!
    event::run(ctx, event_loop, my_game);
}

struct MyGame {
    players: Vec<player::Player>,
    // TODO: hold rand here
}

impl MyGame {
    fn rand() -> Rand32 {
        let mut seed: [u8; 8] = [0; 8];
        getrandom(&mut seed).unwrap();
        Rand32::new(u64::from_ne_bytes(seed))
    }

    pub fn new(_ctx: &mut Context) -> MyGame {
        // Currently one player at a fixed position
        let mut players = Vec::<player::Player>::new();
        let mut rand = MyGame::rand();
        players.push(player::Player::new(
            Vec2::new(1920.0 / 2.0, 1080.0 / 2.0),
            angle::random_angle(&mut rand),
            graphics::RED,
        ));
        MyGame { players }
    }
}

impl EventHandler for MyGame {
    fn update(&mut self, ctx: &mut Context) -> GameResult<()> {
        const FPS: u32 = 60;

        while timer::check_update_time(ctx, FPS) {
            for player in &mut self.players {
                player.pos += player.angle * player.speed;
            }
        }

        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult<()> {
        graphics::clear(ctx, graphics::BLACK);

        for player in &self.players {
            let circle = graphics::Mesh::new_circle(
                ctx,
                graphics::DrawMode::fill(),
                player.pos,
                20.0,
                0.000001,
                player.color,
            )?;
            graphics::draw(ctx, &circle, graphics::DrawParam::default())?;
        }

        graphics::present(ctx)?;
        Ok(())
    }
}
