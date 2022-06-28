

use crate::app::App;
use gethostname::gethostname;
use tui::{backend::Backend, Frame, layout::{Layout, Constraint, Direction}, widgets::{Block, Borders, BorderType}, style::{Style, Color}, text::Spans
};
use username::get_user_name;

pub fn draw<B: Backend>(f: &mut Frame<B>, app: &mut App) {
    let chunks = Layout::default()
    .direction(Direction::Vertical)
    .margin(1)
    .constraints(
        [
            Constraint::Percentage(13),
            Constraint::Percentage(87),
            Constraint::Percentage(0)
        ].as_ref()
    )
    .split(f.size());

    let username = get_user_name().unwrap().to_string();
    let hostname = gethostname().to_str().unwrap().to_string();
    let block = Block::default()
        .title(username + "@" + hostname.as_str())
        .borders(Borders::ALL);
f.render_widget(block, chunks[0]);
let block = Block::default()
     .title("Block 2")
     .borders(Borders::ALL);
f.render_widget(block, chunks[1]);
}

