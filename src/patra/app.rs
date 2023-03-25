// use super::display::move_cursor_cursor;
use super::logger;
use std::fs;
use std::io::Stdout;
use termion::screen::AlternateScreen;

#[derive(Clone, Debug, PartialEq)]
pub enum PatraFileItemType {
    File,
    Dir,
    Sym,
    Unknown,
}

#[derive(Clone, Debug)]
pub struct PatraFileListItem {
    pub name: String,
    pub file_type: PatraFileItemType,
}

#[derive(std::clone::Clone, Debug)]
pub struct PatraFileState {
    pub list: Vec<PatraFileListItem>,
    pub path: String,
    pub c_idx: u16,
}

impl PatraFileState {
    pub fn new(path: String) -> PatraFileState {
        PatraFileState {
            path,
            list: vec![],
            c_idx: 1,
        }
    }

    pub fn list_dir(&mut self) -> std::io::Result<()> {
        let dir_list = fs::read_dir(&self.path)?;
        self.list = dir_list
            .into_iter()
            .map(|x| PatraFileListItem {
                name: String::from(x.as_ref().unwrap().file_name().to_str().unwrap()),
                file_type: if x.as_ref().unwrap().path().is_dir() {
                    PatraFileItemType::Dir
                } else if x.as_ref().unwrap().path().is_file() {
                    PatraFileItemType::File
                } else if x.unwrap().path().is_symlink() {
                    PatraFileItemType::Sym
                } else {
                    PatraFileItemType::Unknown
                },
            })
            .collect();
        self.c_idx = 1;

        Ok(())
    }

    pub fn enter(&mut self, _screen: &mut AlternateScreen<Stdout>) -> Result<(), std::io::Error> {
        let idx: usize = self.c_idx as usize - 1;
        let original_path = String::from(&self.path);
        let old_idx = self.c_idx;
        let new_path = &self
            .list
            .iter()
            .enumerate()
            .filter(|(i, item)| {
                logger::debug(&format!("items : {:?}", &item));
                if item.file_type == PatraFileItemType::Dir && i == &idx {
                    true
                } else {
                    false
                }
            })
            .map(|(_, item)| match self.path.as_str() {
                "/" => format!("/{}", &item.name),
                _ => format!("{}/{}", &self.path, &item.name),
            })
            .collect::<Vec<_>>();

        logger::debug(&format!("New path: {:?}", new_path));
        self.path = match new_path.last().cloned() {
            Some(x) => x,
            None => self.path.clone(),
        };

        self.list_dir()
            .map_err(|e| -> Result<(), std::io::Error> {
                self.path = original_path;
                self.c_idx = old_idx;
                logger::error(&format!("Error opening: {:?}", &e));
                Ok(())
            })
            .iter();
        logger::debug(&format!("new path: {:?}", &self.path));
        // logger::log!("new path: {:?}", &self.path)?;

        Ok(())
    }

    pub fn up_dir(&mut self) -> Result<(), std::io::Error> {
        if &self.path != "/" {
            self.path = std::path::Path::new(&self.path)
                .parent()
                .unwrap()
                .to_str()
                .unwrap()
                .to_string();
            self.list_dir()?;
        }
        Ok(())
    }

    pub fn next(&mut self) {
        self.c_idx = if self.c_idx == self.list.len() as u16 {
            1
        } else {
            self.c_idx + 1
        };
    }

    pub fn prev(&mut self) {
        self.c_idx = if self.c_idx == 1 {
            self.list.len() as u16
        } else {
            self.c_idx - 1
        }
    }
}
