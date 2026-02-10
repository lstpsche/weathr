use crate::render::TerminalRenderer;
use crossterm::style::Color;
use std::io;

#[derive(Clone)]
struct Airplane {
    x: f32,
    y: f32,
    speed: f32,
    trail_positions: Vec<f32>,
}

pub struct AirplaneSystem {
    planes: Vec<Airplane>,
    terminal_width: u16,
    terminal_height: u16,
    spawn_cooldown: u16,
}

impl AirplaneSystem {
    pub fn new(terminal_width: u16, terminal_height: u16) -> Self {
        Self {
            planes: Vec::new(),
            terminal_width,
            terminal_height,
            spawn_cooldown: 0,
        }
    }

    pub fn update(&mut self, terminal_width: u16, terminal_height: u16) {
        self.terminal_width = terminal_width;
        self.terminal_height = terminal_height;

        for plane in &mut self.planes {
            plane.x += plane.speed;

            plane.trail_positions.insert(0, plane.x);
            if plane.trail_positions.len() > 8 {
                plane.trail_positions.truncate(8);
            }
        }

        self.planes.retain(|p| p.x < terminal_width as f32);

        self.spawn_cooldown = self.spawn_cooldown.saturating_sub(1);
        if self.spawn_cooldown == 0 && rand::random::<f32>() < 0.003 {
            self.spawn_plane();
            self.spawn_cooldown = 400 + (rand::random::<u16>() % 200);
        }
    }

    fn spawn_plane(&mut self) {
        let y = (rand::random::<u16>() % (self.terminal_height / 4)) as f32;
        let speed = 0.3 + (rand::random::<f32>() * 0.2);

        self.planes.push(Airplane {
            x: 0.0,
            y,
            speed,
            trail_positions: Vec::new(),
        });
    }

    pub fn render(&self, renderer: &mut TerminalRenderer) -> io::Result<()> {
        let airplane_art = [
            "           _",
            "         -=\\`\\",
            "     |\\ ____\\_\\__",
            "   -=\\c`\"\"\"\"\"\"\" \"`)",
            "      `~~~~~/ /~~`",
            "        -==/ /",
            "          '-'",
        ];

        for plane in &self.planes {
            let x = plane.x as u16;
            let y = plane.y as u16;

            for (i, &trail_x) in plane.trail_positions.iter().enumerate() {
                let trail_render_x = trail_x as u16;
                let trail_y = y + 3;

                if trail_render_x < self.terminal_width && trail_y < self.terminal_height {
                    let (ch, color) = match i {
                        0..=1 => ('.', Color::White),
                        2..=3 => ('.', Color::Grey),
                        4..=5 => ('.', Color::DarkGrey),
                        _ => ('·', Color::DarkGrey),
                    };
                    renderer.render_char(trail_render_x, trail_y, ch, color)?;
                }
            }

            for (line_offset, line) in airplane_art.iter().enumerate() {
                let render_y = y + line_offset as u16;
                if render_y >= self.terminal_height {
                    break;
                }

                for (char_offset, ch) in line.chars().enumerate() {
                    let render_x = x + char_offset as u16;
                    if render_x >= self.terminal_width {
                        break;
                    }

                    if ch != ' ' {
                        let color = match ch {
                            '"' => Color::Cyan,

                            '\\' => Color::Blue,

                            '_' => Color::DarkGrey,

                            '~' => Color::Grey,

                            _ => Color::White,
                        };
                        renderer.render_char(render_x, render_y, ch, color)?;
                    }
                }
            }
        }
        Ok(())
    }
}
