use ggez::conf::{FullscreenType, WindowMode, WindowSetup};
use ggez::event::{self, EventHandler};
use ggez::timer;
use ggez::{graphics, mint, Context, ContextBuilder, GameResult};

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
    // Your state here...
}

impl MyGame {
    pub fn new(_ctx: &mut Context) -> MyGame {
        // Load/create resources such as images here.
        MyGame {
            // ...
        }
    }
}

impl EventHandler for MyGame {
    fn update(&mut self, ctx: &mut Context) -> GameResult<()> {
        const FPS: u32 = 60;

        while timer::check_update_time(ctx, FPS) {}

        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult<()> {
        graphics::clear(ctx, graphics::BLACK);
        let circle = graphics::Mesh::new_circle(
            ctx,
            graphics::DrawMode::fill(),
            mint::Point2 { x: 200.0, y: 300.0 },
            100.0,
            0.000001,
            graphics::RED,
        )?;
        graphics::draw(ctx, &circle, graphics::DrawParam::default())?;
        // Draw code here...
        graphics::present(ctx)?;
        Ok(())
    }
}
