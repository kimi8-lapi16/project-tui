use crate::app::state::AppState;
use ratatui::{
    layout::{Alignment, Constraint, Direction, Layout},
    prelude::*,
    widgets::{Block, Borders, Cell, Paragraph, Row, Table},
};

pub fn render(frame: &mut Frame, state: &AppState) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(3),
            Constraint::Min(0),
            Constraint::Length(3),
        ])
        .split(frame.area());

    // Title
    let title = Paragraph::new("GitHub Projects TUI - Ticket List")
        .style(Style::default().fg(Color::Cyan).bold())
        .alignment(Alignment::Center)
        .block(Block::default().borders(Borders::ALL));
    frame.render_widget(title, chunks[0]);

    // Ticket table
    if state.loading {
        let loading = Paragraph::new("Loading tickets...")
            .style(Style::default().fg(Color::Yellow))
            .alignment(Alignment::Center)
            .block(Block::default().borders(Borders::ALL).title("Tickets"));
        frame.render_widget(loading, chunks[1]);
    } else if state.tickets.is_empty() {
        let empty = Paragraph::new("No tickets found in this project.")
            .style(Style::default().fg(Color::Yellow))
            .alignment(Alignment::Center)
            .block(Block::default().borders(Borders::ALL).title("Tickets"));
        frame.render_widget(empty, chunks[1]);
    } else {
        let header_cells = ["#", "Title", "Status", "State", "Repository"]
            .iter()
            .map(|h| Cell::from(*h).style(Style::default().fg(Color::Yellow).bold()));
        let header = Row::new(header_cells).height(1).bottom_margin(1);

        let rows: Vec<Row> = state
            .tickets
            .iter()
            .enumerate()
            .map(|(idx, ticket)| {
                let cells = vec![
                    Cell::from(ticket.number.to_string()),
                    Cell::from(ticket.title.clone()),
                    Cell::from(ticket.status.clone().unwrap_or_else(|| "None".to_string())),
                    Cell::from(ticket.state.clone()),
                    Cell::from(
                        ticket
                            .repository
                            .clone()
                            .unwrap_or_else(|| "Unknown".to_string()),
                    ),
                ];

                let style = if idx == state.selected_index {
                    Style::default().bg(Color::DarkGray).fg(Color::White)
                } else {
                    Style::default()
                };

                Row::new(cells).style(style)
            })
            .collect();

        let table = Table::new(
            rows,
            [
                Constraint::Length(6),
                Constraint::Min(30),
                Constraint::Length(15),
                Constraint::Length(10),
                Constraint::Length(20),
            ],
        )
        .header(header)
        .block(Block::default().borders(Borders::ALL).title("Tickets"))
        .column_spacing(1);

        frame.render_widget(table, chunks[1]);
    }

    // Help text
    let help = Paragraph::new("j/k or ↓/↑: Navigate | Enter: View Details | Esc: Back | q: Quit")
        .style(Style::default().fg(Color::Gray))
        .alignment(Alignment::Center)
        .block(Block::default().borders(Borders::ALL));
    frame.render_widget(help, chunks[2]);
}
