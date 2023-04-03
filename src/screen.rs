pub struct Screen;
use crate::{
    patra::app::{PatraFileListItem, PatraFileState},
    ui::{ListWidget, Rect},
};
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

        let file_list_one = Rect::new(2_u16, 3_u16, 12_u16, 5_u16);
        file_list_one.draw();

        let mut list: ListWidget<PatraFileListItem, Box<dyn Fn(&PatraFileListItem) -> String>> =
            ListWidget::new(
                [].to_vec(),
                file_list_one,
                Box::new(|item| item.name.as_str().to_string()),
            );

        print!("{}", termion::color::Fg(termion::color::White));

        let mut file_list_two = Rect::new(25_u16, 3_u16, 30_u16, 10_u16);
        file_list_two.draw();
        print!("{}", termion::color::Fg(termion::color::White));

        let mut state = PatraFileState::new("./".to_string());
        state.list_dir()?;

        list.update_list(state.clone().get_list().clone().to_vec());

        list.populate();

        let mut file_list = Rect::new(2_u16, 14_u16, 30_u16, 10_u16);
        file_list.draw();

        state.get_list().iter().for_each(|file| {
            // file_list_one.add_line(file.name.as_str());
            file_list_two.add_line(file.name.as_str());
            file_list.add_line(file.name.as_str());
        });

        io::stdout().flush()?;
        Ok(())
    }
}
