pub mod gui;
mod utils;

use std::fs::File;
use std::path::{PathBuf, self};

use ggez::conf::Conf;
use ggez::{Context, ContextBuilder, GameResult};
use ggez::graphics::{self, Color};
use ggez::event::{self, EventHandler};

use gui::GUI;


static DEFAULT_BACKGROUND_COL: Color = Color::BLACK;
static SETTINGS_PATH: &str = "Settings.toml";
static STATIC_FILES_PATH: &str = "./static";


fn get_config() -> GameResult<Conf> {
    let mut config_file = File::open(SETTINGS_PATH)?;
    let conf = Conf::from_toml_file(&mut config_file)?;

    Ok(conf)
}

fn get_resources_dir() -> PathBuf {
    path::PathBuf::from(STATIC_FILES_PATH)
}

fn main() -> GameResult{

    // Make a Context.
    let (mut ctx, event_loop) = ContextBuilder::new("selenialSector", "Vlad")
        .default_conf(get_config()?)
        .add_resource_path(get_resources_dir())
        .build()
        .expect("Could not create game context");

    let main_state = MainState::new(&mut ctx);

    // Run!
    event::run(ctx, event_loop, main_state?);
}

struct MainState {
    gui: GUI,
    temp: String
}
impl MainState {
    fn new(ctx: &mut Context) -> GameResult<Self> {
        Ok(Self {
            gui: GUI::new(ctx)?,
            temp: String::new()
        })
    }
}

impl EventHandler for MainState {
    fn update(&mut self, ctx: &mut Context) -> GameResult<()> {
        self.gui.update(ctx, self.temp.to_owned())?;
        self.temp.clear();
        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult {
        graphics::clear(ctx, DEFAULT_BACKGROUND_COL);
        self.gui.display(ctx)?;
        graphics::present(ctx)
    }

    fn key_down_event(&mut self, ctx: &mut Context, keycode: event::KeyCode, _keymods: event::KeyMods, _repeat: bool) {
        let keychar = format!("{:?}", keycode);
        self.temp = keychar
    }
}