use ggez::graphics::{self, Color, DrawParam, Mesh, MeshBuilder};
use ggez::{Context, GameResult};

use crate::utils::Point2D;
 
pub type Margins = (Point2D, Point2D);

pub trait Vector {
    fn get_start(&self) -> &Point2D;
    fn get_end(&self) -> &Point2D;
}

/// Represents an edge of the gui screen
#[derive(Clone, Debug)]
pub struct Edge {
    margins: Margins,
    mesh: Mesh,
}

impl Edge {
    pub fn new(x: Point2D, y: Point2D, ctx: &mut Context) -> GameResult<Edge> {
        let line = MeshBuilder::new()
            .line(&[x.as_vec(), y.as_vec()], 2.0, Color::WHITE)?
            .build(ctx)?;
        Ok(Self {
            mesh: line,
            margins: (x, y)
        })
    }

    pub fn display(&self, ctx: &mut Context) -> GameResult {
        let draw_params = DrawParam::new();
        graphics::draw(ctx, &self.mesh, draw_params)
    }
}

impl Vector for Edge {
    fn get_start(&self) -> &Point2D {
        &self.margins.0
    }

    fn get_end(&self) -> &Point2D {
        &self.margins.1
    }
}
