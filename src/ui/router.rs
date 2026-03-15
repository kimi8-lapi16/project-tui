use crate::app::state::{AppState, Screen};
use ratatui::prelude::*;

pub fn render(frame: &mut Frame, state: &AppState) {
    match &state.current_screen {
        Screen::ProjectSelector => {
            super::project_selector::render(frame, state);
        }
        Screen::TicketList { .. } => {
            super::ticket_list::render(frame, state);
        }
        Screen::TicketDetail { .. } | Screen::TicketEdit { .. } => {
            super::ticket_detail::render(frame, state);
        }
    }
}
