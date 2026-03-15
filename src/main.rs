use crossterm::{
    event::{self, DisableMouseCapture, EnableMouseCapture, Event, KeyCode, KeyModifiers},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use project_tui::{
    app::state::AppState,
    config::settings::Settings,
    github::{auth::Auth, client::GitHubClient},
    ui,
};
use ratatui::{
    backend::CrosstermBackend,
    Terminal,
};
use std::io;
use std::time::Duration;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    // Load configuration
    let settings = match Settings::load() {
        Ok(s) => s,
        Err(e) => {
            eprintln!("Error: {}", e);
            eprintln!("\nExample configuration file:\n");
            eprintln!("{}", Settings::example());
            eprintln!("Save this to: {}", Settings::config_path()?.display());
            std::process::exit(1);
        }
    };

    // Initialize GitHub client
    let auth = Auth::new(settings.github.token.clone());
    let github_client = GitHubClient::new(auth, Some(settings.github.api_url.clone()))?;

    // Setup terminal
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    // Initialize app state
    let mut app_state = AppState::new();

    // Run the application
    let result = run_app(&mut terminal, &mut app_state, github_client).await;

    // Restore terminal
    disable_raw_mode()?;
    execute!(
        terminal.backend_mut(),
        LeaveAlternateScreen,
        DisableMouseCapture
    )?;
    terminal.show_cursor()?;

    if let Err(e) = result {
        eprintln!("Error: {}", e);
    }

    Ok(())
}

async fn run_app<B: ratatui::backend::Backend>(
    terminal: &mut Terminal<B>,
    state: &mut AppState,
    github_client: GitHubClient,
) -> anyhow::Result<()> {
    // Load initial projects
    state.set_loading(true);
    match github_client.fetch_projects().await {
        Ok(projects) => {
            state.projects = projects;
            state.set_loading(false);
        }
        Err(e) => {
            state.set_error(format!("Failed to load projects: {}", e));
        }
    }

    loop {
        // Render
        terminal.draw(|f| {
            ui::router::render(f, state);
        })?;

        // Handle input with timeout
        if event::poll(Duration::from_millis(100))? {
            if let Event::Key(key) = event::read()? {
                // Global quit handlers
                if key.code == KeyCode::Char('q')
                    || (key.code == KeyCode::Char('c')
                        && key.modifiers.contains(KeyModifiers::CONTROL))
                {
                    state.quit();
                }

                // Screen-specific handlers
                handle_key_event(state, key, &github_client).await?;
            }
        }

        if state.should_quit {
            break;
        }
    }

    Ok(())
}

async fn handle_key_event(
    state: &mut AppState,
    key: event::KeyEvent,
    github_client: &GitHubClient,
) -> anyhow::Result<()> {
    use project_tui::app::state::Screen;

    match &state.current_screen {
        Screen::ProjectSelector => {
            match key.code {
                KeyCode::Char('j') | KeyCode::Down => {
                    state.move_selection_down(state.projects.len());
                }
                KeyCode::Char('k') | KeyCode::Up => {
                    state.move_selection_up();
                }
                KeyCode::Enter => {
                    if !state.projects.is_empty() {
                        let project = state.projects[state.selected_index].clone();
                        state.navigate_to(Screen::TicketList {
                            project_id: project.id.clone(),
                        });

                        // Load tickets for the selected project
                        state.set_loading(true);
                        match github_client.fetch_project_items(&project.id).await {
                            Ok(tickets) => {
                                state.tickets = tickets;
                                state.set_loading(false);
                            }
                            Err(e) => {
                                state.set_error(format!("Failed to load tickets: {}", e));
                            }
                        }
                    }
                }
                _ => {}
            }
        }
        Screen::TicketList { project_id } => {
            match key.code {
                KeyCode::Char('j') | KeyCode::Down => {
                    state.move_selection_down(state.tickets.len());
                }
                KeyCode::Char('k') | KeyCode::Up => {
                    state.move_selection_up();
                }
                KeyCode::Enter => {
                    if !state.tickets.is_empty() {
                        let ticket = state.tickets[state.selected_index].clone();

                        // Convert Ticket to TicketDetail
                        let ticket_detail = project_tui::github::models::TicketDetail {
                            id: ticket.id.clone(),
                            title: ticket.title.clone(),
                            number: ticket.number,
                            body: None, // Will need to fetch full details later
                            status: ticket.status.clone(),
                            repository: ticket.repository.clone(),
                            state: ticket.state.clone(),
                            url: String::new(), // Placeholder
                        };

                        state.selected_ticket = Some(ticket_detail);
                        state.navigate_to(Screen::TicketDetail {
                            project_id: project_id.clone(),
                            item_id: ticket.id,
                        });
                    }
                }
                KeyCode::Esc => {
                    state.navigate_to(Screen::ProjectSelector);
                }
                _ => {}
            }
        }
        Screen::TicketDetail { project_id, item_id } => {
            match key.code {
                KeyCode::Char('e') => {
                    state.navigate_to(Screen::TicketEdit {
                        project_id: project_id.clone(),
                        item_id: item_id.clone(),
                    });
                }
                KeyCode::Esc => {
                    state.navigate_to(Screen::TicketList {
                        project_id: project_id.clone(),
                    });
                }
                _ => {}
            }
        }
        Screen::TicketEdit { project_id, .. } => {
            match key.code {
                KeyCode::Esc => {
                    if let Some(ticket) = &state.selected_ticket {
                        state.navigate_to(Screen::TicketDetail {
                            project_id: project_id.clone(),
                            item_id: ticket.id.clone(),
                        });
                    }
                }
                _ => {}
            }
        }
    }

    Ok(())
}
