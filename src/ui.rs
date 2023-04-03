#[derive(Clone)]
struct Position {
    x: u16,
    y: u16,
}

impl Position {
    pub fn add(&self, pos: Self) -> Self {
        let x = self.x + pos.x;
        let y = self.y + pos.y;
        Position { x, y }
    }
}

struct Size {
    height: u16,
    width: u16,
}

#[allow(dead_code)]
pub struct ListWidget<T, F>
where
    F: Fn(&T) -> String,
{
    list: Vec<T>,
    selected_index: usize,
    frame: Rect,
    scroll: Position,
    render: F,
}

impl<T, F> ListWidget<T, F>
where
    F: Fn(&T) -> String,
    T: std::fmt::Debug,
{
    pub fn new(list: Vec<T>, frame: Rect, render: F) -> ListWidget<T, F> {
        ListWidget {
            list,
            selected_index: 0,
            frame,
            scroll: Position { x: 0, y: 0 },
            render,
        }
    }

    pub fn update_list(&mut self, list: Vec<T>) {
        self.list = list
    }

    pub fn populate(&mut self) {
        let count = self.frame.size.height;
        self.list
            .iter()
            .clone()
            .enumerate()
            .for_each(|(index, item)| {
                if (index as u16) < count - 1 {
                    self.frame.add_line(&((self.render)(item)));
                }
            })
    }
}

pub struct Rect {
    position: Position,
    size: Size,
    cursor: Position,
}

// ╭ (U+256D) - Top left corner
// ╮ (U+256E) - Top right corner
// ╰ (U+2570) - Bottom left corner
// ╯ (U+256F) - Bottom right corner
// ─ (U+2500) - Horizontal line
// │ (U+2502) - Vertical line

enum BorderChars {
    TopLeftCorner,
    TopRightCorner,
    BottomLeftCorner,
    BottomRightCorner,
    HorizontalLine,
    VerticalLine,
}

impl ToString for BorderChars {
    fn to_string(&self) -> String {
        match self {
            BorderChars::TopLeftCorner => "┌".to_string(),
            BorderChars::TopRightCorner => "┐".to_string(),
            BorderChars::BottomLeftCorner => "└".to_string(),
            BorderChars::BottomRightCorner => "┘".to_string(),
            BorderChars::HorizontalLine => "─".to_string(),
            BorderChars::VerticalLine => "│".to_string(),
        }
    }
}

impl Rect {
    pub fn add_line(&mut self, text: &str) {
        print!("{}", termion::cursor::Goto(self.cursor.x, self.cursor.y));
        print!("{:}", text);
        self.cursor.y += 1;
    }

    pub fn new(x: u16, y: u16, w: u16, h: u16) -> Self {
        let position = Position { x, y };
        let cursor = Position {
            x: x.saturating_add_signed(1),
            y: y.saturating_add(1),
        };
        let size = Size {
            width: w,
            height: h,
        };
        Rect {
            position,
            size,
            cursor,
        }
    }
    pub fn draw(&self) {
        // horizontal

        let Size { width, height } = self.size;
        let end_pos = self.position.clone().add(Position {
            x: width,
            y: height,
        });

        for i in self.position.x..end_pos.x {
            print!("{}", termion::cursor::Goto(i, self.position.y));
            print!("{:}", BorderChars::HorizontalLine.to_string());
        }
        for i in self.position.x..end_pos.x {
            print!("{}", termion::cursor::Goto(i, end_pos.y));
            print!("{:}", BorderChars::HorizontalLine.to_string());
        }
        // vertical
        for i in self.position.y..end_pos.y {
            print!("{}", termion::cursor::Goto(self.position.x, i + 1));
            print!("{:}", BorderChars::VerticalLine.to_string());
        }
        for i in self.position.y..end_pos.y {
            print!("{}", termion::cursor::Goto(end_pos.x, i + 1));
            print!("{:}", BorderChars::VerticalLine.to_string());
        }
        print!(
            "{}",
            termion::cursor::Goto(self.position.x, self.position.y)
        );
        print!("{:}", BorderChars::TopLeftCorner.to_string());
        print!("{}", termion::cursor::Goto(end_pos.x, self.position.y));
        print!("{:}", BorderChars::TopRightCorner.to_string());
        print!("{}", termion::cursor::Goto(self.position.x, end_pos.y));
        print!("{:}", BorderChars::BottomLeftCorner.to_string());
        print!("{}", termion::cursor::Goto(end_pos.x, end_pos.y));
        print!("{:}", BorderChars::BottomRightCorner.to_string());
    }
}
