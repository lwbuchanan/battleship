use ratatui:: {
    buffer::Buffer,
    layout::{Alignment, Rect},
    style::{Color, Stylize},
    widgets::{Block, Borders, Paragraph, Widget},
};

use crate::app::App;

impl Widget for &App {
    fn render(self, area: Rect, buf: &mut Buffer) {
        // let block = 
        Block::bordered()
            .title("battleship")
            .title_alignment(Alignment::Center)
            .borders(Borders::NONE)
            .render(area, buf);
    }
}