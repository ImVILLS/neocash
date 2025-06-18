// completion/menu.rs

use crossterm::{
    event::{self, Event, KeyCode, KeyEvent},
    execute,
    terminal::{EnterAlternateScreen, LeaveAlternateScreen, enable_raw_mode, disable_raw_mode},
};
use std::io::stdout;
use tui::{
    backend::CrosstermBackend,
    widgets::{Block, Borders, List, ListItem, ListState},
    Terminal,
    layout::{Layout, Constraint, Direction},
    style::{Style, Color, Modifier},
};

pub struct CompletionMenu {
    items: Vec<String>,
    selected: usize,
    filter: String,
}

impl CompletionMenu {
    pub fn new(items: Vec<String>) -> Self {
        Self { 
            items,
            selected: 0,
            filter: String::new(),
        }
    }

    pub fn show(&mut self) -> Option<String> {
        enable_raw_mode().ok()?;
        let mut stdout = stdout();
        execute!(stdout, EnterAlternateScreen).ok()?;

        let backend = CrosstermBackend::new(stdout);
        let mut terminal = Terminal::new(backend).ok()?;
        let mut list_state = ListState::default();
        list_state.select(Some(self.selected));

        let result = loop {
            let filtered_items: Vec<String> = self.items.iter()
                .filter(|item| item.starts_with(&self.filter))
                .cloned()
                .collect();

            if filtered_items.is_empty() {
                break None;
            }

            // Adjust selected index if out of bounds
            if self.selected >= filtered_items.len() {
                self.selected = filtered_items.len().saturating_sub(1);
            }

            terminal.draw(|f| {
                let size = f.size();
                let block = Block::default()
                    .borders(Borders::ALL)
                    .title(format!("Completion (Filter: '{}') ↑/↓: navigate, Enter: select, Esc: cancel", self.filter));
                
                f.render_widget(block, size);

                let list_area = Layout::default()
                    .direction(Direction::Vertical)
                    .margin(1)
                    .constraints([Constraint::Min(1)].as_ref())
                    .split(size);

                let items: Vec<ListItem> = filtered_items
                    .iter()
                    .enumerate()
                    .map(|(i, item)| {
                        let style = if i == self.selected {
                            Style::default()
                                .fg(Color::Yellow)
                                .add_modifier(Modifier::BOLD)
                        } else {
                            Style::default()
                        };
                        ListItem::new(item.as_str()).style(style)
                    })
                    .collect();

                let list = List::new(items)
                    .highlight_style(Style::default().bg(Color::DarkGray));
                f.render_stateful_widget(list, list_area[0], &mut list_state);
            }).ok()?;

            if let Event::Key(KeyEvent { code, .. }) = event::read().ok()? {
                match code {
                    KeyCode::Up => {
                        if self.selected > 0 {
                            self.selected -= 1;
                        }
                    }
                    KeyCode::Down => {
                        if self.selected < filtered_items.len().saturating_sub(1) {
                            self.selected += 1;
                        }
                    }
                    KeyCode::Char(c) => {
                        self.filter.push(c);
                        self.selected = 0;
                    }
                    KeyCode::Backspace => {
                        self.filter.pop();
                        self.selected = 0;
                    }
                    KeyCode::Enter => {
                        if let Some(selected) = filtered_items.get(self.selected) {
                            break Some(selected.clone());
                        }
                    }
                    KeyCode::Esc => {
                        break None;
                    }
                    _ => {}
                }
                list_state.select(Some(self.selected));
            }
        };

        disable_raw_mode().ok()?;
        execute!(terminal.backend_mut(), LeaveAlternateScreen).ok()?;
        terminal.show_cursor().ok()?;

        result
    }
}
