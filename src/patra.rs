use std::fs::read_dir;
use std::io::{Stdout, Write};
use termion::screen::AlternateScreen;

#[derive(Clone, Debug, PartialEq)]
pub enum FileItemType {
    File,
    Dir,
    Sym,
    Unknown,
}

#[derive(Clone, Debug)]
pub struct FileItem {
    pub name: String,
    pub file_type: FileItemType,
}

// TODO: make the path vector to easily go back and forth
#[derive(std::clone::Clone)]
pub struct FileList {
    pub items: Option<Vec<FileItem>>,
    pub path: String,
    pub c_idx: u16,
}

impl FileList {
    pub fn list_dir(&mut self) -> std::io::Result<()> {
        // let dir_list = read_dir(&self.path).unwrap();
        let dir_list = match read_dir(&self.path) {
            Ok(dir) => dir,
            Err(e) => return Err(e),
        };

        self.items = Some(
            dir_list
                .into_iter()
                .map(|x| FileItem {
                    name: String::from(x.as_ref().unwrap().file_name().to_str().unwrap()),
                    file_type: if x.as_ref().unwrap().path().is_dir() {
                        FileItemType::Dir
                    } else if x.as_ref().unwrap().path().is_file() {
                        FileItemType::File
                    } else if x.unwrap().path().is_symlink() {
                        FileItemType::Sym
                    } else {
                        FileItemType::Unknown
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
        self.items
            .as_ref()
            .filter(|items| items[idx].file_type == FileItemType::Dir)
            .iter()
            .for_each(|_| {
                self.path = if &self.path == "/" {
                    String::from(&self.path) + &self.items.as_ref().unwrap()[idx].name
                } else {
                    String::from(&self.path) + "/" + &self.items.as_ref().unwrap()[idx].name
                };
            });
        if let Err(e) = self.list_dir() {
            write!(screen, "{}{} ", termion::cursor::Goto(10, 2), e).unwrap();
            self.path = original_path;
        }
        write!(screen, "{}{} ", termion::cursor::Goto(10, 4), "done").unwrap();
        Ok(())
    }
    pub fn up_dir(&mut self) {
        if &self.path != "/" {
            self.path = std::path::Path::new(&self.path)
                .parent()
                .unwrap()
                .to_str()
                .unwrap()
                .to_string();
            self.list_dir().unwrap();
        }
        let out = std::io::stderr();
        writeln!(&out, "updated dir {:?}", self.path).unwrap();
    }
    pub fn next(&mut self) {
        match self.items {
            Some(_) => {
                if self.c_idx == self.items.as_ref().unwrap().len() as u16 {
                    self.c_idx = 1
                } else {
                    self.c_idx = self.c_idx + 1
                }
            }
            None => (),
        }
    }
    pub fn prev(&mut self) {
        match self.items {
            Some(_) => {
                if self.c_idx == 1 {
                    self.c_idx = self.items.as_ref().unwrap().len() as u16
                } else {
                    self.c_idx = self.c_idx - 1
                }
            }
            None => (),
        }
    }
}
