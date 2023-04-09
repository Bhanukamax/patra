use crate::logger;
use std::{fs, io::Write};

#[derive(Default)]
pub struct Flags {
    write_to_file: bool,
}

pub struct App {
    pub should_quit: bool,
    pub state: PatraFileState,
    pub flags: Flags,
    pub selection_file_path: String,
}

impl Default for App {
    fn default() -> Self {
        Self {
            selection_file_path: "".into(),
            should_quit: false,
            flags: Flags::default(),
            state: PatraFileState::new(String::from(
                std::env::current_dir().unwrap().to_str().unwrap(),
            )),
        }
    }
}

impl App {
    pub fn set_should_write_to_file(&mut self, file_path: String) {
        self.flags.write_to_file = true;
        self.selection_file_path = file_path;
    }

    pub fn quit(&mut self) {
        self.should_quit = true;
    }

    pub fn enter(&mut self) -> Result<(), std::io::Error> {
        let idx: usize = self.state.c_idx as usize - 1;
        let original_path = String::from(&self.state.path);
        let old_idx = self.state.c_idx;
        let current_file = self.state.list.get(idx).unwrap();

        logger::debug(&format!("Current file: {:?}", current_file));
        let mut new_path = original_path.clone();
        if current_file.file_type == PatraFileItemType::Dir {
            new_path = match self.state.path.as_str() {
                "/" => format!("/{}", &current_file.name),
                _ => format!("{}/{}", &self.state.path, &current_file.name),
            }
        } else if self.flags.write_to_file {
            let mut file = fs::File::create(self.selection_file_path.clone())
                .expect("Could not output file to write the selection path");
            write!(file, "{}/{}", &original_path, current_file.name)
                .expect("Could not write to file");
            self.quit()
        }

        logger::debug(&format!("New path: {:?}", new_path));
        self.state.path = new_path;

        self.state
            .list_dir()
            .map_err(|e| -> Result<(), std::io::Error> {
                self.state.path = original_path;
                self.state.c_idx = old_idx;
                logger::error(&format!("Error opening: {:?}", &e));
                Ok(())
            })
            .iter();
        logger::debug(&format!("new path: {:?}", &self.state.path));

        Ok(())
    }
}

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
