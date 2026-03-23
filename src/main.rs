use color_eyre::Result;
use crossterm::event::{self, Event};
use ratatui::{
    DefaultTerminal, Frame,
    layout::{Constraint, Direction, Layout},
    style::{Color, Modifier, Style},
    widgets::{Block, List, ListState},
};

struct App {
    items: Vec<String>,
    new_items: Vec<String>,
    state: ListState,
}

impl App {
    fn new(initial_list: Vec<&str>) -> Self {
        let mut state = ListState::default();
        state.select(Some(0));
        let list_to_store: Vec<String> = initial_list.iter().map(|s| s.to_string()).collect();
        let new_items: Vec<String> = Vec::new();

        Self {
            items: list_to_store,
            new_items,
            state,
        }
    }

    fn next(&mut self) {
        let i = match self.state.selected() {
            Some(i) => {
                if i >= self.items.len() - 1 {
                    0
                } else {
                    i + 1
                }
            }
            _ => 0,
        };
        self.state.select(Some(i));
    }

    fn prev(&mut self) {
        let i = match self.state.selected() {
            Some(i) => {
                if i == 0 {
                    self.items.len()
                } else {
                    i - 1
                }
            }
            _ => 0,
        };
        self.state.select(Some(i));
    }

    fn select(&mut self) {
        if let Some(i) = self.state.selected() {
            self.new_items.push(self.items[i].clone());
        }
    }
}

fn main() -> Result<()> {
    let mut app = App::new(vec!["one", "two", "three"]);
    app.state.select(Some(0));
    color_eyre::install()?;
    let terminal = ratatui::init();
    let result = run(terminal, &mut app);
    ratatui::restore();
    result
}

fn run(mut terminal: DefaultTerminal, app: &mut App) -> Result<()> {
    loop {
        terminal.draw(|frame| render(frame, app))?;
        let Event::Key(key) = event::read()? else {
            continue;
        };
        if let event::KeyCode::Char(c) = key.code {
            if c == 'q' {
                break Ok(());
            }
            if c == 'j' {
                app.next();
            }
            if c == 'k' {
                app.prev();
            }
            if c == 's' {
                app.select();
            }
        }
    }
}

fn render(frame: &mut Frame, app: &mut App) {
    // frame.render_widget("Hello World!", frame.area());
    let outer_layout = Layout::default()
        .direction(Direction::Horizontal)
        .margin(1)
        .constraints(vec![Constraint::Percentage(60), Constraint::Percentage(40)])
        .split(frame.area());

    let item_list = List::new(app.items.clone())
        .block(
            Block::bordered()
                .title("List of things")
                .style(Style::default().fg(Color::Red)),
        )
        .highlight_style(
            Style::default()
                .fg(Color::White)
                .add_modifier(Modifier::BOLD),
        )
        .highlight_symbol(">> ");

    // let image_display = Block::bordered()
    //    .title("Block 2")
    //    .style(Style::default().fg(Color::LightCyan));

    let item_list2 = List::new(app.new_items.clone())
        .block(
            Block::bordered()
                .title("List of selsected things")
                .style(Style::default().fg(Color::Blue)),
        )
        .style(Style::default().fg(Color::Magenta));

    frame.render_stateful_widget(item_list, outer_layout[0], &mut app.state);
    frame.render_widget(item_list2, outer_layout[1]);
}
