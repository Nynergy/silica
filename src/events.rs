use crossterm::{
    event::{
        self,
        Event,
        KeyCode,
    },
};
use std::error::Error;

use crate::app::*;

type DynResult<T> = Result<T, Box<dyn Error>>;

pub fn handle_events(app: &mut App) -> DynResult<()> {
    if let Event::Key(key) = event::read()? {
        match key.code {
            KeyCode::Char('q') => app.quit = true,
            KeyCode::Esc => app.quit = true,
            _ => {}
        }
    }

    Ok(())
}
