use crossterm::{
    event::{self, KeyCode, KeyEvent, KeyEventKind},
    execute, terminal,
};
use ratatui::prelude::{CrosstermBackend, Terminal};
use std::{
    io::{stderr, Stderr},
    time::{self, Instant},
};

use crate::app::App;
use crate::ui::draw;

type Err = Box<dyn std::error::Error>;
type Result<T> = std::result::Result<T, Err>;

#[derive(Debug)]
pub struct Term {
    should_quit: bool,
    terminal: Terminal<CrosstermBackend<Stderr>>,
    app: App,
    refresh_rate: time::Duration,
}

impl Term {
    pub fn new() -> Result<Term> {
        terminal::enable_raw_mode()?;
        execute!(stderr(), terminal::EnterAlternateScreen)?;
        Ok(Term {
            should_quit: false,
            terminal: Terminal::new(CrosstermBackend::new(stderr()))?,
            app: App::new(),
            refresh_rate: time::Duration::from_millis(100),
        })
    }

    pub fn run(&mut self) -> Result<()> {
        self.terminal.draw(|frame| {
            draw(frame, &self.app);
        })?;
        let mut next_update = Instant::now();
        while !self.should_quit {
            if event::poll(time::Duration::from_millis(500))? {
                if let event::Event::Key(key) = event::read()? {
                    self.on_key(key);
                }
            }
            if Instant::now() >= next_update {
                let size = self.terminal.size()?;
                let (x, y) = (size.width, size.height);
                self.app.update_bounds(x, y);
                self.app.update_center();
                self.terminal.draw(|frame| {
                    draw(frame, &self.app);
                })?;
                next_update = Instant::now() + self.refresh_rate;
            }
        }
        self.shutdown()?;
        Ok(())
    }

    fn shutdown(&mut self) -> Result<()> {
        execute!(stderr(), terminal::LeaveAlternateScreen)?;
        terminal::disable_raw_mode()?;
        Ok(())
    }

    fn on_key(&mut self, key: KeyEvent) {
        if key.kind == KeyEventKind::Press {
            match key.code {
                KeyCode::Esc => self.should_quit = true,
                KeyCode::Char('q') => self.should_quit = true,
                _ => {}
            }
        }
    }
}
