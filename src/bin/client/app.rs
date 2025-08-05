use std::io;

use crossterm::event::{self, Event, KeyCode, KeyEvent, KeyEventKind};
use ratatui:: {
    buffer::Buffer,
    layout::Rect,
    style::Stylize,
    symbols::border,
    text::{Line, Text},
    widgets::{Block, Paragraph, Widget},
    DefaultTerminal, Frame,
};

pub struct App {
    state: AppState,
    counter: u8,
}

#[derive(Eq, PartialEq)]
enum AppState {
    Running,
    Done,
}

impl App {
    pub fn new() -> App {
        App {
            state: AppState:: Running,
            counter: 0,
        }
    }

    pub fn run(&mut self, terminal: &mut DefaultTerminal) -> io::Result<()> {
        while !(self.state == AppState::Done) {
            terminal.draw(|frame| self.draw(frame))?;
            self.handle_events()?;
        }
        Ok(())
    }

    fn draw(&self, frame: &mut Frame) {
        frame.render_widget(self, frame.area());
    }

    fn handle_events(&mut self) -> io::Result<()> {
        match event::read()? {
            Event::Key(key_event) if key_event.kind == KeyEventKind::Press => {
                match key_event.code {
                    KeyCode::Left => self.counter -= 1,
                    KeyCode::Right => self.counter += 1,
                    KeyCode::Char('q') => self.state = AppState::Done,
                    _ => {},
                }
            }
            _ => {}
        }
        Ok(())
    }

}

impl Widget for &App {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let title = Line::from(" Counter App Tutorial ".bold());
        let instructions = Line::from(vec![
            " Dec ".into(),
            "<Left>".blue().bold(),
            " Inc ".into(),
            "<Right>".blue().bold(),
            " Quit ".into(),
            "<Q>".blue().bold(),
        ]);
        let block = Block::bordered()
            .title(title.centered())
            .title_bottom(instructions.centered())
            .border_set(border::THICK);

        let counter_text = Text::from(vec![Line::from(vec![
            "Value: ".into(),
            self.counter.to_string().yellow(),
        ])]);

        Paragraph::new(counter_text)
            .centered()
            .block(block)
            .render(area, buf);
    }
}

// #[cfg(test)]
// mod tests {
//     use super::*;
//     use ratatui::style::Style;

//     #[test]
//     fn render() {
//         let app = App::new();
//         let mut buf = Buffer::empty(Rect::new(0, 0, 50, 4));

//         app.render(buf.area, &mut buf);

//         let mut expected = Buffer::with_lines(vec![
//             "┏━━━━━━━━━━━━━ Counter App Tutorial ━━━━━━━━━━━━━┓",
//             "┃                    Value: 0                    ┃",
//             "┃                                                ┃",
//             "┗━━━━━━━━ Dec <Left> Inc <Right> Quit <Q>━━━━━━━━┛", 
//         ]);
//         expected.set_style(Rect::new(14, 0, 22, 1), Style::new().bold());
//         expected.set_style(Rect::new(28, 1, 1, 1), Style::new().yellow());
//         expected.set_style(Rect::new(13, 3, 6, 1), Style::new().blue().bold());
//         expected.set_style(Rect::new(30, 3, 7, 1), Style::new().blue().bold());
//         expected.set_style(Rect::new(43, 3, 4, 1), Style::new().blue().bold());

//         assert_eq!(buf, expected);
//     }
// }