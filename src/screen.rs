pub struct Screen;
use crate::ui::Rect;
use std::io::{self, Write};

impl Default for Screen {
    fn default() -> Self {
        Screen
    }
}

impl Screen {
    pub fn render() -> Result<(), std::io::Error> {
        println!("{}", termion::clear::All);
        print!("{}", termion::cursor::Goto(1, 1));
        let mut title = Rect::new(2_u16, 1_u16, 30_u16, 2_u16);
        // title.draw();
        title.add_line("Patra: File Manager");

        print!("{}", termion::color::Fg(termion::color::Blue));

        let mut file_list = Rect::new(2_u16, 3_u16, 22_u16, 10_u16);
        file_list.draw();
        print!("{}", termion::color::Fg(termion::color::White));
        file_list.add_line("file one");
        file_list.add_line("file two");
        file_list.add_line("file three");

        let mut file_list = Rect::new(25_u16, 3_u16, 30_u16, 10_u16);
        file_list.draw();
        print!("{}", termion::color::Fg(termion::color::White));
        file_list.add_line("file one");
        file_list.add_line("file two");
        file_list.add_line("file three");

        let mut file_list = Rect::new(2_u16, 14_u16, 30_u16, 10_u16);
        file_list.draw();
        print!("{}", termion::color::Fg(termion::color::White));
        file_list.add_line("file one");
        file_list.add_line("file two");
        file_list.add_line("file three");

        io::stdout().flush()?;
        Ok(())
    }
}
