use crate::app::state::{AppState, Screen};
use ratatui::{
    layout::{Alignment, Constraint, Direction, Layout},
    prelude::*,
    widgets::{Block, Borders, Paragraph, Wrap},
};

pub fn render(frame: &mut Frame, state: &AppState) {
    let is_edit_mode = matches!(state.current_screen, Screen::TicketEdit { .. });

    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(3),
            Constraint::Min(0),
            Constraint::Length(3),
        ])
        .split(frame.area());

    // Title
    let title_text = if is_edit_mode {
        "GitHub Projects TUI - Edit Ticket"
    } else {
        "GitHub Projects TUI - Ticket Details"
    };
    let title = Paragraph::new(title_text)
        .style(Style::default().fg(Color::Cyan).bold())
        .alignment(Alignment::Center)
        .block(Block::default().borders(Borders::ALL));
    frame.render_widget(title, chunks[0]);

    // Content
    if let Some(ticket) = &state.selected_ticket {
        let content_chunks = Layout::default()
            .direction(Direction::Vertical)
            .constraints([
                Constraint::Length(3),
                Constraint::Length(3),
                Constraint::Length(3),
                Constraint::Min(0),
            ])
            .split(chunks[1]);

        // Number and State
        let info = Paragraph::new(format!("#{} | State: {}", ticket.number, ticket.state))
            .style(Style::default().fg(Color::White))
            .block(Block::default().borders(Borders::ALL).title("Info"));
        frame.render_widget(info, content_chunks[0]);

        // Title
        let title_widget = Paragraph::new(ticket.title.clone())
            .style(if is_edit_mode {
                Style::default().fg(Color::Yellow)
            } else {
                Style::default().fg(Color::White)
            })
            .block(Block::default().borders(Borders::ALL).title("Title"));
        frame.render_widget(title_widget, content_chunks[1]);

        // Status
        let status_text = ticket.status.clone().unwrap_or_else(|| "None".to_string());
        let status_widget = Paragraph::new(status_text)
            .style(if is_edit_mode {
                Style::default().fg(Color::Yellow)
            } else {
                Style::default().fg(Color::White)
            })
            .block(Block::default().borders(Borders::ALL).title("Status"));
        frame.render_widget(status_widget, content_chunks[2]);

        // Body
        let body_text = ticket.body.clone().unwrap_or_else(|| "No description".to_string());
        let body_widget = Paragraph::new(body_text)
            .style(if is_edit_mode {
                Style::default().fg(Color::Yellow)
            } else {
                Style::default().fg(Color::White)
            })
            .block(Block::default().borders(Borders::ALL).title("Description"))
            .wrap(Wrap { trim: false });
        frame.render_widget(body_widget, content_chunks[3]);
    } else {
        let loading = Paragraph::new("Loading ticket details...")
            .style(Style::default().fg(Color::Yellow))
            .alignment(Alignment::Center)
            .block(Block::default().borders(Borders::ALL));
        frame.render_widget(loading, chunks[1]);
    }

    // Help text
    let help_text = if is_edit_mode {
        "Tab: Next Field | Ctrl+S: Save | Esc: Cancel"
    } else {
        "e: Edit | Esc: Back | q: Quit"
    };
    let help = Paragraph::new(help_text)
        .style(Style::default().fg(Color::Gray))
        .alignment(Alignment::Center)
        .block(Block::default().borders(Borders::ALL));
    frame.render_widget(help, chunks[2]);
}
