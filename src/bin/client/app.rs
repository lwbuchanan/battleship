use ratatui:: {
    DefaultTerminal,
    crossterm::event::{KeyCode, KeyEvent, KeyModifiers},
};

use crate::event::{AppEvent, Event, EventHandler};

#[derive(Debug)]
pub struct App {
    pub events: EventHandler,
    pub state: AppState,
    pub counter: u8,
    pub running: bool,
    pub connected: bool,
}

#[derive(Debug, Eq, PartialEq)]
pub enum AppState {
    InGame,
}

impl App {
    pub fn new() -> App {
        App {
            events: EventHandler::new(),
            state: AppState::InGame,
            counter: 0,
            running: true,
            connected: true,
        }
    }

    // Takes control of the app and runs it's main loop
    pub fn run(mut self, terminal: &mut DefaultTerminal) -> color_eyre::Result<()> {
        while self.running {
            terminal.draw(|frame| frame.render_widget(&self, frame.area()))?;
            self.handle_events()?;
        }
        Ok(())
    }

    fn handle_events(&mut self) -> color_eyre::Result<()> {
        match self.events.next()? {
            Event::Tick => self.tick(),
            Event::Crossterm(event) => match event {
                ratatui::crossterm::event::Event::Key(key_event) => self.handle_key_events(key_event)?,
                // Handle other terminal io events here
                _ => {},
            },
            Event::App(app_event) => match app_event {
                AppEvent::Quit => self.quit(),
            },
        }
        Ok(())
    }

    fn handle_key_events(&mut self, key_event: KeyEvent) -> color_eyre::Result<()> {
        match key_event.code {
            KeyCode::Char('q' | 'Q') | KeyCode::Esc => self.events.send(AppEvent::Quit),
            KeyCode::Char('c' | 'C') if key_event.modifiers == KeyModifiers::CONTROL => {
                self.events.send(AppEvent::Quit);
            },
            _ => {},
        };
        Ok(())
    }

    fn tick(&self) {}

    fn quit(&mut self) {
        self.running = false;
    }
}