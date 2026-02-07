use crate::render::TerminalRenderer;
use crossterm::style::Color;
use std::io;

#[derive(Default)]
pub struct House;

impl House {
    pub fn height(&self) -> u16 {
        self.get_ascii().len() as u16
    }

    pub fn width(&self) -> u16 {
        self.get_ascii().iter().map(|l| l.len()).max().unwrap_or(0) as u16
    }

    pub fn door_offset(&self) -> u16 {
        18 // Hardcoded based on ASCII art structure
    }

    pub fn get_ascii(&self) -> Vec<&'static str> {
        vec![
            "          (                  ",
            "                             ",
            "            )                ",
            "          ( _   _._          ",
            "           |_|-'_~_`-._      ",
            "        _.-'-_~_-~_-~-_`-._  ",
            "    _.-'_~-_~-_-~-_~_~-_~-_`-._",
            "   ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~",
            "     |  []  []   []   []  [] |",
            "     |           __    ___   |",
            "   ._|  []  []  | .|  [___]  |_._._._._._._._._._._._._._._._._.",
            "   |=|________()|__|()_______|=|=|=|=|=|=|=|=|=|=|=|=|=|=|=|=|=|",
            " ^^^^^^^^^^^^^^^ === ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^",
        ]
    }

    pub fn render(&self, renderer: &mut TerminalRenderer, x: u16, y: u16) -> io::Result<()> {
        let ascii = self.get_ascii();
        for (i, line) in ascii.iter().enumerate() {
            renderer.render_line_colored(x, y + i as u16, line, Color::Yellow)?;
        }
        Ok(())
    }
}
