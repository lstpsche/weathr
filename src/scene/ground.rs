use crate::render::TerminalRenderer;
use crossterm::style::Color;
use std::io;

#[derive(Default)]
pub struct Ground;

impl Ground {
    pub fn render(
        &self,
        renderer: &mut TerminalRenderer,
        width: u16,
        height: u16,
        y_start: u16,
        path_center: u16,
    ) -> io::Result<()> {
        let width = width as usize;
        let height = height as usize;
        let path_center = path_center as usize;

        let grass_colors = [Color::Green, Color::DarkGreen];
        let flower_colors = [Color::Magenta, Color::Red, Color::Cyan, Color::Yellow];
        let soil_color = Color::Rgb {
            r: 101,
            g: 67,
            b: 33,
        }; // Brownish
        let path_color = Color::Rgb {
            r: 180,
            g: 160,
            b: 120,
        }; // Light sandy path

        // Simple seeded RNG for deterministic decoration positions
        fn pseudo_rand(x: usize, y: usize) -> u32 {
            ((x as u32 ^ 0x5DEECE6).wrapping_mul(y as u32 ^ 0xB)) % 100
        }

        for y in 0..height {
            let path_width = 4 + y; // Widens as it comes closer
            let path_start = path_center.saturating_sub(path_width / 2);
            let path_end = path_center + path_width / 2;

            for x in 0..width {
                let is_path = x >= path_start && x <= path_end;

                let (ch, color) = if y == 0 {
                    // Top layer: Grass/Flowers and Path
                    if is_path {
                        ('=', path_color)
                    } else {
                        // Grass with random flowers
                        let r = pseudo_rand(x, y);
                        if r < 5 {
                            // 5% chance of flower
                            let f_idx = (x + y) % flower_colors.len();
                            ('*', flower_colors[f_idx])
                        } else if r < 15 {
                            // 10% chance of distinct grass blade
                            (',', grass_colors[1])
                        } else {
                            ('^', grass_colors[0])
                        }
                    }
                } else {
                    // Lower layers: Soil/Rocks and Path
                    if is_path {
                        ('=', path_color)
                    } else {
                        // Soil pattern
                        let r = pseudo_rand(x, y);
                        let ch = if r < 20 {
                            '~'
                        } else if r < 25 {
                            '.'
                        } else {
                            ' '
                        };
                        (ch, soil_color)
                    }
                };

                renderer.render_char(x as u16, y_start + y as u16, ch, color)?;
            }
        }
        Ok(())
    }
}
