# Patra
A TUI file manager written in spaghetti code :)

While Patra works is a stand alone TUI application, one of the main goals is to use it embedded in other applications.

Currently it's developed together with [`patra.nvim`](https://github.com/Bhanukamax/patra.nvim) which is a neovim pluggin that wraps Patra to be a simple alternative for netrw.

## Caution!
This is still at very early development, not recommended for regular use 




## Key bindings

### Navigation

| key       | action                        |
| --------- | ----------------------------- |
| q         | quit                          |
| j         | down                          |
| k         | up                            |
| l, Enter  | open current item             |
| h, -      | go back one directory item    |

### Actions

| key       | action                        |
| --------- | ----------------------------- |
| d         | create directory              |
| %         | create file                   |
| D         | delete file or directory (Caution!, it will delete, non empty directories with all it's contents)     |
