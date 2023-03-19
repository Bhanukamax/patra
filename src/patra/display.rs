use super::{
    app::{PatraFileItemType, PatraFileListItem, PatraFileState},
    logger,
};
use std::io::Write;

use tui::{
    backend::{Backend, TermionBackend},
    layout::{Constraint, Direction, Layout, Rect},
    style::{Color, Modifier, Style},
    widgets::{Block, Borders, List, ListItem},
    Frame, Terminal,
};

pub fn render<W: Write>(
    terminal: &mut Terminal<TermionBackend<W>>,
    state: &PatraFileState,
) -> Result<(), std::io::Error> {
    terminal.draw(|f| render_app(f, state).unwrap()).unwrap();
    Ok(())
}

pub fn render_ui<B: Backend>(f: &mut Frame<B>) -> Vec<Rect> {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .margin(1)
        .constraints([Constraint::Percentage(10), Constraint::Percentage(20)].as_ref())
        .split(f.size());
    return chunks;
}

pub fn render_app<B: Backend>(
    f: &mut Frame<B>,
    state: &PatraFileState,
) -> Result<(), std::io::Error> {
    let chunks = render_ui(f);
    render_title(f, chunks[0], state);
    render_list(f, chunks[1], state);
    Ok(())
}

pub fn render_list<B: Backend>(f: &mut Frame<B>, chunk: Rect, state: &PatraFileState) {
    let mut items = vec![];
    logger::debug(&format!("Some: {:?}", state.list));
    state
        .list
        .iter()
        .enumerate()
        .map(|(idx, x)| {
            items.push(render_item(x, idx as u16, idx as u16 == state.c_idx));
        })
        .for_each(|_| {});

    let mut border: Borders = Borders::ALL;
    border.remove(Borders::TOP);
    let list = List::new(items)
        .block(Block::default().title(state.path.as_str()).borders(border))
        .style(Style::default().fg(Color::White))
        .highlight_style(Style::default().add_modifier(Modifier::ITALIC))
        .highlight_symbol(">>");
    f.render_widget(list, chunk);
}

pub fn render_title<B: Backend>(f: &mut Frame<B>, chunk: Rect, state: &PatraFileState) {
    // let path = Text::from(state.path.as_str());
    let block = Block::default()
        .title(state.path.as_str())
        .borders(Borders::RIGHT)
        .borders(Borders::RIGHT);
    f.render_widget(block, chunk);
}

pub fn render_item(item: &PatraFileListItem, _idx: u16, selected: bool) -> ListItem {
    // set_style_file(screen);
    let mut style: Style = Style::default().fg(Color::White);
    let (icon, suffix) = match item.file_type {
        PatraFileItemType::Dir => {
            style = style.fg(Color::Blue).add_modifier(Modifier::BOLD);
            ("", "/")
        }
        PatraFileItemType::File => ("", ""),
        PatraFileItemType::Sym => ("", ""),
        PatraFileItemType::Unknown => ("⚠", ""),
    };
    if selected {
        style = style
            .add_modifier(Modifier::BOLD)
            .add_modifier(Modifier::UNDERLINED);
    }

    return ListItem::new(format!("{} {}{}", icon, item.name, suffix)).style(style);
}
