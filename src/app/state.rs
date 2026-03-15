use crate::github::models::{Project, Ticket, TicketDetail};

#[derive(Debug, Clone)]
pub enum Screen {
    ProjectSelector,
    TicketList { project_id: String },
    TicketDetail { project_id: String, item_id: String },
    TicketEdit { project_id: String, item_id: String },
}

#[derive(Debug)]
pub struct AppState {
    pub current_screen: Screen,
    pub projects: Vec<Project>,
    pub tickets: Vec<Ticket>,
    pub selected_ticket: Option<TicketDetail>,
    pub selected_index: usize,
    pub error: Option<String>,
    pub loading: bool,
    pub should_quit: bool,
}

impl Default for AppState {
    fn default() -> Self {
        Self {
            current_screen: Screen::ProjectSelector,
            projects: Vec::new(),
            tickets: Vec::new(),
            selected_ticket: None,
            selected_index: 0,
            error: None,
            loading: false,
            should_quit: false,
        }
    }
}

impl AppState {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn quit(&mut self) {
        self.should_quit = true;
    }

    pub fn set_loading(&mut self, loading: bool) {
        self.loading = loading;
    }

    pub fn set_error(&mut self, error: String) {
        self.error = Some(error);
        self.loading = false;
    }

    pub fn clear_error(&mut self) {
        self.error = None;
    }

    pub fn navigate_to(&mut self, screen: Screen) {
        self.current_screen = screen;
        self.selected_index = 0;
    }

    pub fn move_selection_up(&mut self) {
        if self.selected_index > 0 {
            self.selected_index -= 1;
        }
    }

    pub fn move_selection_down(&mut self, max: usize) {
        if self.selected_index < max.saturating_sub(1) {
            self.selected_index += 1;
        }
    }
}
