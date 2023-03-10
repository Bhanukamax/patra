use crossterm::{
    cursor,
    event::{poll, read, Event, KeyCode, KeyEvent},
    style::{self, Stylize},
    terminal::{self, enable_raw_mode},
    ExecutableCommand, QueueableCommand, Result,
};
use std::{
    io::{stdout, Write},
    time::Duration,
};

fn main() -> Result<()> {
    let mut stdout = stdout();

    enable_raw_mode()?;
    stdout.execute(terminal::Clear(terminal::ClearType::All))?;

    // for y in 0..40 {
    //     for x in 0..150 {
    //         if (y == 0 || y == 40 - 1) || (x == 0 || x == 150 - 1) {
    //             // in this loop we are more efficient by not flushing the buffer.
    //             stdout
    //                 .queue(cursor::MoveTo(x, y))?
    //                 .queue(style::PrintStyledContent("█".magenta()))?;
    //         }
    //     }
    // }

    stdout
        .queue(cursor::MoveTo(0, 0))?
        .queue(style::PrintStyledContent("item".magenta()))?;
    stdout
        .queue(cursor::MoveTo(0, 2))?
        .queue(style::PrintStyledContent("item".magenta()))?;
    stdout.flush()?;
    handle_events()?;

    Ok(())
}

fn handle_key_events(event: Event) {
    match event {
        Event::Key(KeyEvent {
            code: KeyCode::Char('q'),
            ..
        }) => {
            println!("got key ");
            std::process::exit(1)
        },
        _ => println!("zzzzzzzzzz {:?}", event),
    }
}

fn handle_events() -> crossterm::Result<()> {
    loop {
        if poll(Duration::from_millis(500))? {
            match read()? {
                Event::Key(event) => handle_key_events(Event::Key(event)),
                _ => println!("testing"),
            }
        }
    }
    Ok(())
}
