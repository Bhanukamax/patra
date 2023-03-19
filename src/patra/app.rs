// use super::display::move_cursor_cursor;
use super::logger;
use std::fs;

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

    pub fn list_dir(&mut self) {
        if let Ok(list) = fs::read_dir(&self.path) {
            self.list = list
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
                .collect()
        } else {
            self.list = vec![]
        }
    }

    pub fn enter(&mut self) -> Result<(), std::io::Error> {
        let item = &self.list[self.c_idx as usize];
        if item.file_type == PatraFileItemType::Dir {
            self.path = match self.path.as_str() {
                "/" => format!("/{}", &item.name),
                _ => format!("{}/{}", &self.path, &item.name),
            };
            self.list_dir();
            self.c_idx = 0;
        }

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
            self.list_dir();
        }
        Ok(())
    }

    pub fn next(&mut self) {
        logger::debug(&format!("NEXT >>>> {} {}", self.c_idx, self.path));
        self.c_idx = match self.c_idx {
            idx if idx == self.list.len() as u16 => 0,
            _ => self.c_idx + 1,
        }
    }

    pub fn prev(&mut self) {
        logger::debug(&format!("PREV <<<<< {} {}", self.c_idx, self.path));
        self.c_idx = match self.c_idx {
            0 => self.list.len() as u16,
            _ => self.c_idx - 1,
        }
    }
}
