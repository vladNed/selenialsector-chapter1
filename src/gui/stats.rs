use ggez::graphics::{self, Font, Text};
use ggez::{Context, GameResult};

use crate::utils::Vector2D;

static STATS_SEPARATOR: &str = "  |  ";
static STATS_FONT_SIZE: f32 = 18.0;
static STATS_FONT_PATH: &str = "/fonts/RobotoMono-VariableFont_wght.ttf";

pub trait Stat {
    fn get_name(&self) -> String;
    fn get_value(&self) -> String;
    fn build(&self) -> String {
        format!("{}: {}", self.get_name(), self.get_value())
    }
}

/// Player chosen username
pub struct PlayerName {
    display_name: String,
    display_value: String,
}
impl PlayerName {
    pub fn new(value: String) -> Box<Self> {
        Box::new(Self {
            display_name: "Username".to_string(),
            display_value: value,
        })
    }
}
impl Stat for PlayerName {
    fn get_name(&self) -> String {
        self.display_name.to_owned()
    }

    fn get_value(&self) -> String {
        self.display_value.to_owned()
    }
}

/// Terminal Name used as the game terminal change
pub struct TerminalName {
    display_name: String,
    display_value: String,
}
impl TerminalName {
    pub fn new(value: String) -> Box<Self> {
        Box::new(Self {
            display_name: "Terminal".to_string(),
            display_value: value,
        })
    }
}
impl Stat for TerminalName {
    fn get_name(&self) -> String {
        self.display_name.to_owned()
    }

    fn get_value(&self) -> String {
        self.display_value.to_owned()
    }
}

/// Factory to build and display all stats
///
/// Stats should implement the `Stat` trait
pub struct GUIStats {
    font: Font,
    stats: Vec<Box<dyn Stat>>,
}

impl GUIStats {
    pub fn new(stats: Vec<Box<dyn Stat>>, ctx: &mut Context) -> GameResult<Self> {
        let font = Font::new(ctx, STATS_FONT_PATH)?;

        Ok(Self { font, stats })
    }

    pub fn display(&self, ctx: &mut Context) -> GameResult {
        let dest_point = Vector2D::new(20.0, 15.0);
        let text = Text::new((self.build_stats_text_body(), self.font, STATS_FONT_SIZE));

        graphics::draw(ctx, &text, (dest_point.as_vec(),))
    }

    fn build_stats_text_body(&self) -> String {
        let mut body: Vec<String> = Vec::new();
        for stat in self.stats.iter() {
            body.push(stat.build());
        }

        body.join(STATS_SEPARATOR)
    }
}
