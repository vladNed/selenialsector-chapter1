use std::fs::File;

use ggez::conf::Conf;
use ggez::{Context, ContextBuilder, GameResult};
use ggez::graphics::{self, Color};
use ggez::event::{self, EventHandler};

pub mod gui;

static DEFAULT_BACKGROUND_COL: Color = Color::BLACK;
static SETTINGS_PATH: &str = "Settings.toml";


fn get_config() -> GameResult<Conf> {
    let mut config_file = File::open(SETTINGS_PATH)?;
    let conf = Conf::from_toml_file(&mut config_file)?;

    Ok(conf)
}

fn main() -> GameResult{

    // Make a Context.
    let (ctx, event_loop) = ContextBuilder::new("selenialSector", "Vlad")
        .default_conf(get_config()?)
        .build()
        .expect("Could not create game context");

    let main_state = MainState::new();

    // Run!
    event::run(ctx, event_loop, main_state);
}

struct MainState;
impl MainState {
    fn new() -> Self { Self {}}
}

impl EventHandler for MainState {
    fn update(&mut self, _ctx: &mut Context) -> GameResult<()> {
        // Update code here...
        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult<()> {
        graphics::clear(ctx, DEFAULT_BACKGROUND_COL);
        // Draw code here...
        graphics::present(ctx)
    }
}