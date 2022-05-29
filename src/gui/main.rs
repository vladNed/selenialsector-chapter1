use ggez::{Context, GameResult, graphics};
use ndarray::{arr3, Array3, Axis, s};

use crate::utils::Vector2D;

use super::command::{TerminalInput, TerminalOp};
use super::edge::Edge;
use super::stats::{GUIStats, PlayerName, TerminalName};

static DEFAULT_MARGIN: f32 = 20.0;
static LOWER_MARGIN: f32 = 100.0;


pub struct GUI {
    edges: Vec<Edge>,
    stats: GUIStats,
    term_input: TerminalInput
}

impl GUI {
    pub fn new(ctx: &mut Context) -> GameResult<Self> {

        let username = PlayerName::new("Vlad".to_string());
        let term_name = TerminalName::new("0001-0001".to_string());
        let term_input = TerminalInput::new();

        Ok(Self {
            edges: Self::build_edges(ctx)?,
            stats: GUIStats::new(vec![username, term_name], ctx)?,
            term_input
        })
    }

    pub fn update(&mut self, _ctx: &mut Context, new_input: String) -> GameResult{
        let op: TerminalOp;
        if new_input.len() > 1 {
            op = match new_input.as_str() {
                "Delete" => super::command::TerminalOp::Clear,
                "Back" => super::command::TerminalOp::Backspace,
                "Return" => super::command::TerminalOp::Enter,
                "Space" => super::command::TerminalOp::Add { val: " ".to_string() },
                _ => super::command::TerminalOp::None
            };
        } else {
            op = TerminalOp::Add { val: new_input.to_owned() };
        }

        self.term_input.update(op);
        Ok(())
    }

    pub fn display(&self, ctx: &mut Context) -> GameResult {

        // Display edges
        for edge in self.edges.iter() {
            edge.display(ctx)?;
        }

        // Display stats
        self.stats.display(ctx)?;

        // Display input
        self.term_input.display(ctx)?;

        Ok(())
    }

    fn build_edges(ctx: &mut Context) -> GameResult<Vec<Edge>>{
        let mut edges = Vec::new();

        let edges_matrix = Self::compute_edges_matrix(ctx);
        for row in edges_matrix.axis_iter(Axis(0)){
            let heights = row.slice(s![0, ..]);
            let widths = row.slice(s![1, ..]);
            let edge = Self::build_edge(heights.to_vec(), widths.to_vec(), ctx)?;
            edges.push(edge);
        }

        Ok(edges)
    }

    fn build_edge(heights: Vec<f32>, widths: Vec<f32>, ctx: &mut Context) -> GameResult<Edge> {
        let start = Vector2D::new(widths[0], heights[0]);
        let end = Vector2D::new(widths[1], heights[1]);

        Ok(Edge::new(start, end, ctx)?)
    }

    fn compute_edges_matrix(ctx: &mut Context) -> Array3<f32>{
        let(max_width, max_height) = graphics::drawable_size(ctx);
        arr3(&[
            // Upper [heights, widths]
            [[DEFAULT_MARGIN*2.0, DEFAULT_MARGIN*2.0], [DEFAULT_MARGIN, max_width - DEFAULT_MARGIN]],
            // Lower [heights, widths]
            [[max_height - LOWER_MARGIN, max_height - LOWER_MARGIN], [DEFAULT_MARGIN, max_width - DEFAULT_MARGIN]],
            // Left [heights, widths]
            [[DEFAULT_MARGIN*2.0, max_height - LOWER_MARGIN], [DEFAULT_MARGIN, DEFAULT_MARGIN]],
            // Right [heights, widths]
            [[DEFAULT_MARGIN*2.0, max_height - LOWER_MARGIN], [max_width - DEFAULT_MARGIN, max_width - DEFAULT_MARGIN]]
        ])
    }
}

