use super::display::{move_cursor_cursor, set_style_dir};
use std::fs;
use std::io::{Stdout, Write};
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

// TODO: make the path vector to easily go back and forth
#[derive(std::clone::Clone)]
pub struct PatraFileState {
    pub list: Option<Vec<PatraFileListItem>>,
    pub path: String,
    pub c_idx: u16,
    error: Vec<String>,
}

impl PatraFileState {
    pub fn new(path: String) -> PatraFileState {
        PatraFileState {
            path,
            list: None,
            c_idx: 1,
            error: vec![],
        }
    }

    pub fn get_error(&self) -> &Vec<String> {
        &self.error
    }

    pub fn list_dir(&mut self) -> std::io::Result<()> {
        // let dir_list = read_dir(&self.path).unwrap();
        let dir_list = fs::read_dir(&self.path)?;
        self.list = Some(
            dir_list
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
                .collect(),
        );

        self.c_idx = 1;
        Ok(())
    }

    pub fn enter(&mut self, screen: &mut AlternateScreen<Stdout>) -> Result<(), std::io::Error> {
        let idx: usize = self.c_idx as usize - 1;
        let original_path = String::from(&self.path);
        let new_path = self
            .list
            .as_ref()
            .map(|item| &item[idx])
            .filter(|items| items.file_type == PatraFileItemType::Dir)
            .iter()
            .map(|item| match self.path.as_str() {
                "/" => format!("/{}", &item.name),
                _ => format!("{}/{}", &self.path, &item.name),
            })
            .collect::<Vec<_>>();

        self.path = new_path.last().cloned().unwrap_or(self.path.clone());

        if let Err(e) = self.list_dir() {
            self.error.push(e.to_string());
            move_cursor_cursor(screen, 10, 2);
            self.path = original_path;
        }
        Ok(())
    }

    pub fn up_dir(&mut self) -> Result<(), std::io::Error>{
        if &self.path != "/" {
            self.path = std::path::Path::new(&self.path)
                .parent()
                .unwrap()
                .to_str()
                .unwrap()
                .to_string();
            self.list_dir()?;
        }
        let out = std::io::stderr();
        writeln!(&out, "updated dir {:?}", self.path)?;
        Ok(())
    }

    pub fn next(&mut self) {
        if let Some(items) = &self.list {
            self.c_idx = match self.c_idx {
                idx if idx == items.len() as u16 => 1,
                _ => self.c_idx + 1,
            }
        }
    }

    pub fn prev(&mut self) {
        if let Some(items) = &self.list {
            self.c_idx = match self.c_idx {
                1 => items.len() as u16,
                _ => self.c_idx - 1,
            }
        }
    }
}
