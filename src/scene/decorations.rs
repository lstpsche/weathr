use crate::render::TerminalRenderer;
use crossterm::style::Color;
use std::io;

#[derive(Default)]
pub struct Decorations;

impl Decorations {
    pub fn new() -> Self {
        Self
    }

    pub fn render(
        &self,
        renderer: &mut TerminalRenderer,
        horizon_y: u16,
        house_x: u16,
        house_width: u16,
        path_center: u16,
        width: u16,
    ) -> io::Result<()> {
        // Render Tree (Left of house)
        let (tree_lines, tree_color) = self.get_tree();
        let tree_height = tree_lines.len() as u16;
        let tree_y = horizon_y.saturating_sub(tree_height);
        let tree_x = house_x.saturating_sub(20);

        if tree_x > 0 {
            for (i, line) in tree_lines.iter().enumerate() {
                renderer.render_line_colored(tree_x, tree_y + i as u16, line, tree_color)?;
            }
        }

        // Render Fence (Right of house)
        let (fence_lines, fence_color) = self.get_fence();
        let fence_height = fence_lines.len() as u16;
        let fence_y = horizon_y.saturating_sub(fence_height); // Sitting on ground
        let fence_x = house_x + house_width + 2; // Slight gap

        if fence_x < width {
            for (i, line) in fence_lines.iter().enumerate() {
                renderer.render_line_colored(fence_x, fence_y + i as u16, line, fence_color)?;
            }
        }

        // Render Mailbox (Near path, slightly down)
        let (mailbox_lines, mailbox_color) = self.get_mailbox();
        let mailbox_x = path_center + 6; // Right of path
        let mailbox_y = horizon_y + 1; // Slightly on the ground/grass

        if mailbox_x < width {
            for (i, line) in mailbox_lines.iter().enumerate() {
                renderer.render_line_colored(
                    mailbox_x,
                    mailbox_y + i as u16,
                    line,
                    mailbox_color,
                )?;
            }
        }

        // Render Bush (Left of path, near house)
        let (bush_lines, bush_color) = self.get_bush();
        let bush_height = bush_lines.len() as u16;
        let bush_x = path_center.saturating_sub(10);
        let bush_y = horizon_y.saturating_sub(bush_height / 2); // Sitting partially on ground line

        if bush_x > 0 {
            for (i, line) in bush_lines.iter().enumerate() {
                renderer.render_line_colored(bush_x, bush_y + i as u16, line, bush_color)?;
            }
        }
        Ok(())
    }

    fn get_tree(&self) -> (Vec<&'static str>, Color) {
        (
            vec![
                "      ####      ",
                "    ########    ",
                "   ##########   ",
                "    ########    ",
                "      _||_      ",
            ],
            Color::DarkGreen,
        )
    }

    fn get_bush(&self) -> (Vec<&'static str>, Color) {
        (vec!["  ,.,  ", " (,,,,)", "  \"||\" "], Color::Green)
    }

    fn get_fence(&self) -> (Vec<&'static str>, Color) {
        (vec!["|--|--|--|--|", "|  |  |  |  |"], Color::White)
    }

    fn get_mailbox(&self) -> (Vec<&'static str>, Color) {
        (vec![" ___ ", "|___|", "  |  "], Color::Blue)
    }
}
