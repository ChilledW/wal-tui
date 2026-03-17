use color_eyre::Result;
use crossterm::event::{self, Event};
use ratatui::{DefaultTerminal, Frame, layout::{Constraint, Direction, Layout}, style::{Color, Style}, widgets::{Block, List}};

fn main() -> Result<()> {
    color_eyre::install()?;
    let terminal = ratatui::init();
    let result = run(terminal);
    ratatui::restore();
    return result;    
}

fn run(mut terminal: DefaultTerminal) -> Result<()> {
    loop {
        terminal.draw(render)?;
        if let Event::Key(key) = event::read()? {
            match key.code {
                event::KeyCode::Char(c) => {
                    if c  == 'q' {
                        break Ok(())
                    }
                }
                _ => ()
            }
        }
    }
}



fn render(frame: &mut Frame) {
    // frame.render_widget("Hello World!", frame.area());
    let outer_layout = Layout::default()
        .direction(Direction::Horizontal)
        .margin(1)
        .constraints(vec![Constraint::Percentage(60),
                          Constraint::Percentage(40)])
        .split(frame.area());

//     let item_list = Block::bordered()
//         .title("Block 1")
//         .style(Style::default().fg(Color::Red));

    let strings: Vec<&str> = vec!["item1", "item2", "item3"];

    let item_list = List::new(strings)
        .block(Block::bordered()
                .title("List of things")
                .style(Style::default().fg(Color::Red))
        )
        .style(Style::default().fg(Color::Red)); 

    let image_display = Block::bordered()
        .title("Block 2")
        .style(Style::default().fg(Color::LightCyan));

    frame.render_widget(item_list, outer_layout[0]);
    frame.render_widget(image_display, outer_layout[1]);

}


