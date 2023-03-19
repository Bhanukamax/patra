use super::{
    app::{PatraFileItemType, PatraFileListItem, PatraFileState},
    logger,
};
use std::io::Write;
use termion::{self, color, screen::AlternateScreen, style};
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
    let mut items = vec![];
    if let Some(file_list) = state.list.as_ref() {
        logger::debug(&format!("Some: {:?}", file_list));
        file_list
            .iter()
            .enumerate()
            .map(|(idx, x)| {
                items.push(render_item(x, idx as u16, idx as u16 == state.c_idx));
            })
            .collect()
    }
    let list = List::new(items)
        .block(Block::default().title("List").borders(Borders::ALL))
        .style(Style::default().fg(Color::White))
        .highlight_style(Style::default().add_modifier(Modifier::ITALIC))
        .highlight_symbol(">>");
    f.render_widget(list, chunks[1]);
    Ok(())
}


pub fn render_title<W: Write>(terminal: &mut Terminal<TermionBackend<W>>) {
    terminal
        .draw(|f| {
            let size = f.size();
            let block = Block::default().title("Patra").borders(Borders::ALL);
            f.render_widget(block, size);
        })
        .unwrap();
}

pub fn render_item(item: &PatraFileListItem, idx: u16, selected: bool) -> ListItem {
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
