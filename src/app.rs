use crate::logger;
use std::{
    fs::{self, File},
    io::Write,
};

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
}
#[derive(Default)]
pub struct Flags {
    write_to_file: bool,
}

pub enum CommandType {
    CreateFile,
    CreateDir,
    ConfirmDelete,
    GoToNormalMode,
    _Input,
}

pub enum UiMode {
    Normal,
    // TODO:
    // Get rid oth the second item of the command tuple (Option<String>) as it's not used
    // the app.command_str is used instead
    Command(CommandType, Option<String>),
}

pub struct App {
    pub should_quit: bool,
    pub state: PatraFileState,
    pub flags: Flags,
    pub selection_file_path: String,
    pub exit_code: i32,
    pub ui_mode: UiMode,
    pub command_str: Option<String>,
}

impl Default for App {
    fn default() -> Self {
        Self {
            selection_file_path: "".into(),
            should_quit: false,
            exit_code: 0,
            command_str: Some("".into()),
            ui_mode: UiMode::Normal,
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

    pub fn insert_command_char(&mut self, c: &char) {
        if let Some(s) = &self.command_str {
            self.command_str = Some(s.to_owned() + &c.to_string())
        }
    }

    pub fn try_delete_file(&mut self) {
        let idx: usize = self.state.c_idx as usize - 1;
        let current_file = self.state.list.get(idx).unwrap();
        if current_file.file_type == PatraFileItemType::Dir {
            let path = format!("{}/{}", &self.state.path, &current_file.name);
            logger::debug(&format!("Trying to delete dir: {}", &path));
            fs::remove_dir_all(format!("{}", path)).expect("unable to remove dir")
        } else {
            let path = format!("{}/{}", &self.state.path, &current_file.name);
            logger::debug(&format!("Trying to delete file: {}", &path));
            fs::remove_file(format!("{}", path)).expect("unable to remove file")
        }
        // self.list_dir().expect("unable to list dir")
        self.run_command(CommandType::GoToNormalMode);
        self.list_dir().unwrap();
    }

    pub fn try_create_file(&mut self) -> Result<(), std::io::Error> {
        if let Some(f_name) = &self.command_str {
            let file_name = format!("{}/{}", self.state.path, f_name);
            File::create(file_name)?;
            self.run_command(CommandType::GoToNormalMode);
            self.list_dir()?;
        }
        Ok(())
    }

    pub fn try_create_dir(&mut self) -> Result<(), std::io::Error> {
        if let Some(f_name) = &self.command_str {
            let file_name = format!("{}/{}", self.state.path, f_name);
            fs::create_dir(file_name)?;
            self.run_command(CommandType::GoToNormalMode);
            self.list_dir()?;
        }
        Ok(())
    }

    pub fn delete_command_char(&mut self) {
        match &self.command_str {
            Some(cmd) => {
                let mut new_cmd = cmd.to_owned();
                new_cmd.pop();
                self.command_str = Some(format!("{}", new_cmd))
            }
            _ => {}
        }
    }

    pub fn run_command(&mut self, cmd: CommandType) {
        match cmd {
            CommandType::GoToNormalMode => {
                self.ui_mode = UiMode::Normal;
                self.command_str = Some("".into());
            }
            CommandType::ConfirmDelete => {
                let idx: usize = self.state.c_idx as usize - 1;
                let current_file = self.state.list.get(idx).unwrap();
                let path = format!("{}/{}", &self.state.path, &current_file.name);

                let mut command_text: String = "Confirm deletion of ".into();

                if current_file.file_type == PatraFileItemType::Dir {
                    command_text.push_str("file?")
                } else {
                    command_text.push_str("directory and it's content? ")
                }
                self.ui_mode = UiMode::Command(cmd, Some(format!("{}: {}", command_text, path)));
            }
            _ => self.ui_mode = UiMode::Command(cmd, None),
        }
    }

    pub fn update_path(&mut self, path: String) {
        self.state.path = if path == "." {
            self.state.path.clone()
        } else if path.starts_with("./") {
            self.state.path.clone() + path.trim_start_matches('.')
        } else if path.starts_with('/') {
            path
        } else {
            self.state.path.clone()
        }
    }

    pub fn quit(&mut self, exit_code: Option<i32>) {
        self.should_quit = true;
        self.exit_code = exit_code.unwrap_or(0);
    }

    pub fn list_dir(&mut self) -> std::io::Result<()> {
        let dir_list = fs::read_dir(&self.state.path)?;
        self.state.list = dir_list
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
        self.state.list.sort_by(|a, b| a.name.cmp(&b.name));
        self.state.c_idx = 1;

        Ok(())
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
            self.quit(Some(2))
        }

        logger::debug(&format!("New path: {:?}", new_path));
        self.state.path = new_path;

        self.list_dir()
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

    pub fn up_dir(&mut self) -> Result<(), std::io::Error> {
        if &self.state.path != "/" {
            self.state.path = std::path::Path::new(&self.state.path)
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
        self.state.c_idx = if self.state.c_idx == self.state.list.len() as u16 {
            1
        } else {
            self.state.c_idx + 1
        };
    }

    pub fn prev(&mut self) {
        self.state.c_idx = if self.state.c_idx == 1 {
            self.state.list.len() as u16
        } else {
            self.state.c_idx - 1
        }
    }
}
