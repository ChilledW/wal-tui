use ratatui::{
    DefaultTerminal, Frame,
    layout::{Alignment, Constraint, Direction, Layout},
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{Block, List, Paragraph},
};
use ratatui_image::{StatefulImage, picker::Picker, protocol::StatefulProtocol};

use crate::app_logic::{AppLogic, Events};

pub fn render(frame: &mut Frame, app: &mut AppLogic) {
    // frame.render_widget("Hello World!", frame.area());
    let outer_layout = Layout::default()
        .direction(Direction::Horizontal)
        .margin(1)
        .constraints(vec![Constraint::Percentage(60), Constraint::Percentage(40)])
        .split(frame.area());

    let display_items: Vec<String> = app.get_displayable_items().to_vec();

    let item_list = List::new(display_items.iter().map(|s| s.as_str()))
        .block(
            Block::bordered()
                .title("List of things")
                .title_alignment(Alignment::Center)
                .title_bottom(
                    Line::from(" j/k: up/down | q: quit ")
                        .style(Style::default().fg(Color::Yellow)),
                )
                .style(Style::default().fg(Color::Red)),
        )
        .highlight_style(
            Style::default()
                .fg(Color::White)
                .add_modifier(Modifier::BOLD),
        )
        .highlight_symbol(">> ");

    let state = app.get_state();

    // let path = app.selected_item();

    // let right = outer_layout[1];
    // let preview_block = Block::bordered().title("Wallpaper Preview");
    // let inner = preview_block.inner(right);
    frame.render_stateful_widget(item_list, outer_layout[0], state);
}
